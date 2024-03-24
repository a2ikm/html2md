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

pub fn render(node: &parse::Node) -> Result<String> {
    let mut result = render_node(node)?;
    if !result.ends_with("\n") {
        result.push_str("\n");
    }
    Ok(result)
}

pub fn render_node(node: &parse::Node) -> Result<String> {
    match node {
        parse::Node::Element(element) => render_element(element),
        parse::Node::Text(content) => render_text(content),
    }
}

fn render_element(element: &parse::Element) -> Result<String> {
    match element.tag {
        "a" => render_a_element(element),
        "abbr" => render_abbr_element(element),
        "address" => render_address_element(element),
        "article" => render_article_element(element),
        "aside" => render_aside_element(element),
        "b" => render_b_element(element),
        "bdi" => render_bdi_element(element),
        "bdo" => render_bdo_element(element),
        "blockquote" => render_blockquote_element(element),
        "body" => render_body_element(element),
        "br" => render_br_element(element),
        "cite" => render_cite_element(element),
        "code" => render_code_element(element),
        "data" => render_data_element(element),
        "dd" => render_dd_element(element),
        "del" => render_del_element(element),
        "details" => render_details_element(element),
        "dfn" => render_dfn_element(element),
        "div" => render_div_element(element),
        "dl" => render_dl_element(element),
        "dt" => render_dt_element(element),
        "em" => render_em_element(element),
        "h1" => render_h1_element(element),
        "h2" => render_h2_element(element),
        "h3" => render_h3_element(element),
        "h4" => render_h4_element(element),
        "h5" => render_h5_element(element),
        "h6" => render_h6_element(element),
        "hr" => render_hr_element(element),
        "html" => render_html_element(element),
        "i" => render_i_element(element),
        "img" => render_img_element(element),
        "ins" => render_ins_element(element),
        "kbd" => render_kbd_element(element),
        "li" => render_li_element(element),
        "main" => render_main_element(element),
        "mark" => render_mark_element(element),
        "menu" => render_menu_element(element),
        "nav" => render_nav_element(element),
        "ol" => render_ol_element(element),
        "p" => render_p_element(element),
        "pre" => render_pre_element(element),
        "q" => render_q_element(element),
        "rp" => render_rp_element(element),
        "rt" => render_rt_element(element),
        "ruby" => render_ruby_element(element),
        "s" => render_s_element(element),
        "samp" => render_samp_element(element),
        "section" => render_section_element(element),
        "small" => render_small_element(element),
        "span" => render_span_element(element),
        "strong" => render_strong_element(element),
        "sub" => render_sub_element(element),
        "summary" => render_summary_element(element),
        "sup" => render_sup_element(element),
        "time" => render_time_element(element),
        "u" => render_u_element(element),
        "ul" => render_ul_element(element),
        "var" => render_var_element(element),
        "wbr" => render_wbr_element(element),

        // for table, render() dispatches only table, th, and tr elements and builds its structure.
        "table" => render_table_element(element),
        "th" | "td" => render_table_cell_for_tr_or_td_element(element),
        "caption" | "colgroup" | "col" | "tbocy" | "tfoot" | "thead" | "tr" => {
            render_nothing(element)
        }

        // render nothing
        "area" | "audio" | "button" | "canvas" | "datalist" | "dialog" | "embed" | "fieldset"
        | "figcaption" | "figure" | "footer" | "form" | "header" | "hgroup" | "iframe"
        | "input" | "label" | "legend" | "map" | "meter" | "noscript" | "object" | "optgroup"
        | "option" | "output" | "picture" | "progress" | "script" | "search" | "select"
        | "slot" | "source" | "template" | "textarea" | "track" | "video" => {
            render_nothing(element)
        }

        // unsupported
        _ => render_unsupported_element(element),
    }
}

fn render_children(element: &parse::Element) -> Result<String> {
    let mut result = String::new();

    for node in &element.children {
        let content = render_node(&node)?;
        result.push_str(&content);
    }

    Ok(result)
}

fn render_nothing(_: &parse::Element) -> Result<String> {
    Ok(String::new())
}

fn render_unsupported_element(element: &parse::Element) -> Result<String> {
    eprintln!(
        "`{}` element is not supported. rendering nothing.",
        element.tag
    );
    render_nothing(element)
}

fn wrap(content: &str, prefix: &str, suffix: &str) -> Result<String> {
    let mut result = String::new();
    result.push_str(prefix);
    result.push_str(&content);
    result.push_str(suffix);
    Ok(result)
}

fn render_a_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_abbr_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_address_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_article_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_aside_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_b_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_bdi_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_bdo_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_blockquote_element(element: &parse::Element) -> Result<String> {
    let mut result = String::new();
    let content = render_children(element)?;
    for line in content.lines() {
        result.push_str("> ");
        result.push_str(line);
        result.push_str("\n");
    }
    Ok(result)
}

fn render_body_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_br_element(_: &parse::Element) -> Result<String> {
    Ok(String::from("\n"))
}

fn render_cite_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_code_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "`", "`")
}

fn render_data_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_dd_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_del_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "~", "~")
}

fn render_details_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_dfn_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_div_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_dl_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_dt_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_em_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "_", "_")
}

fn render_h1_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "# ", "\n")
}

fn render_h2_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "## ", "\n")
}

fn render_h3_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "### ", "\n")
}

fn render_h4_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "#### ", "\n")
}

fn render_h5_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "##### ", "\n")
}

fn render_h6_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "###### ", "\n")
}

fn render_hr_element(_: &parse::Element) -> Result<String> {
    Ok(String::from("\n\n---\n\n"))
}

fn render_html_element(element: &parse::Element) -> Result<String> {
    let body_node = element.children.iter().find(|node| match node {
        parse::Node::Element(e) => e.tag == "body",
        _ => false,
    });
    match body_node {
        Some(parse::Node::Element(body_element)) => render_body_element(body_element),
        _ => unreachable!(),
    }
}

fn render_i_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_img_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_ins_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_kbd_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_li_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_main_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_mark_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_menu_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_nav_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_ol_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_p_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "", "\n\n")
}

fn render_pre_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_q_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_rp_element(element: &parse::Element) -> Result<String> {
    render_nothing(element)
}

fn render_rt_element(element: &parse::Element) -> Result<String> {
    render_nothing(element)
}

fn render_ruby_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_s_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_samp_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_section_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_small_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_span_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_strong_element(element: &parse::Element) -> Result<String> {
    let content = render_children(element)?;
    wrap(&content, "**", "**")
}

fn render_sub_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_summary_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_sup_element(element: &parse::Element) -> Result<String> {
    render_children(element)
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

fn render_table_element(element: &parse::Element) -> Result<String> {
    let tr_elements = collect_tr_elements_in_children(element)?;
    let head_tr_element = tr_elements[0];
    let body_tr_elements: Vec<&parse::Element> = tr_elements.into_iter().skip(1).collect();

    let mut result = String::new();

    let head = render_table_row_for_tr_element(head_tr_element)?;
    result.push_str(&head);

    let separator = render_table_separator_for_head_tr_element(head_tr_element)?;
    result.push_str(&separator);

    for body_tr_element in body_tr_elements {
        let content = render_table_row_for_tr_element(body_tr_element)?;
        result.push_str(&content);
    }

    result.push_str("\n");
    Ok(result)
}

fn render_table_separator_for_head_tr_element(element: &parse::Element) -> Result<String> {
    let mut result = String::new();

    for _ in &element.children {
        result.push_str("|---")
    }
    result.push_str("|\n");

    Ok(result)
}

fn render_table_row_for_tr_element(element: &parse::Element) -> Result<String> {
    let mut result = String::new();

    for child_node in &element.children {
        result.push_str("| ");

        let content = render_node(child_node)?;
        result.push_str(&content);

        result.push_str(" ");
    }
    result.push_str("|\n");

    Ok(result)
}

fn render_table_cell_for_tr_or_td_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_time_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_u_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_ul_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_var_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_wbr_element(element: &parse::Element) -> Result<String> {
    render_children(element)
}

fn render_text(content: &str) -> Result<String> {
    Ok(content.to_string())
}
