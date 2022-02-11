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

    println!("Content :: \n{}", content);

    // for i in 1..args.len() {
    //     println!("arg {} : {}", i, &args[i]);
    // }
}

fn usage() {
    println!("Usage minigrep : minigrep <file>");
    println!("\t<file> : file to display");
    process::exit(1);
}
