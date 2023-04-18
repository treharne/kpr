mod errors;
use errors::KprError;

mod cli;
use cli::{get_cmd, Commands, ListArgs, SearchArgs};

mod helpers;
use helpers::{to_line, words_from_stdin};

mod locks;
mod search;
mod store;
use store::STORE_FILENAME;

fn keep(message_parts: Vec<String>) -> Result<(), KprError> {

    let message_parts = if message_parts.len() == 0 {
        println!("Your message: ");
        words_from_stdin()?
    } else {
        message_parts
    };

    let message = message_parts.join(" ");
    let line = to_line(&message);
    let line_number = store::write(&line)?;
    search::index::add_line(line_number, &line);
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
    let query = if args.query.len() == 0 {
        println!("Search for: ");
        words_from_stdin()?
    } else {
        args.query
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
            println!("requested {:?} results", args.n);
            list(args)?;
        },
        Commands::Search(args) => {
            println!("requested {:?} results", args.n);
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
