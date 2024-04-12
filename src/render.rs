use std::fmt;

use crate::ast::{is_block_element, is_void_element, Element, Node};

pub type Result<T> = std::result::Result<T, RenderError>;

#[derive(Debug, PartialEq)]
pub enum RenderError {
    OutsideOfList,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderError::OutsideOfList => write!(f, "outside of list"),
        }
    }
}

impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RenderError::OutsideOfList => None,
        }
    }
}

struct ContextItem<'a> {
    element: &'a Element,
}

impl<'a> ContextItem<'a> {
    fn new(element: &'a Element) -> Self {
        ContextItem { element }
    }
}

struct Context<'a> {
    items: Vec<ContextItem<'a>>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        let items = Vec::new();
        Context { items }
    }

    fn push(&mut self, element: &'a Element) {
        let item = ContextItem::new(element);
        self.items.push(item)
    }

    fn pop(&mut self) {
        self.items.pop();
    }

    fn get_last_list_tag(&mut self) -> Option<&str> {
        for item in self.items.iter().rev() {
            let tag_name = &item.element.tag;
            if tag_name == "ul" || tag_name == "ol" {
                return Some(tag_name);
            }
        }
        None
    }

    fn get_last_list_depth(&mut self) -> usize {
        for item in self.items.iter().rev() {
            let tag_name = &item.element.tag;
            if tag_name == "ul" || tag_name == "ol" {
                return item.element.list_depth();
            }
        }
        0
    }
}

pub struct Renderer<'a> {
    ctx: Context<'a>,
    root: &'a Node,
}

impl<'a> Renderer<'a> {
    pub fn new(root: &'a Node) -> Self {
        Self {
            ctx: Context::new(),
            root,
        }
    }

    pub fn render(&mut self) -> Result<String> {
        let mut result = self.render_node(self.root)?;
        if !result.ends_with('\n') {
            result.push('\n');
        }
        Ok(result)
    }

    fn render_node(&mut self, node: &'a Node) -> Result<String> {
        match node {
            Node::Element(element) => {
                self.ctx.push(element);
                let result = self.render_element(element);
                self.ctx.pop();
                result
            }
            Node::Text(content) => self.render_text(content),
        }
    }

    fn render_element(&mut self, element: &'a Element) -> Result<String> {
        match element.tag.as_str() {
            "a" => self.render_a_element(element),
            "abbr" => self.render_children(element),
            "address" => self.render_children(element),
            "article" => self.render_children(element),
            "aside" => self.render_children(element),
            "b" => self.render_children(element),
            "bdi" => self.render_children(element),
            "bdo" => self.render_children(element),
            "blockquote" => self.render_blockquote_element(element),
            "body" => self.render_container_element(element),
            "br" => self.render_br_element(element),
            "cite" => self.render_children(element),
            "code" => self.render_code_element(element),
            "data" => self.render_children(element),
            "dd" => self.render_children(element),
            "del" => self.render_del_element(element),
            "details" => self.render_children(element),
            "dfn" => self.render_children(element),
            "div" => self.render_container_element(element),
            "dl" => self.render_children(element),
            "dt" => self.render_dt_element(element),
            "em" => self.render_em_element(element),
            "h1" => self.render_h1_element(element),
            "h2" => self.render_h2_element(element),
            "h3" => self.render_h3_element(element),
            "h4" => self.render_h4_element(element),
            "h5" => self.render_h5_element(element),
            "h6" => self.render_h6_element(element),
            "hr" => self.render_hr_element(element),
            "html" => self.render_html_element(element),
            "i" => self.render_children(element),
            "img" => self.render_element_in_html_form(element),
            "ins" => self.render_children(element),
            "kbd" => self.render_children(element),
            "li" => self.render_li_element(element),
            "main" => self.render_children(element),
            "mark" => self.render_children(element),
            "menu" => self.render_children(element),
            "nav" => self.render_children(element),
            "ol" => self.render_stacked_children(element),
            "p" => self.render_p_element(element),
            "pre" => self.render_children(element),
            "q" => self.render_children(element),
            "rp" => self.render_nothing(element),
            "rt" => self.render_nothing(element),
            "ruby" => self.render_children(element),
            "s" => self.render_children(element),
            "samp" => self.render_children(element),
            "section" => self.render_children(element),
            "small" => self.render_children(element),
            "span" => self.render_children(element),
            "strong" => self.render_strong_element(element),
            "sub" => self.render_children(element),
            "summary" => self.render_children(element),
            "sup" => self.render_children(element),
            "time" => self.render_children(element),
            "u" => self.render_children(element),
            "ul" => self.render_stacked_children(element),
            "var" => self.render_children(element),
            "wbr" => self.render_children(element),

            // table
            "table" => self.render_table_element(element),
            "thead" => self.render_thead_element(element),
            "tbody" => self.render_tbody_element(element),
            "tr" => self.render_tr_element(element),
            "th" => self.render_th_element(element),
            "td" => self.render_td_element(element),
            "caption" | "colgroup" | "col" | "tfoot" => self.render_nothing(element),

            // successive lists
            "html2md:successive-lists-wrapper" => self.render_stacked_children(element),

            // render nothing
            "area" | "audio" | "button" | "canvas" | "datalist" | "dialog" | "embed"
            | "fieldset" | "figcaption" | "figure" | "footer" | "form" | "header" | "hgroup"
            | "iframe" | "input" | "label" | "legend" | "map" | "meter" | "noscript" | "object"
            | "optgroup" | "option" | "output" | "picture" | "progress" | "script" | "search"
            | "select" | "slot" | "source" | "template" | "textarea" | "track" | "video" => {
                self.render_nothing(element)
            }

            // unsupported
            _ => self.render_unsupported_element(element),
        }
    }

    fn render_children(&mut self, element: &'a Element) -> Result<String> {
        let mut result = String::new();

        for child in &element.children {
            let content = self.render_node(child)?;
            result.push_str(&content);
        }

        Ok(result)
    }

    fn render_stacked_children(&mut self, element: &'a Element) -> Result<String> {
        let mut parts = Vec::new();

        for node in &element.children {
            let content = self.render_node(node)?;
            parts.push(content);
        }

        Ok(parts.join("\n"))
    }

    fn render_container_element(&mut self, element: &'a Element) -> Result<String> {
        let mut parts = Vec::new();
        let mut part = String::new();

        for node in &element.children {
            let content = self.render_node(node)?;

            match node {
                Node::Element(child) => {
                    if is_block_element(&child.tag) && !part.is_empty() {
                        parts.push(part);
                        part = String::new();
                    }
                    part.push_str(&content);
                }
                Node::Text(_) => {
                    part.push_str(&content);
                }
            }
        }
        parts.push(part);

        let result = parts.join("\n\n");
        Ok(result)
    }

    fn render_nothing(&mut self, _: &Element) -> Result<String> {
        Ok(String::new())
    }

    fn render_unsupported_element(&mut self, element: &'a Element) -> Result<String> {
        eprintln!(
            "`{}` element is not supported. rendering nothing.",
            element.tag
        );
        self.render_nothing(element)
    }

    fn render_element_in_html_form(&mut self, element: &'a Element) -> Result<String> {
        let mut open_tag = String::new();
        open_tag.push('<');
        open_tag.push_str(&element.tag);
        if !element.attributes.is_empty() {
            let mut names: Vec<&String> = element.attributes.keys().collect();
            names.sort();

            for name in names {
                let value = element.attributes.get(name).unwrap();
                open_tag.push_str(&format!(" {}=\"{}\"", name, value));
            }
        }
        open_tag.push('>');

        if is_void_element(&element.tag) {
            return Ok(open_tag);
        }

        let close_tag = format!("</{}>", &element.tag);
        let content = self.render_children(element)?;

        Self::wrap(&content, &open_tag, &close_tag)
    }

    fn wrap(content: &str, prefix: &str, suffix: &str) -> Result<String> {
        let mut result = String::new();
        result.push_str(prefix);
        result.push_str(content);
        result.push_str(suffix);
        Ok(result)
    }

    fn render_a_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;

        if element.attributes.contains_key("name") {
            self.render_element_in_html_form(element)
        } else if let Some(href) = element.attributes.get("href") {
            Ok(format!("[{}]({})", content, href))
        } else {
            Ok(content)
        }
    }

    fn render_blockquote_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_container_element(element)?;

        let mut parts = Vec::new();
        for line in content.lines() {
            parts.push(format!("> {}", line));
        }
        Ok(parts.join("\n"))
    }

    fn render_br_element(&mut self, _: &Element) -> Result<String> {
        Ok(String::from("\n"))
    }

    fn render_code_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "`", "`")
    }

    fn render_del_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "~", "~")
    }

    fn render_dt_element(&mut self, element: &'a Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_em_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "_", "_")
    }

    fn render_h1_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "# ", "")
    }

    fn render_h2_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "## ", "")
    }

    fn render_h3_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "### ", "")
    }

    fn render_h4_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "#### ", "")
    }

    fn render_h5_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "##### ", "")
    }

    fn render_h6_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "###### ", "")
    }

    fn render_hr_element(&mut self, _: &Element) -> Result<String> {
        Ok(String::from("---"))
    }

    fn render_html_element(&mut self, element: &'a Element) -> Result<String> {
        if let Some(body_node) = element.children.iter().find(|node| match node {
            Node::Element(e) => e.tag == "body",
            _ => false,
        }) {
            self.render_node(body_node)
        } else {
            unreachable!()
        }
    }

    fn render_li_element(&mut self, element: &'a Element) -> Result<String> {
        let mut result = String::new();

        let marker = match self.ctx.get_last_list_tag() {
            Some("ul") => "-",
            Some("ol") => "1.",
            _ => return Err(RenderError::OutsideOfList),
        };

        let content = self.render_container_element(element)?;
        let marked_content = Self::prepend_list_marker(marker, &content);
        let indented_content = Self::indent(&marked_content, self.ctx.get_last_list_depth());
        result.push_str(&indented_content);

        Ok(result)
    }

    fn prepend_list_marker(marker: &str, content: &str) -> String {
        let mut parts = Vec::new();

        let sp = Self::spaces(marker.chars().count());
        for (i, line) in content.lines().enumerate() {
            let mut part = String::new();
            if i == 0 {
                part.push_str(marker);
            } else {
                part.push_str(&sp);
            }
            part.push(' ');
            part.push_str(line);
            parts.push(part);
        }

        parts.join("\n")
    }

    fn indent(content: &str, depth: usize) -> String {
        let mut parts = Vec::new();

        let sp = Self::spaces(depth * 4);
        for line in content.lines() {
            let mut part = String::new();
            part.push_str(&sp);
            part.push_str(line);
            parts.push(part);
        }

        parts.join("\n")
    }

    fn spaces(len: usize) -> String {
        let mut result = String::new();
        for _ in 0..len {
            result.push(' ');
        }
        result
    }

    fn render_p_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "", "")
    }

    fn render_strong_element(&mut self, element: &'a Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "**", "**")
    }

    fn render_table_element(&mut self, element: &'a Element) -> Result<String> {
        self.render_stacked_children(element)
    }

    fn render_thead_element(&mut self, element: &'a Element) -> Result<String> {
        let mut parts = Vec::new();

        let tr = self.render_children(element)?;
        parts.push(tr);

        let separator = self.render_table_separator_with_tr_node(&element.children[0])?;
        parts.push(separator);

        Ok(parts.join("\n"))
    }

    fn render_table_separator_with_tr_node(&mut self, node: &Node) -> Result<String> {
        let Node::Element(element) = node else {
            unreachable!()
        };
        if element.tag != "tr" {
            unreachable!()
        };

        let mut result = String::new();

        for _ in 0..element.children.len() {
            result.push_str("|---");
        }
        result.push('|');

        Ok(result)
    }

    fn render_tbody_element(&mut self, element: &'a Element) -> Result<String> {
        self.render_stacked_children(element)
    }

    fn render_tr_element(&mut self, element: &'a Element) -> Result<String> {
        let mut cells = Vec::new();
        for child in &element.children {
            let cell = self.render_node(child)?;
            cells.push(cell);
        }

        let numcols = cells.len();
        let numrows = cells
            .iter()
            .map(|cell| cell.lines().count())
            .max()
            .unwrap_or(0);

        let mut matrix = Vec::with_capacity(numrows);
        for _ in 0..numrows {
            matrix.push(vec![""; numcols]);
        }

        for (col, cell) in cells.iter().enumerate() {
            for (row, line) in cell.lines().enumerate() {
                matrix[row][col] = line;
            }
        }

        let mut parts = Vec::new();

        for row in matrix {
            let mut part = String::new();
            part.push_str("| ");
            part.push_str(&row.join(" | "));
            part.push_str(" |");
            parts.push(part);
        }

        Ok(parts.join("\n"))
    }

    fn render_th_element(&mut self, element: &'a Element) -> Result<String> {
        self.render_container_element(element)
    }

    fn render_td_element(&mut self, element: &'a Element) -> Result<String> {
        self.render_container_element(element)
    }

    fn render_text(&mut self, content: &str) -> Result<String> {
        Ok(decode_text(content))
    }
}

fn decode_text(text: &str) -> String {
    let mut init = String::new();
    let (_, acc) = decode_text_tail_call(text, &mut init);
    acc.to_string()
}

fn decode_text_tail_call<'a>(rest: &'a str, acc: &'a mut String) -> (&'a str, &'a String) {
    if rest.is_empty() {
        return (rest, acc);
    }

    // entity is composed with at latest 3 characters: '&' + name + ';'
    if rest.len() < 3 {
        acc.push_str(rest);
        return ("", acc);
    }

    let mut chars = rest.chars();

    match chars.next() {
        Some('&') => match chars.position(|c| c == ';') {
            Some(pos) => {
                let entity_name = rest.get(1..(pos + 1)).unwrap();
                let decoded = decode_entity(entity_name);
                acc.push_str(&decoded);
                decode_text_tail_call(rest.get((pos + 2)..).unwrap(), acc)
            }
            None => {
                acc.push_str(rest);
                ("", acc)
            }
        },
        Some(_) => match chars.position(|c| c == '&') {
            Some(pos) => {
                let plain = rest.get(0..(pos + 1)).unwrap();
                acc.push_str(plain);
                decode_text_tail_call(rest.get((pos + 1)..).unwrap(), acc)
            }
            None => {
                acc.push_str(rest);
                ("", acc)
            }
        },
        None => unreachable!(),
    }
}

fn decode_entity(name: &str) -> String {
    let mut chars = name.chars();

    match chars.next() {
        Some('#') => match chars.next() {
            Some('x') | Some('X') => {
                let hexadecimal = name.get(2..).unwrap();
                match u32::from_str_radix(hexadecimal, 16) {
                    Ok(code) => match char::from_u32(code) {
                        Some(c) => c.to_string(),
                        None => format!("&{};", name),
                    },
                    Err(_) => format!("&{};", name),
                }
            }
            Some(_) => {
                let decimal = name.get(1..).unwrap();
                match u32::from_str_radix(decimal, 10) {
                    Ok(code) => match char::from_u32(code) {
                        Some(c) => c.to_string(),
                        None => format!("&{};", name),
                    },
                    Err(_) => format!("&{};", name),
                }
            }
            None => format!("&{};", name),
        },
        _ => format!("&{};", name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_text() {
        assert_eq!(decode_text("hello world"), "hello world".to_string());

        assert_eq!(decode_text("&;"), "&;".to_string());

        assert_eq!(decode_text("&nbsp;"), "&nbsp;".to_string());
        assert_eq!(decode_text("&#1234;"), "Ӓ".to_string());
        assert_eq!(decode_text("&#xd06;"), "ആ".to_string());
        assert_eq!(decode_text("&#Xd06;"), "ആ".to_string());

        assert_eq!(decode_text("foo&#1234;"), "fooӒ".to_string());
        assert_eq!(decode_text("&#1234;foo"), "Ӓfoo".to_string());
    }
}
