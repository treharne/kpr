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

    let message_parts = if message_parts.is_empty() {
        println!("Your message: ");
        words_from_stdin()?
    } else {
        message_parts
    };

    let message = message_parts.join(" ");
    let line = to_line(&message);
    let line_number = store::write(&line)?;

    let mut index = search::Index::load();
    index.add_line(line_number, &line);
    index.save();
    Ok(())
}


fn list(args: &ListArgs) {
    let lines = store::load_lines(Some(args.n));

    for line in lines {
        println!("{line}");
    }

}

fn search(args: SearchArgs) -> Result<(), KprError> {
    let query = if args.query.is_empty() {
        println!("Search for: ");
        words_from_stdin()?
    } else {
        args.query
    };

    let results = search::search(&query, args.n);
    for result in results {
        println!("{result}");
    }
    Ok(())
}

fn reindex() {
    let index = search::Index::from_store_path(STORE_FILENAME);
    index.save();
}

fn dispatch(cmd: Commands) -> Result<(), KprError> {
    match cmd {
        Commands::Keep { message } => {
            keep(message)?;
            println!("kpr kept your message.");
        },
        Commands::List(args) => {
            list(&args);
        },
        Commands::Search(args) => {
            search(args)?;
        },
        Commands::Index => {
            reindex();
            println!("kpr indexed your messages from scratch.");
        },
    };
    Ok(())
}


fn main() {
    let cmd = get_cmd();
    dispatch(cmd).expect("everything is broken");

}
