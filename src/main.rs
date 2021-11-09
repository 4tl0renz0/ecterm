use std::io::*;
use std::process;
use std::{thread, time::Duration};
use std::path::Path;
use std::fs;

// for directory creation
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()>

fn mkdir(dir) -> std::io::Result<()> {
    fs::create_dir(dir)?;
    Ok(())
}

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

    println!("\rECTerm v{0}.{1}.{2}\nCopyright (c) Redpendrew 2021\nSee Redpendrew @ \"https://github.com/Redpendrew\"\nSee the ECTerm repository @ \"https://github.com/Redpendrew/ECTerm\"\nInput \"help\" for commands.", vmajor, vminor, vfix);
    loop {
        print!("@{}>", dir);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut term).ok().expect("[ERROR: Failed to read line]");
        term.remove(term.len() - 1);

        if term == "help" {
            println!("{{sample help menu}}");
        } else if term.contains("cd") {
            let term_parts: Vec<_> = term.split_whitespace().collect();
            let reqdir = vec![dir.to_string(),term_parts[1].to_string()].join("/");
            let dir_exists: bool = Path::new(&reqdir).is_dir();
            if dir_exists == true {
                dir.push_str(term_parts[1]);
            } else {
                println!("[ERROR: Directory path: {}: is not a valid directory]", reqdir);
            };
        } else if term.contains("mkdir") {
            let term_parts: Vec<_> = term.split_whitespace().collect();
            let reqdir = vec![dir.to_string(),term_parts[1].to_string()].join("/");
            mkdir(reqdir)
                .ok("{0} created @ {1} successfully.", term_parts[1], dir)
                .except("[ERROR: fn: \"mkdir()\": refused to create directory]");
        } else if term.contains("test") {
            let term_parts: Vec<_> = term.split_whitespace().collect();
            println!("{}", term_parts[1]);
        } else if term == "exit" {
            println!("Thank you for using ECTerm.");
            process::exit(0x0100);
        } else {
            println!("Invalid command or term.");
        }

        term = String::new();
    }
}
