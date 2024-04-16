use std::error::Error;

mod ast;
mod parse;
mod render;
mod restruct;
mod tokenize;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let source = std::fs::read_to_string(&args[1])?;
    let markdown = convert(&source)?;

    println!("{}", markdown);
    Ok(())
}

fn convert(source: &str) -> Result<String, Box<dyn Error>> {
    let tokens = tokenize::Tokenizer::new(source).tokenize()?;
    let original_node = parse::Parser::new(&tokens).parse()?;
    let node = restruct::restruct(&original_node);
    let markdown = render::Renderer::new(&node).render()?;
    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_only_body() {
        let source = "<body>hello</body>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "hello\n"),
            Err(e) => assert!(false, "Expected Ok(\"Hello!\") but got Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_text() {
        let source = "<!DOCTYPE html><html><head></head><body>Hello!</body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "Hello!\n"),
            Err(e) => assert!(false, "Expected Ok(\"Hello!\") but got Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_blockquote() {
        let source = "<!DOCTYPE html><html><head></head><body><blockquote>hello<br/>world</blockquote></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "> hello\n> world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_blockquote_with_p() {
        let source = "<!DOCTYPE html><html><head></head><body><blockquote><p>hello</p><p>world</p></blockquote></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "> hello\n> \n> world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_br() {
        let source = "<!DOCTYPE html><html><head></head><body>hello<br/>world</body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "hello\nworld\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_code() {
        let source =
            "<!DOCTYPE html><html><head></head><body>This is <code>hello</code>.</body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "This is `hello`.\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_del() {
        let source =
            "<!DOCTYPE html><html><head></head><body>This is <del>hello</del>.</body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "This is ~hello~.\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_div() {
        let source =
            "<!DOCTYPE html><html><head></head><body><div><p>hello</p><p>world</p></div></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "hello\n\nworld\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_em() {
        let source =
            "<!DOCTYPE html><html><head></head><body>This is <em>hello</em>.</body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "This is _hello_.\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_heading() {
        let source = "<!DOCTYPE html><html><head></head><body><h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(
                result,
                "# H1\n\n## H2\n\n### H3\n\n#### H4\n\n##### H5\n\n###### H6\n"
            ),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_hr() {
        let source =
            "<!DOCTYPE html><html><head></head><body><p>para1</p><hr/><p>para2</p></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "para1\n\n---\n\npara2\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_paragraph() {
        let source =
            "<!DOCTYPE html><html><head></head><body><p>para1</p><p>para2</p></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "para1\n\npara2\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ruby() {
        let source =
                "<!DOCTYPE html><html><head></head><body><ruby>hello<rt>world</rt></ruby></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "hello\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ruby_with_rp_and_rt() {
        let source =
                "<!DOCTYPE html><html><head></head><body><ruby>hello<rp>(</rp><rt>world</rt><rp>)</rp></ruby></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "hello\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_strong() {
        let source =
                "<!DOCTYPE html><html><head></head><body>This is <strong>strong</strong>.</body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "This is **strong**.\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_complete_table() {
        let source =
                "<!DOCTYPE html><html><head></head><body><table><tr><th>1,1</th><th>1,2</th></tr><tr><td>2,1</td><td>2,2</td></tr><tr><td>3,1</td><td>3,2</td></tr></table></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(
                result,
                "| 1,1 | 1,2 |\n|---|---|\n| 2,1 | 2,2 |\n| 3,1 | 3,2 |\n"
            ),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_standard_table() {
        let source =
                "<!DOCTYPE html><html><head></head><body><table><thead><tr><th>1,1</th><th>1,2</th></tr></thead><tbody><tr><td>2,1</td><td>2,2</td></tr><tr><td>3,1</td><td>3,2</td></tr></tbody></table></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(
                result,
                "| 1,1 | 1,2 |\n|---|---|\n| 2,1 | 2,2 |\n| 3,1 | 3,2 |\n"
            ),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_newline_joints() {
        let source = "<html><head></head><body><p>hello</p><p>world</p></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "hello\n\nworld\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_table_including_p() {
        let source = "<html><head></head><body><table><tr><th>hello</th></tr><tr><td><p>world</p></td></tr></table></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "| hello |\n|---|\n| world |\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_table_including_br() {
        let source =
                "<!DOCTYPE html><html><head></head><body><table><thead><tr><th>1,1</th><th>1,2</th></tr></thead><tbody><tr><td>2<br>,<br>1</td><td>2<br>,<br>2</td></tr><tr><td>3<br>,<br>1</td><td>3,2</td></tr></tbody></table></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(
                result,
                "| 1,1 | 1,2 |\n|---|---|\n| 2<br>,<br>1 | 2<br>,<br>2 |\n| 3<br>,<br>1 | 3,2 |\n"
            ),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ul() {
        let source = "<html><head></head><body><ul><li>hello</li><li>world</li></ul></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "- hello\n- world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ol() {
        let source = "<html><head></head><body><ol><li>hello</li><li>world</li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "1. hello\n1. world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ul_with_br() {
        let source = "<html><head></head><body><ul><li>hello<br>world</li></ul></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "- hello\n  world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ol_with_br() {
        let source = "<html><head></head><body><ol><li>hello<br>world</li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "1. hello\n   world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ul_with_p() {
        let source =
            "<html><head></head><body><ul><li><p>hello</p><p>world</p></li></ul></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "- hello\n  \n  world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ol_with_p() {
        let source =
            "<html><head></head><body><ol><li><p>hello</p><p>world</p></li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "1. hello\n   \n   world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ul_and_ul() {
        let source = "<html><head></head><body><ul><li><ul><li>hello</li><li>world</li></ul></li></ul></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "- - hello\n  - world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ol_and_ol() {
        let source = "<html><head></head><body><ol><li><ol><li>hello</li><li>world</li></ol></li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "1. 1. hello\n   1. world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ol_in_google_doc_tyle() {
        let source = "<html><head></head><body><ol class=\"foo-0\"><li>hello</li><li>world</li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "1. hello\n1. world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_indented_ol_in_google_doc_tyle() {
        let source = "<html><head></head><body><ol class=\"foo-1\"><li>hello</li><li>world</li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "    1. hello\n    1. world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_ol_and_indented_ol_in_google_doc_tyle() {
        let source = "<html><head></head><body><ol class=\"foo-0\"><li>hello</li><li>world</li></ol><ol class=\"foo-1\"><li>hello</li><li>world</li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "1. hello\n1. world\n    1. hello\n    1. world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_p_and_ol_in_google_doc_tyle() {
        let source = "<html><head></head><body><p>foobar</p><ol class=\"foo-0\"><li>hello</li><li>world</li></ol></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "foobar\n\n1. hello\n1. world\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_a_without_attributes() {
        let source = "<html><head></head><body><a>hello</a></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "hello\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_a_with_href() {
        let source =
            "<html><head></head><body><a href=\"https://example.com\">hello</a></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "[hello](https://example.com)\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_a_with_name() {
        let source = "<html><head></head><body><a name=\"world\">hello</a></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(result, "<a name=\"world\">hello</a>\n"),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_a_with_href_and_name() {
        let source = "<html><head></head><body><a href=\"https://example.com\" name=\"world\">hello</a></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(
                result,
                "<a href=\"https://example.com\" name=\"world\">hello</a>\n"
            ),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_img() {
        let source = "<html><head></head><body><img src=\"https://example.com/example.png\" width=\"400\" height=\"300\"></body></html>";
        match convert(source) {
            Ok(result) => assert_eq!(
                result,
                "<img height=\"300\" src=\"https://example.com/example.png\" width=\"400\">\n"
            ),
            Err(e) => assert!(false, "Unexpected Err({:?})", e),
        }
    }

    #[test]
    fn test_convert_entity() {
        {
            let source = "<body>&#x3042;&#x3044;&#x3046;&#x3048;&#x304A; Foo &#x304B;&#x304D;&#x304F;&#x3051;&#x3053; Bar</body>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "あいうえお Foo かきくけこ Bar\n"),
                Err(e) => assert!(
                    false,
                    "Expected Ok(\"あいうえお Foo かきくけこ Bar\") but got Err({:?})",
                    e
                ),
            }
        }
        {
            let source = "<html><head></head><body>&nbsp;</body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "&nbsp;\n"),
                Err(e) => assert!(false, "Unexpected Err({:?})", e),
            }
        }
        {
            let source = "<html><head></head><body>&#1234;</body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "Ӓ\n"),
                Err(e) => assert!(false, "Unexpected Err({:?})", e),
            }
        }
        {
            let source = "<html><head></head><body>&#xd06;</body></html>";
            match convert(source) {
                Ok(result) => assert_eq!(result, "ആ\n"),
                Err(e) => assert!(false, "Unexpected Err({:?})", e),
            }
        }
    }
}
