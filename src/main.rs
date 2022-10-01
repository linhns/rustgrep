use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::PathBuf;

fn grep<R>(target: &str, reader: R) -> io::Result<()> 
    where R: BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(arg) => arg,
        None => Err("Usage: rustgrep PATTERN FILE(s)")?
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();
    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }
    Ok(())
}

fn main() {
    let result = run();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
