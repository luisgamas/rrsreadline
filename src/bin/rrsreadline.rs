use rrsreadline::config::Config;
use rrsreadline::history;
use rrsreadline::matching::Matcher;
use rrsreadline::shell::zsh;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--version") | Some("-V") => {
            println!("rrsreadline {}", env!("CARGO_PKG_VERSION"));
        }
        Some("--help") | Some("-h") => print_help(),
        Some("init") if args.next().as_deref() == Some("zsh") => {
            print!("{}", zsh::generate());
        }
        Some("suggest") => {
            let query = args.collect::<Vec<_>>().join(" ");
            if let Err(error) = suggest(&query) {
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

fn suggest(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    if query.is_empty() {
        return Ok(());
    }
    let config = Config::load();
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
    println!("usage: rrsreadline <--version | --help | init zsh | suggest <query>>");
}
