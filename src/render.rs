use crate::parse;
use std::fmt;

pub type Result<T> = std::result::Result<T, RenderError>;

#[derive(Debug, PartialEq)]
pub enum RenderError {
    // UnexpectedNode,
}

impl fmt::Display for RenderError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // RenderError::UnexpectedNode => write!(f, "unexpected node"),
        }
    }
}

impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            // RenderError::UnexpectedElement => None,
        }
    }
}

struct Context {}

impl Context {
    fn new() -> Self {
        Context {}
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

    fn push(&mut self) -> () {
        let ctx = Context::new();
        self.stack.push(ctx)
    }

    fn pop(&mut self) -> () {
        self.stack.pop();
    }
}

pub fn render(node: &parse::Node) -> Result<String> {
    let mut stack = ContextStack::new();

    let mut result = render_node(node, &mut stack)?;
    if !result.ends_with("\n") {
        result.push_str("\n");
    }
    Ok(result)
}

fn render_node(node: &parse::Node, stack: &mut ContextStack) -> Result<String> {
    stack.push();

    let result = match node {
        parse::Node::Element(element) => render_element(element, stack),
        parse::Node::Text(content) => render_text(content),
    };

    stack.pop();

    result
}

fn render_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    match element.tag {
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

        // for table, render() dispatches only table, th, and tr elements and builds its structure.
        "table" => render_table_element(element, stack),
        "th" | "td" => render_table_cell_for_tr_or_td_element(element, stack),
        "caption" | "colgroup" | "col" | "tbocy" | "tfoot" | "thead" | "tr" => {
            render_nothing(element, stack)
        }

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

fn render_children(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let mut result = String::new();

    for node in &element.children {
        let content = render_node(&node, stack)?;
        result.push_str(&content);
    }

    Ok(result)
}

fn render_nothing(_: &parse::Element, _: &mut ContextStack) -> Result<String> {
    Ok(String::new())
}

fn render_unsupported_element(
    element: &parse::Element,
    stack: &mut ContextStack,
) -> Result<String> {
    eprintln!(
        "`{}` element is not supported. rendering nothing.",
        element.tag
    );
    render_nothing(element, stack)
}

fn wrap(content: &str, prefix: &str, suffix: &str) -> Result<String> {
    let mut result = String::new();
    result.push_str(prefix);
    result.push_str(&content);
    result.push_str(suffix);
    Ok(result)
}

fn render_a_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_abbr_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_address_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_article_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_aside_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_b_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_bdi_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_bdo_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_blockquote_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let mut result = String::new();
    let content = render_children(element, stack)?;
    for line in content.lines() {
        result.push_str("> ");
        result.push_str(line);
        result.push_str("\n");
    }
    Ok(result)
}

fn render_body_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_br_element(_: &parse::Element, _: &mut ContextStack) -> Result<String> {
    Ok(String::from("\n"))
}

fn render_cite_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_code_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "`", "`")
}

fn render_data_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dd_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_del_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "~", "~")
}

fn render_details_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dfn_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_div_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dl_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_dt_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_em_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "_", "_")
}

fn render_h1_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "# ", "\n")
}

fn render_h2_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "## ", "\n")
}

fn render_h3_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "### ", "\n")
}

fn render_h4_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "#### ", "\n")
}

fn render_h5_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "##### ", "\n")
}

fn render_h6_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "###### ", "\n")
}

fn render_hr_element(_: &parse::Element, _: &mut ContextStack) -> Result<String> {
    Ok(String::from("\n\n---\n\n"))
}

fn render_html_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    if let Some(body_node) = element.children.iter().find(|node| match node {
        parse::Node::Element(e) => e.tag == "body",
        _ => false,
    }) {
        render_node(body_node, stack)
    } else {
        unreachable!()
    }
}

fn render_i_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_img_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_ins_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_kbd_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_li_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_main_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_mark_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_menu_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_nav_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_ol_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_p_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "", "\n\n")
}

fn render_pre_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_q_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_rp_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_nothing(element, stack)
}

fn render_rt_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_nothing(element, stack)
}

fn render_ruby_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_s_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_samp_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_section_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_small_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_span_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_strong_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let content = render_children(element, stack)?;
    wrap(&content, "**", "**")
}

fn render_sub_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_summary_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_sup_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn collect_tr_elements_in_children<'a>(
    element: &'a parse::Element,
) -> Result<Vec<&'a parse::Element<'a>>> {
    let mut result = Vec::new();

    for child_node in &element.children {
        match child_node {
            parse::Node::Element(child_element) => {
                if child_element.tag == "tr" {
                    result.push(child_element);
                } else {
                    let mut descendants = collect_tr_elements_in_children(&child_element)?;
                    result.append(&mut descendants);
                }
            }
            _ => continue,
        }
    }

    Ok(result)
}

fn render_table_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    let tr_elements = collect_tr_elements_in_children(element)?;
    let head_tr_element = tr_elements[0];
    let body_tr_elements: Vec<&parse::Element> = tr_elements.into_iter().skip(1).collect();

    let mut result = String::new();

    let head = render_table_row_for_tr_element(head_tr_element, stack)?;
    result.push_str(&head);

    let separator = render_table_separator_for_head_tr_element(head_tr_element, stack)?;
    result.push_str(&separator);

    for body_tr_element in body_tr_elements {
        let content = render_table_row_for_tr_element(body_tr_element, stack)?;
        result.push_str(&content);
    }

    result.push_str("\n");
    Ok(result)
}

fn render_table_separator_for_head_tr_element(
    element: &parse::Element,
    _: &mut ContextStack,
) -> Result<String> {
    let mut result = String::new();

    for _ in &element.children {
        result.push_str("|---")
    }
    result.push_str("|\n");

    Ok(result)
}

fn render_table_row_for_tr_element(
    element: &parse::Element,
    stack: &mut ContextStack,
) -> Result<String> {
    let mut result = String::new();

    for child_node in &element.children {
        result.push_str("| ");

        let content = render_node(child_node, stack)?;
        result.push_str(&content);

        result.push_str(" ");
    }
    result.push_str("|\n");

    Ok(result)
}

fn render_table_cell_for_tr_or_td_element(
    element: &parse::Element,
    stack: &mut ContextStack,
) -> Result<String> {
    render_children(element, stack)
}

fn render_time_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_u_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_ul_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_var_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_wbr_element(element: &parse::Element, stack: &mut ContextStack) -> Result<String> {
    render_children(element, stack)
}

fn render_text(content: &str) -> Result<String> {
    Ok(content.to_string())
}
