use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn suggest_command_returns_each_history_command_once() {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before Unix epoch")
        .as_nanos();
    let home = std::env::temp_dir().join(format!(
        "rrsreadline-dedup-test-{}-{suffix}",
        std::process::id()
    ));
    std::fs::create_dir_all(&home).expect("create fake HOME");
    std::fs::write(
        home.join(".zsh_history"),
        "git status\ngit status\ngit log\ngit status\ngit log\n",
    )
    .expect("write fake history");

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_rrsreadline"))
        .env("HOME", &home)
        .args(["suggest", "--shell", "zsh", "git"])
        .output()
        .expect("run suggest command");
    let _ = std::fs::remove_dir_all(&home);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "git log\ngit status\n"
    );
}
