mod errors;
use errors::KprError;

mod cli;
use cli::{get_cmd, Commands, ListArgs, SearchArgs};

mod helpers;
use helpers::to_line;

mod locks;
mod search;
mod store;
use store::STORE_FILENAME;

fn words_from_stdin() -> Result<Vec<String>, std::io::Error> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let message = buffer.trim();
    let message = message
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    Ok(message)
}


fn keep(message_parts: Vec<String>) -> Result<(), KprError> {
    let message = match message_parts.len() {
        0 => {
                println!("Your message: ");
                words_from_stdin()?
            },
        _ => message_parts,
    };

    let line = to_line(message.join(" "));
    let line_number = store::write(&line)?;
    search::index::add_line(line_number, line);
    Ok(())
}


fn list(args: ListArgs) -> Result<(), KprError> {
    let lines = store::load_lines(Some(args.n));

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

fn search(args: SearchArgs) -> Result<(), KprError> {
    let query = match args.query.len() {
        0 => {
                println!("Search for: ");
                words_from_stdin()?
            },
        _ => args.query,
    };

    let results = search::search(query, args.n);
    for result in results {
        println!("{}", result);
    }
    Ok(())
}

fn reindex() -> Result<(), KprError> {
    let index = search::index::build(STORE_FILENAME);
    search::index::save(&index);
    Ok(())
}

fn dispatch_cmd(cmd: Commands) -> Result<(), KprError> {
    match cmd {
        Commands::Keep { message } => {
            keep(message)?;
            println!("kpr kept your message.");
        },
        Commands::List(args) => {
            println!("requsted {:?} results", args.n);
            list(args)?;
        },
        // Commands::Search { query } => {
            Commands::Search(args) => {
            println!("requsted {:?} results", args.n);
            println!("search for {:?}", args.query);
            search(args)?;
        },
        Commands::Index => {
            reindex()?;
        },
    };
    Ok(())
}


fn main() {
    let cmd = get_cmd();
    dispatch_cmd(cmd).expect("everything is broken");

}
