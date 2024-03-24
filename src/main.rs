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

        // heading
        {
            let source = "<!DOCTYPE html><html><head></head><body><h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6></body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(
                    result,
                    "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6\n"
                ),
                Err(e) => assert!(false, "Unexpected Err({:?})", e),
            }
        }

        // paragraph
        {
            let source =
                "<!DOCTYPE html><html><head></head><body><p>para1</p><p>para2</p></body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "para1\n\npara2\n\n"),
                Err(e) => assert!(false, "Unexpected Err({:?})", e),
            }
        }

        // ruby
        {
            let source =
                "<!DOCTYPE html><html><head></head><body><ruby>hello<rt>world</rt></ruby></body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "hello"),
                Err(e) => assert!(false, "Unexpected Err({:?})", e),
            }
        }
        {
            let source =
                "<!DOCTYPE html><html><head></head><body><ruby>hello<rp>(</rp><rt>world</rt><rp>)</rp></ruby></body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "hello"),
                Err(e) => assert!(false, "Unexpected Err({:?})", e),
            }
        }
    }
}
