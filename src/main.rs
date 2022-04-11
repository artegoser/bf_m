use clap::Parser;
use std::error::Error;
use std::fs;
use std::time::Instant;

mod interpreter;
use interpreter::Interpreter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: String,

    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let args = Args::parse();
    if args.verbose == true {
        println!("Running: {}\n", args.file);
    }
    let file_input = fs::read_to_string(args.file)?.parse()?;

    let mut bf_int = Interpreter::new(file_input);

    bf_int.run()?;

    if args.verbose == true {
        println!("\nElapsed: {:?}", start.elapsed());
    }

    Ok(())
}
