mod errors;
use errors::KprError;

mod cli;
use cli::{get_cmd, Commands, ListArgs, SearchArgs};

mod helpers;
use helpers::{words_from_stdin, format_records_to_table, format_records};
use formatters::get_date_fmt_fn;

mod locks;
mod search;
mod store;
mod ago;
mod tables;
mod records;
mod formatters;
use search::format_matches;
use store::STORE_FILENAME;
use tables::make_table;
use records::Record;


fn keep(message_parts: Vec<String>) -> Result<(), KprError> {

    let message_parts = if message_parts.is_empty() {
        println!("Your message: ");
        words_from_stdin()?
    } else {
        message_parts
    };

    let message = message_parts.join(" ");
    let record = Record::create(message);
    let line_number = store::write(&record)?;

    let mut index = search::Index::load();
    index.add_line(line_number, &record);
    index.save();
    Ok(())
}


fn list(args: &ListArgs) {
    let records = store::load_records(Some(args.n));
    let fmt_fn = get_date_fmt_fn(args.date_format);
    let formatted_records = format_records_to_table(&records, fmt_fn);

    // let lines = store::load_lines(Some(args.n));

    // let fmt_fn = get_date_fmt_fn(args.date_format);
    // let formatted_lines = format_records_to_table(lines, fmt_fn);
    
    for record in formatted_records {
        println!("{record}");
    }
}

fn search(args: SearchArgs) -> Result<(), KprError> {
    let query = if args.query.is_empty() {
        println!("Search for: ");
        words_from_stdin()?
    } else {
        args.query
    };

    let records = search::search(&query, args.n);
    let fmt_fn = get_date_fmt_fn(args.date_format);
    let formatted_results = format_records(&records, fmt_fn);
    let formatted_results = format_matches(&query, formatted_results);
    let table = make_table(&formatted_results);
    for line in table {
        println!("{line}");
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
