# `kpr`: Take notes without leaving your CLI
`kpr` (pronounced "keeper") is a utility which lets you take and retrieve notes without leaving your command line.

The main purpose is for me to learn Rust, but also so that I can take notes quickly and easily in the cli.

# Usage
`kpr keep`: Save a note  
`kpr list`: Retrieve recent notes  
`kpr search <search phrase>`: Search for notes containing the search phrase

# TODO
- Make search better
    - search by date
    - sort by date, sort by relevance
    - relevance score. exact strings should match better (order matters)
    - maybe allow some keywords (`AND` `OR` `NOT` or `&` `|` `-`. Are there others?)
- Colour matching words in search results
- "browse" command to allow scrolling through notes
- List and Search results should "chunk" into days (so date is not repeated but time is)
- [Maybe] date based dirs for messages

# Dev
## Build
```bash
cargo build --release
```