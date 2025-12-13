use std::fs;
use std::env;
use axity::run_source;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("usage: axity <file.ax>"); std::process::exit(1); }
    let src = match fs::read_to_string(&args[1]) { Ok(s) => s, Err(e) => { eprintln!("read error: {}", e); std::process::exit(1) } };
    match run_source(&src) {
        Ok(out) => print!("{}", out),
        Err(e) => { eprintln!("{}", e); std::process::exit(1); }
    }
}

