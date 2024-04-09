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

pub struct Renderer<'a> {
    ctx: Context,
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
        let mut result = self.render_node("", self.root)?;
        if !result.ends_with("\n") {
            result.push_str("\n");
        }
        Ok(result)
    }

    fn render_node(&mut self, parent_tag: &str, node: &Node) -> Result<String> {
        self.ctx.push(parent_tag);

        let result = match node {
            Node::Element(element) => self.render_element(element),
            Node::Text(content) => self.render_text(content),
        };

        self.ctx.pop();

        result
    }

    fn render_element(&mut self, element: &Element) -> Result<String> {
        match element.tag.as_str() {
            "a" => self.render_a_element(element),
            "abbr" => self.render_abbr_element(element),
            "address" => self.render_address_element(element),
            "article" => self.render_article_element(element),
            "aside" => self.render_aside_element(element),
            "b" => self.render_b_element(element),
            "bdi" => self.render_bdi_element(element),
            "bdo" => self.render_bdo_element(element),
            "blockquote" => self.render_blockquote_element(element),
            "body" => self.render_body_element(element),
            "br" => self.render_br_element(element),
            "cite" => self.render_cite_element(element),
            "code" => self.render_code_element(element),
            "data" => self.render_data_element(element),
            "dd" => self.render_dd_element(element),
            "del" => self.render_del_element(element),
            "details" => self.render_details_element(element),
            "dfn" => self.render_dfn_element(element),
            "div" => self.render_div_element(element),
            "dl" => self.render_dl_element(element),
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
            "i" => self.render_i_element(element),
            "img" => self.render_img_element(element),
            "ins" => self.render_ins_element(element),
            "kbd" => self.render_kbd_element(element),
            "li" => self.render_li_element(element),
            "main" => self.render_main_element(element),
            "mark" => self.render_mark_element(element),
            "menu" => self.render_menu_element(element),
            "nav" => self.render_nav_element(element),
            "ol" => self.render_ol_element(element),
            "p" => self.render_p_element(element),
            "pre" => self.render_pre_element(element),
            "q" => self.render_q_element(element),
            "rp" => self.render_rp_element(element),
            "rt" => self.render_rt_element(element),
            "ruby" => self.render_ruby_element(element),
            "s" => self.render_s_element(element),
            "samp" => self.render_samp_element(element),
            "section" => self.render_section_element(element),
            "small" => self.render_small_element(element),
            "span" => self.render_span_element(element),
            "strong" => self.render_strong_element(element),
            "sub" => self.render_sub_element(element),
            "summary" => self.render_summary_element(element),
            "sup" => self.render_sup_element(element),
            "time" => self.render_time_element(element),
            "u" => self.render_u_element(element),
            "ul" => self.render_ul_element(element),
            "var" => self.render_var_element(element),
            "wbr" => self.render_wbr_element(element),

            // table
            "table" => self.render_table_element(element),
            "thead" => self.render_thead_element(element),
            "tbody" => self.render_tbody_element(element),
            "tr" => self.render_tr_element(element),
            "th" => self.render_th_element(element),
            "td" => self.render_td_element(element),
            "caption" | "colgroup" | "col" | "tfoot" => self.render_nothing(element),

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

    fn render_children(&mut self, element: &Element) -> Result<String> {
        let mut result = String::new();

        for child in &element.children {
            let content = self.render_node(&element.tag, &child)?;
            result.push_str(&content);
        }

        Ok(result)
    }

    fn render_ctxed_children(&mut self, element: &Element) -> Result<String> {
        let mut parts = Vec::new();

        for node in &element.children {
            let content = self.render_node(&element.tag, &node)?;
            parts.push(content);
        }

        Ok(parts.join("\n"))
    }

    fn render_container_element(&mut self, element: &Element) -> Result<String> {
        let mut parts = Vec::new();
        let mut part = String::new();

        for node in &element.children {
            let content = self.render_node(&element.tag, &node)?;

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

    fn render_nothing(&mut self, _: &Element) -> Result<String> {
        Ok(String::new())
    }

    fn render_unsupported_element(&mut self, element: &Element) -> Result<String> {
        eprintln!(
            "`{}` element is not supported. rendering nothing.",
            element.tag
        );
        self.render_nothing(element)
    }

    fn render_element_in_html_form(&mut self, element: &Element) -> Result<String> {
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
        let content = self.render_children(element)?;

        Self::wrap(&content, &open_tag, &close_tag)
    }

    fn wrap(content: &str, prefix: &str, suffix: &str) -> Result<String> {
        let mut result = String::new();
        result.push_str(prefix);
        result.push_str(&content);
        result.push_str(suffix);
        Ok(result)
    }

    fn render_a_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;

        if element.attributes.contains_key("name") {
            self.render_element_in_html_form(element)
        } else if let Some(href) = element.attributes.get("href") {
            Ok(format!("[{}]({})", content, href))
        } else {
            Ok(content)
        }
    }

    fn render_abbr_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_address_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_article_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_aside_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_b_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_bdi_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_bdo_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_blockquote_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_container_element(element)?;

        let mut parts = Vec::new();
        for line in content.lines() {
            parts.push(format!("> {}", line));
        }
        Ok(parts.join("\n"))
    }

    fn render_body_element(&mut self, element: &Element) -> Result<String> {
        self.render_container_element(element)
    }

    fn render_br_element(&mut self, _: &Element) -> Result<String> {
        Ok(String::from("\n"))
    }

    fn render_cite_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_code_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "`", "`")
    }

    fn render_data_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_dd_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_del_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "~", "~")
    }

    fn render_details_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_dfn_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_div_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_dl_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_dt_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_em_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "_", "_")
    }

    fn render_h1_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "# ", "")
    }

    fn render_h2_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "## ", "")
    }

    fn render_h3_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "### ", "")
    }

    fn render_h4_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "#### ", "")
    }

    fn render_h5_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "##### ", "")
    }

    fn render_h6_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "###### ", "")
    }

    fn render_hr_element(&mut self, _: &Element) -> Result<String> {
        Ok(String::from("---"))
    }

    fn render_html_element(&mut self, element: &Element) -> Result<String> {
        if let Some(body_node) = element.children.iter().find(|node| match node {
            Node::Element(e) => e.tag == "body",
            _ => false,
        }) {
            self.render_node(&element.tag, body_node)
        } else {
            unreachable!()
        }
    }

    fn render_i_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_img_element(&mut self, element: &Element) -> Result<String> {
        self.render_element_in_html_form(element)
    }

    fn render_ins_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_kbd_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_li_element(&mut self, element: &Element) -> Result<String> {
        let mut result = String::new();

        let marker = match self.ctx.get_last_list_tag() {
            Some("ul") => "-",
            Some("ol") => "1.",
            _ => return Err(RenderError::OutsideOfList),
        };

        let content = self.render_container_element(element)?;
        let content_with_marker = Self::prepend_list_marker(marker, &content);
        result.push_str(&content_with_marker);

        Ok(result)
    }

    fn prepend_list_marker(marker: &str, content: &str) -> String {
        let mut parts = Vec::new();

        let sp = Self::spaces(marker.len());
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

    fn render_main_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_mark_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_menu_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_nav_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_ol_element(&mut self, element: &Element) -> Result<String> {
        self.render_ctxed_children(element)
    }

    fn render_p_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "", "")
    }

    fn render_pre_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_q_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_rp_element(&mut self, element: &Element) -> Result<String> {
        self.render_nothing(element)
    }

    fn render_rt_element(&mut self, element: &Element) -> Result<String> {
        self.render_nothing(element)
    }

    fn render_ruby_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_s_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_samp_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_section_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_small_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_span_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_strong_element(&mut self, element: &Element) -> Result<String> {
        let content = self.render_children(element)?;
        Self::wrap(&content, "**", "**")
    }

    fn render_sub_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_summary_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_sup_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_table_element(&mut self, element: &Element) -> Result<String> {
        self.render_ctxed_children(element)
    }

    fn render_thead_element(&mut self, element: &Element) -> Result<String> {
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
        result.push_str("|");

        Ok(result)
    }

    fn render_tbody_element(&mut self, element: &Element) -> Result<String> {
        self.render_ctxed_children(element)
    }

    fn render_tr_element(&mut self, element: &Element) -> Result<String> {
        let mut cells = Vec::new();
        for child in &element.children {
            let cell = self.render_node(&element.tag, &child)?;
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

    fn render_th_element(&mut self, element: &Element) -> Result<String> {
        self.render_container_element(element)
    }

    fn render_td_element(&mut self, element: &Element) -> Result<String> {
        self.render_container_element(element)
    }

    fn render_time_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_u_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_ul_element(&mut self, element: &Element) -> Result<String> {
        self.render_ctxed_children(element)
    }

    fn render_var_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_wbr_element(&mut self, element: &Element) -> Result<String> {
        self.render_children(element)
    }

    fn render_text(&mut self, content: &str) -> Result<String> {
        Ok(content.to_string())
    }
}
