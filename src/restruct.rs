use crate::parse;

pub fn restruct<'a>(node: &'a parse::Node<'a>) -> parse::Node<'a> {
    match node {
        parse::Node::Element(element) => restruct_element(element),
        parse::Node::Text(content) => restruct_text(content),
    }
}

fn restruct_text<'a>(content: &'a str) -> parse::Node<'a> {
    parse::Node::Text(content)
}

fn restruct_element<'a>(element: &'a parse::Element) -> parse::Node<'a> {
    let new_element = match element.tag {
        "table" => restruct_table_element(element),
        _ => restruct_arbitrary_element(element),
    };
    parse::Node::Element(new_element)
}

fn restruct_arbitrary_element<'a>(element: &'a parse::Element) -> parse::Element<'a> {
    let mut children = Vec::new();
    for child in &element.children {
        children.push(restruct(&child));
    }
    parse::Element::new_with_children(element.tag, children)
}

// Ensure TABLE element structure as follows:
//
//   TABLE
//     THEAD
//       TR
//     TBODY
//       TR*
//
fn restruct_table_element<'a>(element: &'a parse::Element<'a>) -> parse::Element<'a> {
    let mut new_element = parse::Element::new("table");

    let mut tr_nodes = Vec::new();
    for child in &element.children {
        let mut child_tr_nodes = collect_tr_nodes(child);
        tr_nodes.append(&mut child_tr_nodes);
    }

    if tr_nodes.len() == 0 {
        return new_element;
    }

    let head_tr_node = tr_nodes[0];
    let thead_node = parse::Node::Element(parse::Element {
        tag: "thead",
        children: vec![head_tr_node.clone()],
    });
    new_element.children.push(thead_node);

    let mut body_tr_nodes: Vec<parse::Node> = Vec::new();
    for tr_node in tr_nodes
        .into_iter()
        .skip(1)
        .collect::<Vec<&'a parse::Node<'a>>>()
    {
        body_tr_nodes.push(tr_node.clone());
    }
    let tbody_node = parse::Node::Element(parse::Element {
        tag: "tbody",
        children: body_tr_nodes,
    });
    new_element.children.push(tbody_node);

    new_element
}

fn collect_tr_nodes<'a>(node: &'a parse::Node<'a>) -> Vec<&'a parse::Node<'a>> {
    match node {
        parse::Node::Element(element) => match element.tag {
            "tr" => vec![node],
            _ => {
                let mut nodes = Vec::new();
                for child in &element.children {
                    let mut children = collect_tr_nodes(&child);
                    nodes.append(&mut children);
                }
                nodes
            }
        },
        parse::Node::Text(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restruct_complete_structure() {
        let original_node = parse::Node::Element(parse::Element::new_with_children(
            "body",
            vec![
                parse::Node::Element(parse::Element::new_with_children(
                    "table",
                    vec![
                        parse::Node::Element(parse::Element::new_with_children(
                            "thead",
                            vec![parse::Node::Element(parse::Element::new_with_children(
                                "tr",
                                vec![
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "th",
                                        vec![parse::Node::Text("1,1")],
                                    )),
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "th",
                                        vec![parse::Node::Text("1,2")],
                                    )),
                                ],
                            ))],
                        )),
                        parse::Node::Element(parse::Element::new_with_children(
                            "tbody",
                            vec![parse::Node::Element(parse::Element::new_with_children(
                                "tr",
                                vec![
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "td",
                                        vec![parse::Node::Text("2,1")],
                                    )),
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "td",
                                        vec![parse::Node::Text("2,2")],
                                    )),
                                ],
                            ))],
                        )),
                    ],
                )),
                parse::Node::Text("Hello"),
            ],
        ));

        let expected_node = parse::Node::Element(parse::Element::new_with_children(
            "body",
            vec![
                parse::Node::Element(parse::Element::new_with_children(
                    "table",
                    vec![
                        parse::Node::Element(parse::Element::new_with_children(
                            "thead",
                            vec![parse::Node::Element(parse::Element::new_with_children(
                                "tr",
                                vec![
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "th",
                                        vec![parse::Node::Text("1,1")],
                                    )),
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "th",
                                        vec![parse::Node::Text("1,2")],
                                    )),
                                ],
                            ))],
                        )),
                        parse::Node::Element(parse::Element::new_with_children(
                            "tbody",
                            vec![parse::Node::Element(parse::Element::new_with_children(
                                "tr",
                                vec![
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "td",
                                        vec![parse::Node::Text("2,1")],
                                    )),
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "td",
                                        vec![parse::Node::Text("2,2")],
                                    )),
                                ],
                            ))],
                        )),
                    ],
                )),
                parse::Node::Text("Hello"),
            ],
        ));

        assert_eq!(restruct(&original_node), expected_node);
    }

    #[test]
    fn test_restruct_no_thead_or_tbody() {
        let original_node = parse::Node::Element(parse::Element::new_with_children(
            "body",
            vec![
                parse::Node::Element(parse::Element::new_with_children(
                    "table",
                    vec![
                        parse::Node::Element(parse::Element::new_with_children(
                            "tr",
                            vec![
                                parse::Node::Element(parse::Element::new_with_children(
                                    "th",
                                    vec![parse::Node::Text("1,1")],
                                )),
                                parse::Node::Element(parse::Element::new_with_children(
                                    "th",
                                    vec![parse::Node::Text("1,2")],
                                )),
                            ],
                        )),
                        parse::Node::Element(parse::Element::new_with_children(
                            "tr",
                            vec![
                                parse::Node::Element(parse::Element::new_with_children(
                                    "td",
                                    vec![parse::Node::Text("2,1")],
                                )),
                                parse::Node::Element(parse::Element::new_with_children(
                                    "td",
                                    vec![parse::Node::Text("2,2")],
                                )),
                            ],
                        )),
                    ],
                )),
                parse::Node::Text("Hello"),
            ],
        ));

        let expected_node = parse::Node::Element(parse::Element::new_with_children(
            "body",
            vec![
                parse::Node::Element(parse::Element::new_with_children(
                    "table",
                    vec![
                        parse::Node::Element(parse::Element::new_with_children(
                            "thead",
                            vec![parse::Node::Element(parse::Element::new_with_children(
                                "tr",
                                vec![
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "th",
                                        vec![parse::Node::Text("1,1")],
                                    )),
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "th",
                                        vec![parse::Node::Text("1,2")],
                                    )),
                                ],
                            ))],
                        )),
                        parse::Node::Element(parse::Element::new_with_children(
                            "tbody",
                            vec![parse::Node::Element(parse::Element::new_with_children(
                                "tr",
                                vec![
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "td",
                                        vec![parse::Node::Text("2,1")],
                                    )),
                                    parse::Node::Element(parse::Element::new_with_children(
                                        "td",
                                        vec![parse::Node::Text("2,2")],
                                    )),
                                ],
                            ))],
                        )),
                    ],
                )),
                parse::Node::Text("Hello"),
            ],
        ));

        assert_eq!(restruct(&original_node), expected_node);
    }
}
