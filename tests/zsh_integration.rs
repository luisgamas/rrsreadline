//! End-to-end checks for the Zsh/ZLE adapter.
//!
//! These tests intentionally use a real pseudo-terminal. ZLE only processes
//! key events and redraws its line editor while it owns a terminal, so a
//! normal child process or generated-script assertion cannot verify this
//! integration.

#![cfg(unix)]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
use nix::pty::{ForkptyResult, Winsize, forkpty};
use nix::sys::signal::{Signal, kill};
use nix::sys::wait::waitpid;
use nix::unistd::{Pid, execvp, read, write};
use std::ffi::CString;
use std::os::fd::{AsFd, OwnedFd};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct ZshSession {
    master: OwnedFd,
    child: Pid,
    fake_home: PathBuf,
}

impl ZshSession {
    fn spawn(history: &str) -> Self {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let fake_home = std::env::temp_dir().join(format!(
            "rrsreadline-zsh-pty-{}-{suffix}",
            std::process::id()
        ));
        std::fs::create_dir_all(&fake_home).expect("create fake HOME");
        std::fs::write(fake_home.join(".zsh_history"), history).expect("write zsh history");
        std::fs::write(fake_home.join("rrsreadline_tab_target"), "")
            .expect("write completion file");
        std::fs::create_dir(fake_home.join("Documents")).expect("create Documents directory");

        let winsize = Winsize {
            ws_row: 40,
            ws_col: 120,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        // SAFETY: the child branch only sets environment variables and calls
        // execvp before returning control to Rust code.
        match unsafe { forkpty(Some(&winsize), None) }.expect("forkpty") {
            ForkptyResult::Parent { child, master } => Self {
                master,
                child,
                fake_home,
            },
            ForkptyResult::Child => {
                // SAFETY: this is the single-threaded child immediately after
                // fork and the variables are used before exec.
                unsafe {
                    std::env::set_var("HOME", &fake_home);
                    std::env::set_var("TERM", "xterm-256color");
                }
                std::env::set_current_dir(&fake_home).expect("change to fake HOME");
                let zsh = CString::new("zsh").expect("no NUL");
                let no_rc = CString::new("-f").expect("no NUL");
                let _ = execvp(&zsh, &[zsh.clone(), no_rc]);
                std::process::exit(127);
            }
        }
    }

    fn send(&self, bytes: &[u8]) {
        write(self.master.as_fd(), bytes).expect("write to pty master");
    }

    fn send_and_drain(&self, bytes: &[u8]) -> Vec<u8> {
        self.send(bytes);
        std::thread::sleep(Duration::from_millis(120));
        self.drain(Duration::from_millis(250))
    }

    fn drain(&self, quiet_for: Duration) -> Vec<u8> {
        let mut output = Vec::new();
        let deadline = Instant::now() + quiet_for;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }
            let mut fds = [PollFd::new(self.master.as_fd(), PollFlags::POLLIN)];
            let timeout = PollTimeout::try_from(remaining).unwrap_or(PollTimeout::MAX);
            if poll(&mut fds, timeout).unwrap_or(0) == 0 {
                break;
            }
            let mut buffer = [0u8; 16_384];
            match read(self.master.as_fd(), &mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(size) => output.extend_from_slice(&buffer[..size]),
            }
        }
        output
    }
}

impl Drop for ZshSession {
    fn drop(&mut self) {
        let _ = kill(self.child, Signal::SIGKILL);
        let _ = waitpid(self.child, None);
        let _ = std::fs::remove_dir_all(&self.fake_home);
    }
}

#[test]
fn zsh_can_render_navigate_and_accept_a_suggestion() {
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let history = "git status\ngit branch\ngit log\n";
    let session = ZshSession::spawn(history);

    let setup = format!("eval \"$({} init zsh)\"\n", shell_single_quote(binary));
    session.send_and_drain(setup.as_bytes());

    let typed = session.send_and_drain(b"git");
    let typed_text = String::from_utf8_lossy(&typed);
    assert!(
        typed_text.contains("git log")
            && typed_text.contains("git branch")
            && typed_text.contains("git status"),
        "expected suggestions after typing git, got:\n{typed_text}"
    );

    let selected = session.send_and_drain(b"\x1b[B");
    let selected_text = String::from_utf8_lossy(&selected);
    assert!(
        selected_text.contains("❯ git log"),
        "expected Down to select the newest suggestion, got:\n{selected_text}"
    );

    let accepted = session.send_and_drain(b"\t");
    let accepted_text = String::from_utf8_lossy(&accepted);
    assert!(
        accepted_text.contains("git log"),
        "expected Tab to accept the selected suggestion, got:\n{accepted_text}"
    );

    session.send(b"\x03");
}

#[test]
fn zsh_escape_hides_predictions_for_native_history() {
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let history = "git status\necho RRS_ZSH_OLDER\ngit log\n";
    let session = ZshSession::spawn(history);
    let setup = format!(
        "fc -R ~/.zsh_history\neval \"$({} init zsh)\"\n",
        shell_single_quote(binary)
    );
    session.send_and_drain(setup.as_bytes());

    session.send_and_drain(b"git");
    session.send_and_drain(b"\x1b");
    let shown = session.send_and_drain(b"\x1b[12~");
    let shown_text = String::from_utf8_lossy(&shown);
    assert!(
        shown_text.contains("git log"),
        "expected F2 to show predictions again, got:\n{shown_text}"
    );
    let hidden = session.send_and_drain(b"\x1b[12~");
    let hidden_text = String::from_utf8_lossy(&hidden);
    assert!(
        !hidden_text.contains("git log"),
        "expected F2 to hide predictions, got:\n{hidden_text}"
    );
    let mut history_output = Vec::new();
    for _ in 0..4 {
        history_output.extend(session.send_and_drain(b"\x1b[A"));
    }
    let history_text = String::from_utf8_lossy(&history_output);
    assert!(
        history_text.contains("git log") && !history_text.contains("❯"),
        "expected Escape to hide predictions and Up to use native history, got:\n{history_text}"
    );

    session.send(b"\x03");
}

#[test]
fn zsh_preserves_native_tab_completion() {
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let session = ZshSession::spawn("git status\n");
    let setup = format!("eval \"$({} init zsh)\"\n", shell_single_quote(binary));
    session.send_and_drain(setup.as_bytes());

    session.send_and_drain(b"echo ~/rrsreadline_tab_");
    let completed = session.send_and_drain(b"\t");
    let completed_text = String::from_utf8_lossy(&completed);
    assert!(
        completed_text.contains("target"),
        "expected native Zsh path completion after Tab, got:\n{completed_text}"
    );

    session.send(b"\x03");
}

#[test]
fn zsh_preserves_completion_matcher_styles() {
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let session = ZshSession::spawn("git status\n");
    let setup = format!(
        "autoload -Uz compinit && compinit\nzstyle ':completion:*' matcher-list 'm:{{a-zA-Z}}={{A-Za-z}}'\neval \"$({} init zsh)\"\n_rrsreadline_test_dump_buffer() {{ print -r -- \"\\nRRS_BUFFER=<$BUFFER>\\n\"; }}\nzle -N _rrsreadline_test_dump_buffer\nbindkey '^X^V' _rrsreadline_test_dump_buffer\n",
        shell_single_quote(binary)
    );
    session.send_and_drain(setup.as_bytes());

    session.send_and_drain(b"cd doc");
    session.send_and_drain(b"\t");
    let buffer_dump = session.send_and_drain(b"\x18\x16");
    let buffer_dump_text = String::from_utf8_lossy(&buffer_dump);
    assert!(
        buffer_dump_text.contains("Documents"),
        "expected the configured case-insensitive matcher to complete Documents, got:\n{buffer_dump_text}"
    );

    session.send(b"\x03");
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
