use colour::*;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
    }

    let filename = &args[1];

    let content = fs::read_to_string(filename).expect("Error while opening the file");

    dark_green_ln!("\nCONTENT OF FILE ::");
    dark_green_ln!("{:-<60}\n", "");
    red_ln!("{}", content);
    dark_green_ln!("{:-<60}", "");
    dark_green_ln!(":: END OF FILE\n");
}

/// Display of usage message in case of wrong arguments
fn usage() {
    println!("Usage minigrep : minigrep <file>");
    println!("\t<file> : file to display");
    process::exit(1);
}
