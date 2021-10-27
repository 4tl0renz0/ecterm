use std::io::*;
use std::process;

/*
   ____________________________
  |____________________________|
   |                          |
   |   use std::fs;           |
   |   use std::path::Path;   |
   |__________________________|

*/

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

    println!("ECTerm v{0}.{1}.{2}\nCopyright (c) Redpendrew 2021\nSee Redpendrew @ \"https://github.com/Redpendrew\"\nSee the ECTerm repository @ \"https://github.com/Redpendrew/ECTerm\"\nInput \"help\" for commands.", vmajor, vminor, vfix);
    loop {
        print!("{}>", dir);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut term).ok().expect("[ERROR: Failed to read line]");
        term.remove(term.len() - 1);

        if term.contains("cd") {
            let term_parts: Vec<_> = term.split_whitespace().collect();
            dir.push_str(term_parts[1]);
        };

        if term.contains("mkdir") {
            let term_parts: Vec<_> = term.split_whitespace().collect();
            
        };

        if term.contains("twm") {
            let term_parts: Vec<_> = term.split_whitespace().collect();
            println!("{0}", term_parts[1]);
        };

        if term == "exit" {
            println!("Thank you for using ECTerm.");
            process::exit(0x0100);
        };

        term = String::new();
    }
}