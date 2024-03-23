use std::error::Error;

mod parse;
mod render;
mod tokenize;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let source = std::fs::read_to_string(&args[1])?;
    let markdown = convert(&source);

    println!("markdown = {:?}\n", markdown);
    Ok(())
}

fn convert(source: &str) -> Result<String, Box<dyn Error>> {
    let tokens = tokenize::Tokenizer::new(&source).tokenize()?;
    let html = parse::parse(&tokens)?;
    let markdown = render::render(&html)?;
    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        {
            let source = "<!DOCTYPE html><html><head></head><body>Hello!</body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "Hello!"),
                Err(e) => assert!(false, "Expected Ok(\"Hello!\") but got Err({:?})", e),
            }
        }
    }
}
