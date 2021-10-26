use std::io::*;
// use std::io::prelude::*;
use std::process;

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
    println!("ECTerm v{0}.{1}.{2}
Copyright (c) Redpendrew 2021
See Redpendrew @ \"https://github.com/Redpendrew\"
See the ECTerm repository @ \"https://github.com/Redpendrew/ECTerm\"
Input \"help\" for commands.", vmajor, vminor, vfix);
    loop {
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut term).ok().expect("[ERROR: Failed to read line]");
       /*
        *if term == "exit" {
        *    print!("Thank you for using ECTerm.");
        *    process::exit(0x0100);
        *};
        */
    }
}
