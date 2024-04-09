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

struct ContextItem {
    tag: String,
}

impl ContextItem {
    fn new(tag: &str) -> Self {
        ContextItem {
            tag: tag.to_string(),
        }
    }
}

struct Context {
    items: Vec<ContextItem>,
}

impl Context {
    fn new() -> Self {
        let items = Vec::new();
        Context { items }
    }

    fn push(&mut self, tag: &str) -> () {
        let item = ContextItem::new(tag);
        self.items.push(item)
    }

    fn pop(&mut self) -> () {
        self.items.pop();
    }

    fn get_last_list_tag(&mut self) -> Option<&str> {
        for item in self.items.iter().rev() {
            if item.tag == "ul" || item.tag == "ol" {
                return Some(&item.tag);
            }
        }
        None
    }
}

pub fn render(node: &Node) -> Result<String> {
    let mut ctx = Context::new();

    let mut result = render_node("", node, &mut ctx)?;
    if !result.ends_with("\n") {
        result.push_str("\n");
    }
    Ok(result)
}

fn render_node(parent_tag: &str, node: &Node, ctx: &mut Context) -> Result<String> {
    ctx.push(parent_tag);

    let result = match node {
        Node::Element(element) => render_element(element, ctx),
        Node::Text(content) => render_text(content),
    };

    ctx.pop();

    result
}

fn render_element(element: &Element, ctx: &mut Context) -> Result<String> {
    match element.tag.as_str() {
        "a" => render_a_element(element, ctx),
        "abbr" => render_abbr_element(element, ctx),
        "address" => render_address_element(element, ctx),
        "article" => render_article_element(element, ctx),
        "aside" => render_aside_element(element, ctx),
        "b" => render_b_element(element, ctx),
        "bdi" => render_bdi_element(element, ctx),
        "bdo" => render_bdo_element(element, ctx),
        "blockquote" => render_blockquote_element(element, ctx),
        "body" => render_body_element(element, ctx),
        "br" => render_br_element(element, ctx),
        "cite" => render_cite_element(element, ctx),
        "code" => render_code_element(element, ctx),
        "data" => render_data_element(element, ctx),
        "dd" => render_dd_element(element, ctx),
        "del" => render_del_element(element, ctx),
        "details" => render_details_element(element, ctx),
        "dfn" => render_dfn_element(element, ctx),
        "div" => render_div_element(element, ctx),
        "dl" => render_dl_element(element, ctx),
        "dt" => render_dt_element(element, ctx),
        "em" => render_em_element(element, ctx),
        "h1" => render_h1_element(element, ctx),
        "h2" => render_h2_element(element, ctx),
        "h3" => render_h3_element(element, ctx),
        "h4" => render_h4_element(element, ctx),
        "h5" => render_h5_element(element, ctx),
        "h6" => render_h6_element(element, ctx),
        "hr" => render_hr_element(element, ctx),
        "html" => render_html_element(element, ctx),
        "i" => render_i_element(element, ctx),
        "img" => render_img_element(element, ctx),
        "ins" => render_ins_element(element, ctx),
        "kbd" => render_kbd_element(element, ctx),
        "li" => render_li_element(element, ctx),
        "main" => render_main_element(element, ctx),
        "mark" => render_mark_element(element, ctx),
        "menu" => render_menu_element(element, ctx),
        "nav" => render_nav_element(element, ctx),
        "ol" => render_ol_element(element, ctx),
        "p" => render_p_element(element, ctx),
        "pre" => render_pre_element(element, ctx),
        "q" => render_q_element(element, ctx),
        "rp" => render_rp_element(element, ctx),
        "rt" => render_rt_element(element, ctx),
        "ruby" => render_ruby_element(element, ctx),
        "s" => render_s_element(element, ctx),
        "samp" => render_samp_element(element, ctx),
        "section" => render_section_element(element, ctx),
        "small" => render_small_element(element, ctx),
        "span" => render_span_element(element, ctx),
        "strong" => render_strong_element(element, ctx),
        "sub" => render_sub_element(element, ctx),
        "summary" => render_summary_element(element, ctx),
        "sup" => render_sup_element(element, ctx),
        "time" => render_time_element(element, ctx),
        "u" => render_u_element(element, ctx),
        "ul" => render_ul_element(element, ctx),
        "var" => render_var_element(element, ctx),
        "wbr" => render_wbr_element(element, ctx),

        // table
        "table" => render_table_element(element, ctx),
        "thead" => render_thead_element(element, ctx),
        "tbody" => render_tbody_element(element, ctx),
        "tr" => render_tr_element(element, ctx),
        "th" => render_th_element(element, ctx),
        "td" => render_td_element(element, ctx),
        "caption" | "colgroup" | "col" | "tfoot" => render_nothing(element, ctx),

        // render nothing
        "area" | "audio" | "button" | "canvas" | "datalist" | "dialog" | "embed" | "fieldset"
        | "figcaption" | "figure" | "footer" | "form" | "header" | "hgroup" | "iframe"
        | "input" | "label" | "legend" | "map" | "meter" | "noscript" | "object" | "optgroup"
        | "option" | "output" | "picture" | "progress" | "script" | "search" | "select"
        | "slot" | "source" | "template" | "textarea" | "track" | "video" => {
            render_nothing(element, ctx)
        }

        // unsupported
        _ => render_unsupported_element(element, ctx),
    }
}

fn render_children(element: &Element, ctx: &mut Context) -> Result<String> {
    let mut result = String::new();

    for child in &element.children {
        let content = render_node(&element.tag, &child, ctx)?;
        result.push_str(&content);
    }

    Ok(result)
}

fn render_ctxed_children(element: &Element, ctx: &mut Context) -> Result<String> {
    let mut parts = Vec::new();

    for node in &element.children {
        let content = render_node(&element.tag, &node, ctx)?;
        parts.push(content);
    }

    Ok(parts.join("\n"))
}

fn render_container_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let mut parts = Vec::new();
    let mut part = String::new();

    for node in &element.children {
        let content = render_node(&element.tag, &node, ctx)?;

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

fn render_nothing(_: &Element, _: &mut Context) -> Result<String> {
    Ok(String::new())
}

fn render_unsupported_element(element: &Element, ctx: &mut Context) -> Result<String> {
    eprintln!(
        "`{}` element is not supported. rendering nothing.",
        element.tag
    );
    render_nothing(element, ctx)
}

fn render_element_in_html_form(element: &Element, ctx: &mut Context) -> Result<String> {
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
    let content = render_children(element, ctx)?;

    wrap(&content, &open_tag, &close_tag)
}

fn wrap(content: &str, prefix: &str, suffix: &str) -> Result<String> {
    let mut result = String::new();
    result.push_str(prefix);
    result.push_str(&content);
    result.push_str(suffix);
    Ok(result)
}

fn render_a_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;

    if element.attributes.contains_key("name") {
        render_element_in_html_form(element, ctx)
    } else if let Some(href) = element.attributes.get("href") {
        Ok(format!("[{}]({})", content, href))
    } else {
        Ok(content)
    }
}

fn render_abbr_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_address_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_article_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_aside_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_b_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_bdi_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_bdo_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_blockquote_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_container_element(element, ctx)?;

    let mut parts = Vec::new();
    for line in content.lines() {
        parts.push(format!("> {}", line));
    }
    Ok(parts.join("\n"))
}

fn render_body_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_container_element(element, ctx)
}

fn render_br_element(_: &Element, _: &mut Context) -> Result<String> {
    Ok(String::from("\n"))
}

fn render_cite_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_code_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "`", "`")
}

fn render_data_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_dd_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_del_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "~", "~")
}

fn render_details_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_dfn_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_div_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_dl_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_dt_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_em_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "_", "_")
}

fn render_h1_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "# ", "")
}

fn render_h2_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "## ", "")
}

fn render_h3_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "### ", "")
}

fn render_h4_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "#### ", "")
}

fn render_h5_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "##### ", "")
}

fn render_h6_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "###### ", "")
}

fn render_hr_element(_: &Element, _: &mut Context) -> Result<String> {
    Ok(String::from("---"))
}

fn render_html_element(element: &Element, ctx: &mut Context) -> Result<String> {
    if let Some(body_node) = element.children.iter().find(|node| match node {
        Node::Element(e) => e.tag == "body",
        _ => false,
    }) {
        render_node(&element.tag, body_node, ctx)
    } else {
        unreachable!()
    }
}

fn render_i_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_img_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_element_in_html_form(element, ctx)
}

fn render_ins_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_kbd_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_li_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let mut result = String::new();

    let marker = match ctx.get_last_list_tag() {
        Some("ul") => "-",
        Some("ol") => "1.",
        _ => return Err(RenderError::OutsideOfList),
    };

    let content = render_container_element(element, ctx)?;
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

fn render_main_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_mark_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_menu_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_nav_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_ol_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_ctxed_children(element, ctx)
}

fn render_p_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "", "")
}

fn render_pre_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_q_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_rp_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_nothing(element, ctx)
}

fn render_rt_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_nothing(element, ctx)
}

fn render_ruby_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_s_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_samp_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_section_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_small_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_span_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_strong_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let content = render_children(element, ctx)?;
    wrap(&content, "**", "**")
}

fn render_sub_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_summary_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_sup_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_table_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_ctxed_children(element, ctx)
}

fn render_thead_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let mut parts = Vec::new();

    let tr = render_children(element, ctx)?;
    parts.push(tr);

    let separator = render_table_separator_with_tr_node(&element.children[0], ctx)?;
    parts.push(separator);

    Ok(parts.join("\n"))
}

fn render_table_separator_with_tr_node(node: &Node, _: &mut Context) -> Result<String> {
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

fn render_tbody_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_ctxed_children(element, ctx)
}

fn render_tr_element(element: &Element, ctx: &mut Context) -> Result<String> {
    let mut cells = Vec::new();
    for child in &element.children {
        let cell = render_node(&element.tag, &child, ctx)?;
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

fn render_th_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_container_element(element, ctx)
}

fn render_td_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_container_element(element, ctx)
}

fn render_time_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_u_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_ul_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_ctxed_children(element, ctx)
}

fn render_var_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_wbr_element(element: &Element, ctx: &mut Context) -> Result<String> {
    render_children(element, ctx)
}

fn render_text(content: &str) -> Result<String> {
    Ok(content.to_string())
}
