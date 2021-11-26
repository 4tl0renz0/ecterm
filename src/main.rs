use std::io::{self, Write};
use std::process;
use std::path::Path;
use std::fs;

fn main() {
    println!("Initializing...");
    // major version number
    let vmajor: u8 = 0;
    // minor version number
    let vminor: u8 = 1;
    // bugfix version number
    let vfix: u8 = 0;
    // user input variable, "term"
    let mut term = String::new();
    // directory tracking variable
    let mut dir = String::new();

    println!("ECTerm v{0}.{1}.{2}\nCopyright (c) 2021 EthoCode [MIT license]\nInput \"help\" for commands.", vmajor, vminor, vfix);
    loop {
        print!("{}}}>", dir);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut term).ok().expect("[ERROR: Failed to read input]");
        term.remove(term.len() - 1);
        let term_parts: Vec<_> = term.split_whitespace().collect();

        if term_parts[0] == "help" {
            println!("{{sample help menu}}");
        } else if term_parts[0] == "cd" {
           // if term_parts[1] == ".." {
           //     println!("")
           // } else {
                let reqdir = vec![dir.to_string(),term_parts[1].to_string()].join("/");
                let dir_exists: bool = Path::new(&reqdir).is_dir();
                if dir_exists == true {
                    dir.push_str(term_parts[1]);
                } else {
                    println!("[ERROR: Directory path: {}: is not a valid directory]", reqdir);
                }
           // }
        } else if term_parts[0] == "cdir" {
            println!("Current directory is: {}", dir)
        } else if term_parts[0] == "mkdir" {
            let reqdir: String = vec![dir.to_string(),term_parts[1].to_string()].join("/");
            fs::create_dir(reqdir);
        } else if term_parts[0] == "echo" {
            let mut i: usize = 1;
            while i < term_parts.len() {
                print!("{}", term_parts[i]);
                io::stdout().flush().unwrap();
                if i != term_parts.len() {
                    print!(" ");
                    io::stdout().flush().unwrap();
                }
                i = i + 1;
            }
            print!("\n");
            io::stdout().flush().unwrap();
        } else if term == "exit" {
            println!("Thank you for using ECTerm.");
            process::exit(0x0100);
        } else {
            println!("Invalid command or term.");
        }

        println!();
        term = String::new();
    }
}
