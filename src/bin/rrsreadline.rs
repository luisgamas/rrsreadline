use rrsreadline::config::Config;
use rrsreadline::history;
use rrsreadline::matching::Matcher;
use rrsreadline::shell::bash;
use rrsreadline::shell::zsh;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--version") | Some("-V") => {
            println!("rrsreadline {}", env!("CARGO_PKG_VERSION"));
        }
        Some("--help") | Some("-h") => print_help(),
        Some("init") => match args.next().as_deref() {
            Some("zsh") => print!("{}", zsh::generate()),
            Some("bash") => print!("{}", bash::generate(&Config::load_for_shell("bash"))),
            _ => {
                print_help();
                std::process::exit(2);
            }
        },
        Some("suggest") => {
            let mut values = args.collect::<Vec<_>>();
            let shell = if values.first().map(String::as_str) == Some("--shell") {
                values.remove(0);
                if values.is_empty() {
                    "zsh".to_owned()
                } else {
                    values.remove(0)
                }
            } else {
                "zsh".to_owned()
            };
            let query = values.join(" ");
            if let Err(error) = suggest(&shell, &query) {
                eprintln!("rrsreadline: {error}");
                std::process::exit(1);
            }
        }
        _ => {
            print_help();
            std::process::exit(2);
        }
    }
}

fn suggest(shell: &str, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    if query.is_empty() {
        return Ok(());
    }
    let config = Config::load_for_shell(shell);
    let history = history::load_file(&config.history_path())?;
    let matcher = Matcher::new(
        config.matching,
        config.case_sensitive,
        config.max_suggestions,
    );
    for suggestion in matcher.suggest(history.entries(), query) {
        println!("{}", suggestion.text);
    }
    Ok(())
}

fn print_help() {
    println!(
        "usage: rrsreadline <--version | --help | init <zsh|bash> | suggest [--shell <shell>] <query>>"
    );
}
