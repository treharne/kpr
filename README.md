# `kpr`: Take notes without leaving your CLI
`kpr` (pronounced "keeper") is a utility which lets you take and retrieve notes without leaving your command line.

The main purpose is for me to learn Rust, but also so that I can take notes quickly and easily in the cli.

# Usage
`kpr keep`: Save a note  
`kpr list`: Retrieve recent notes  
`kpr search <search phrase>`: Search for notes containing the search phrase

# TODO
- all functions should take `&str` but return `String` (Rust best practice, I think)
- Make search better
    - notes for a specific date?
- "browse" command to allow scrolling through notes
- [Maybe] date based dirs for messages