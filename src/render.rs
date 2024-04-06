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
            _ => None,
        }
    }
}

struct Context {
    tag: String,
}

impl Context {
    fn new(tag: &str) -> Self {
        Context {
            tag: tag.to_string(),
        }
    }
}

struct ContextStack {
    stack: Vec<Context>,
}

impl ContextStack {
    fn new() -> Self {
        let stack = Vec::new();
        ContextStack { stack }
    }

    fn push(&mut self, tag: &str) -> () {
        let ctx = Context::new(tag);
        self.stack.push(ctx)
    }

    fn pop(&mut self) -> () {
        self.stack.pop();
    }

    fn get_last_list_tag(&mut self) -> Option<&str> {
        for ctx in self.stack.iter().rev() {
            if ctx.tag == "ul" || ctx.tag == "ol" {
                return Some(&ctx.tag);
            }
        }
        None
    }
}

pub fn render(node: &Node) -> Result<String> {
    let mut stack = ContextStack::new();

    let mut result = render_node("", node, &mut stack)?;
    if !result.ends_with("\n") {
        result.push_str("\n");
    }
    Ok(result)
}

fn render_node(parent_tag: &str, node: &Node, stack: &mut ContextStack) -> Result<String> {
    stack.push(parent_tag);

    let result = match node {
        Node::Element(element) => render_element(element, stack),
        Node::Text(content) => render_text(content),
    };

    stack.pop();

    result
}

fn render_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    match element.tag.as_str() {
        "a" => render_a_element(element, stack),
        "abbr" => render_abbr_element(element, stack),
        "address" => render_address_element(element, stack),
        "article" => render_article_element(element, stack),
        "aside" => render_aside_element(element, stack),
        "b" => render_b_element(element, stack),
        "bdi" => render_bdi_element(element, stack),
        "bdo" => render_bdo_element(element, stack),
        "blockquote" => render_blockquote_element(element, stack),
        "body" => render_body_element(element, stack),
        "br" => render_br_element(element, stack),
        "cite" => render_cite_element(element, stack),
        "code" => render_code_element(element, stack),
        "data" => render_data_element(element, stack),
        "dd" => render_dd_element(element, stack),
        "del" => render_del_element(element, stack),
        "details" => render_details_element(element, stack),
        "dfn" => render_dfn_element(element, stack),
        "div" => render_div_element(element, stack),
        "dl" => render_dl_element(element, stack),
        "dt" => render_dt_element(element, stack),
        "em" => render_em_element(element, stack),
        "h1" => render_h1_element(element, stack),
        "h2" => render_h2_element(element, stack),
        "h3" => render_h3_element(element, stack),
        "h4" => render_h4_element(element, stack),
        "h5" => render_h5_element(element, stack),
        "h6" => render_h6_element(element, stack),
        "hr" => render_hr_element(element, stack),
        "html" => render_html_element(element, stack),
        "i" => render_i_element(element, stack),
        "img" => render_img_element(element, stack),
        "ins" => render_ins_element(element, stack),
        "kbd" => render_kbd_element(element, stack),
        "li" => render_li_element(element, stack),
        "main" => render_main_element(element, stack),
        "mark" => render_mark_element(element, stack),
        "menu" => render_menu_element(element, stack),
        "nav" => render_nav_element(element, stack),
        "ol" => render_ol_element(element, stack),
        "p" => render_p_element(element, stack),
        "pre" => render_pre_element(element, stack),
        "q" => render_q_element(element, stack),
        "rp" => render_rp_element(element, stack),
        "rt" => render_rt_element(element, stack),
        "ruby" => render_ruby_element(element, stack),
        "s" => render_s_element(element, stack),
        "samp" => render_samp_element(element, stack),
        "section" => render_section_element(element, stack),
        "small" => render_small_element(element, stack),
        "span" => render_span_element(element, stack),
        "strong" => render_strong_element(element, stack),
        "sub" => render_sub_element(element, stack),
        "summary" => render_summary_element(element, stack),
        "sup" => render_sup_element(element, stack),
        "time" => render_time_element(element, stack),
        "u" => render_u_element(element, stack),
        "ul" => render_ul_element(element, stack),
        "var" => render_var_element(element, stack),
        "wbr" => render_wbr_element(element, stack),

        // table
        "table" => render_table_element(element, stack),
        "thead" => render_thead_element(element, stack),
        "tbody" => render_tbody_element(element, stack),
        "tr" => render_tr_element(element, stack),
        "th" => render_th_element(element, stack),
        "td" => render_td_element(element, stack),
        "caption" | "colgroup" | "col" | "tfoot" => render_nothing(element, stack),

        // render nothing
        "area" | "audio" | "button" | "canvas" | "datalist" | "dialog" | "embed" | "fieldset"
        | "figcaption" | "figure" | "footer" | "form" | "header" | "hgroup" | "iframe"
        | "input" | "label" | "legend" | "map" | "meter" | "noscript" | "object" | "optgroup"
        | "option" | "output" | "picture" | "progress" | "script" | "search" | "select"
        | "slot" | "source" | "template" | "textarea" | "track" | "video" => {
            render_nothing(element, stack)
        }

        // unsupported
        _ => render_unsupported_element(element, stack),
    }
}

fn render_children(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let mut result = String::new();

    for child in &element.children {
        let content = render_node(&element.tag, &child, stack)?;
        result.push_str(&content);
    }

    Ok(result)
}

fn render_stacked_children(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let mut parts = Vec::new();

    for node in &element.children {
        let content = render_node(&element.tag, &node, stack)?;
        parts.push(content);
    }

    Ok(parts.join("\n"))
}

fn render_container_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let mut parts = Vec::new();
    let mut part = String::new();

    for node in &element.children {
        let content = render_node(&element.tag, &node, stack)?;

        match node {
            Node::Element(child) => {
                if is_block_element(&child.tag) && part.len() > 0 {
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

fn render_nothing(_: &Element, _: &mut ContextStack) -> Result<String> {
    Ok(String::new())
}

fn render_unsupported_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    eprintln!(
        "`{}` element is not supported. rendering nothing.",
        element.tag
    );
    render_nothing(element, stack)
}

fn render_element_in_html_form(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let mut open_tag = String::new();
    open_tag.push_str("<");
    open_tag.push_str(&element.tag);
    if element.attributes.len() > 0 {
        let mut names: Vec<&String> = element.attributes.keys().collect();
        names.sort();

        for name in names {
            let value = element.attributes.get(name).unwrap();
            open_tag.push_str(&format!(" {}=\"{}\"", name, value));
        }
    }
    open_tag.push_str(">");

    if is_void_element(&element.tag) {
        return Ok(open_tag);
    }

    let close_tag = format!("</{}>", &element.tag);
    let content = render_children(element, stack)?;

    wrap(&content, &open_tag, &close_tag)
}

fn wrap(content: &str, prefix: &str, suffix: &str) -> Result<String> {
    let mut result = String::new();
    result.push_str(prefix);
    result.push_str(&content);
    result.push_str(suffix);
    Ok(result)
}

fn render_a_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;

    if element.attributes.contains_key("name") {
        render_element_in_html_form(element, stack)
    } else if let Some(href) = element.attributes.get("href") {
        Ok(format!("[{}]({})", content, href))
    } else {
        Ok(content)
    }
}

fn render_abbr_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_address_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_article_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_aside_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_b_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_bdi_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_bdo_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_blockquote_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_container_element(element, stack)?;

    let mut parts = Vec::new();
    for line in content.lines() {
        parts.push(format!("> {}", line));
    }
    Ok(parts.join("\n"))
}

fn render_body_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_container_element(element, stack)
}

fn render_br_element(_: &Element, _: &mut ContextStack) -> Result<String> {
    Ok(String::from("\n"))
}

fn render_cite_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_code_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "`", "`")
}

fn render_data_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dd_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_del_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "~", "~")
}

fn render_details_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dfn_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_div_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dl_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dt_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_em_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "_", "_")
}

fn render_h1_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "# ", "")
}

fn render_h2_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "## ", "")
}

fn render_h3_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "### ", "")
}

fn render_h4_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "#### ", "")
}

fn render_h5_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "##### ", "")
}

fn render_h6_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "###### ", "")
}

fn render_hr_element(_: &Element, _: &mut ContextStack) -> Result<String> {
    Ok(String::from("---"))
}

fn render_html_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    if let Some(body_node) = element.children.iter().find(|node| match node {
        Node::Element(e) => e.tag == "body",
        _ => false,
    }) {
        render_node(&element.tag, body_node, stack)
    } else {
        unreachable!()
    }
}

fn render_i_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_img_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_element_in_html_form(element, stack)
}

fn render_ins_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_kbd_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_li_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let mut result = String::new();

    let marker = match stack.get_last_list_tag() {
        Some("ul") => "-",
        Some("ol") => "1.",
        _ => return Err(RenderError::OutsideOfList),
    };

    let content = render_container_element(element, stack)?;
    let content_with_marker = prepend_list_marker(marker, &content);
    result.push_str(&content_with_marker);

    Ok(result)
}

fn prepend_list_marker(marker: &str, content: &str) -> String {
    let mut parts = Vec::new();

    let sp = spaces(marker.len());
    for (i, line) in content.lines().enumerate() {
        let mut part = String::new();
        if i == 0 {
            part.push_str(marker);
        } else {
            part.push_str(&sp);
        }
        part.push_str(" ");
        part.push_str(line);
        parts.push(part);
    }

    parts.join("\n")
}

fn spaces(len: usize) -> String {
    let mut result = String::new();
    for _ in 0..len {
        result.push_str(" ");
    }
    result
}

fn render_main_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_mark_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_menu_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_nav_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_ol_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_stacked_children(element, stack)
}

fn render_p_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "", "")
}

fn render_pre_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_q_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_rp_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_nothing(element, stack)
}

fn render_rt_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_nothing(element, stack)
}

fn render_ruby_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_s_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_samp_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_section_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_small_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_span_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_strong_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "**", "**")
}

fn render_sub_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_summary_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_sup_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_table_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_stacked_children(element, stack)
}

fn render_thead_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let mut parts = Vec::new();

    let tr = render_children(element, stack)?;
    parts.push(tr);

    let separator = render_table_separator_with_tr_node(&element.children[0], stack)?;
    parts.push(separator);

    Ok(parts.join("\n"))
}

fn render_table_separator_with_tr_node(node: &Node, _: &mut ContextStack) -> Result<String> {
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
    result.push_str("|");

    Ok(result)
}

fn render_tbody_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_stacked_children(element, stack)
}

fn render_tr_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    let mut cells = Vec::new();
    for child in &element.children {
        let cell = render_node(&element.tag, &child, stack)?;
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
        let mut row = Vec::with_capacity(numcols);
        for _ in 0..numcols {
            row.push("");
        }
        matrix.push(row);
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

fn render_th_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_container_element(element, stack)
}

fn render_td_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_container_element(element, stack)
}

fn render_time_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_u_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_ul_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_stacked_children(element, stack)
}

fn render_var_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_wbr_element(element: &Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_text(content: &str) -> Result<String> {
    Ok(content.to_string())
}
