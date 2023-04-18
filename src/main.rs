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

fn message_from_stdin() -> Result<String, std::io::Error> {
    println!("Enter your message: ");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let message = buffer.trim().to_string();
    Ok(message)
}


fn keep(message_parts: Vec<String>) -> Result<(), KprError> {
    let message = match message_parts.len() {
        0 => message_from_stdin()?,
        _ => message_parts.join(" "),
    };

    let line = to_line(message);
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
    let results = search::search(args.query, args.n);
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

    // let words = search::load_stopwords();
    // for word in words {
    //     println!("{}", word);
    // }

    // let store_filename = PathBuf::from(STORE_FILENAME);
    // let index = search::index::build(store_filename);
    // for (k, v) in index {
    //     let vals = v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    //     println!("{}: {}", k, vals);
    // }
}
