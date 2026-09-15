//! End-to-end check for the Bash/Readline adapter.

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
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct BashSession {
    master: OwnedFd,
    child: Pid,
    fake_home: PathBuf,
}

impl BashSession {
    fn spawn(history: &str) -> Self {
        Self::spawn_with_config(history, None)
    }

    fn spawn_with_config(history: &str, config: Option<&str>) -> Self {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let fake_home = std::env::temp_dir().join(format!(
            "rrsreadline-bash-pty-{}-{suffix}",
            std::process::id()
        ));
        std::fs::create_dir_all(&fake_home).expect("create fake HOME");
        std::fs::write(fake_home.join(".bash_history"), history).expect("write bash history");
        std::fs::write(fake_home.join("rrsreadline_tab_target"), "")
            .expect("write completion file");
        if let Some(config) = config {
            let config_dir = fake_home.join(".config/rrsreadline");
            std::fs::create_dir_all(&config_dir).expect("create config directory");
            std::fs::write(config_dir.join("config.toml"), config).expect("write config");
        }

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
                let bash = CString::new("bash").expect("no NUL");
                let no_rc = CString::new("--norc").expect("no NUL");
                let no_profile = CString::new("--noprofile").expect("no NUL");
                let interactive = CString::new("-i").expect("no NUL");
                let _ = execvp(&bash, &[bash.clone(), no_rc, no_profile, interactive]);
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

impl Drop for BashSession {
    fn drop(&mut self) {
        let _ = kill(self.child, Signal::SIGKILL);
        let _ = waitpid(self.child, None);
        let _ = std::fs::remove_dir_all(&self.fake_home);
    }
}

#[test]
fn bash_can_render_navigate_and_keep_a_suggestion_selected() {
    if !supports_writable_readline_line() {
        eprintln!("skipping Bash integration test: Bash 4+ is required");
        return;
    }
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let history = "echo RRS_BASH_STATUS\necho RRS_BASH_BRANCH\necho RRS_BASH_LOG\n";
    let session = BashSession::spawn(history);
    let setup = format!("eval \"$({} init bash)\"\n", shell_single_quote(binary));
    session.send_and_drain(setup.as_bytes());

    let typed = session.send_and_drain(b"echo");
    let typed_text = String::from_utf8_lossy(&typed);
    assert!(
        typed_text.contains("echo RRS_BASH_LOG")
            && typed_text.contains("echo RRS_BASH_BRANCH")
            && typed_text.contains("echo RRS_BASH_STATUS"),
        "expected suggestions after typing echo, got:\n{typed_text}"
    );

    let selected = session.send_and_drain(b"\x1b[B");
    let selected_text = String::from_utf8_lossy(&selected);
    assert!(
        selected_text.contains("❯ echo RRS_BASH_LOG")
            && selected_text.ends_with("echo RRS_BASH_LOG"),
        "expected Down to select and fill the newest suggestion, got:\n{selected_text}"
    );

    let accepted = session.send_and_drain(b"\r");
    let accepted_text = String::from_utf8_lossy(&accepted);
    assert!(
        accepted_text.contains("RRS_BASH_LOG"),
        "expected Enter to accept the selected suggestion, got:\n{accepted_text}"
    );

    session.send(b"\x03");
}

#[test]
fn bash_honors_configured_suggestion_limit() {
    if !supports_writable_readline_line() {
        eprintln!("skipping Bash integration test: Bash 4+ is required");
        return;
    }
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let history = "git one\ngit two\ngit three\n";
    let config = "max_suggestions = 2\n";
    let session = BashSession::spawn_with_config(history, Some(config));
    let setup = format!("eval \"$({} init bash)\"\n", shell_single_quote(binary));
    session.send_and_drain(setup.as_bytes());

    let typed = session.send_and_drain(b"git");
    let typed_text = String::from_utf8_lossy(&typed);
    assert!(typed_text.contains("git three") && typed_text.contains("git two"));
    assert!(
        !typed_text.contains("git one"),
        "expected the configured limit to omit the oldest suggestion, got:\n{typed_text}"
    );

    session.send(b"\x03");
}

#[test]
fn bash_escape_hides_predictions_for_native_history() {
    if !supports_writable_readline_line() {
        eprintln!("skipping Bash integration test: Bash 4+ is required");
        return;
    }
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let history = "git status\necho RRS_BASH_OLDER\ngit log\n";
    let session = BashSession::spawn(history);
    let setup = format!(
        "history -r ~/.bash_history\neval \"$({} init bash)\"\n",
        shell_single_quote(binary)
    );
    session.send_and_drain(setup.as_bytes());

    session.send_and_drain(b"git");
    session.send_and_drain(b"\x1b");
    let mut history_output = Vec::new();
    for _ in 0..3 {
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
fn bash_preserves_native_tab_completion() {
    if !supports_writable_readline_line() {
        eprintln!("skipping Bash integration test: Bash 4+ is required");
        return;
    }
    let binary = env!("CARGO_BIN_EXE_rrsreadline");
    let session = BashSession::spawn("git status\n");
    let setup = format!("eval \"$({} init bash)\"\n", shell_single_quote(binary));
    session.send_and_drain(setup.as_bytes());

    session.send_and_drain(b"echo ~/rrsreadline_tab_");
    let completed = session.send_and_drain(b"\t");
    let completed_text = String::from_utf8_lossy(&completed);
    assert!(
        completed_text.contains("target"),
        "expected native Bash path completion after Tab, got:\n{completed_text}"
    );

    session.send(b"\x03");
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn supports_writable_readline_line() -> bool {
    Command::new("bash")
        .args(["-c", "(( BASH_VERSINFO[0] >= 4 ))"])
        .status()
        .is_ok_and(|status| status.success())
}
