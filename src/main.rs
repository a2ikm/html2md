use std::error::Error;

mod parse;
mod tokenize;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let source = std::fs::read_to_string(&args[1])?;
    let tokens = tokenize::Tokenizer::new(&source).tokenize()?;
    let html = parse::parse(&tokens)?;

    println!("html = {:?}\n", html);
    Ok(())
}
