use crate::ast::{AttributeMap, Element, Node};

pub fn restruct(node: &Node) -> Node {
    match node {
        Node::Element(element) => restruct_element(element),
        Node::Text(content) => restruct_text(content),
    }
}

fn restruct_text(content: &str) -> Node {
    Node::Text(content.to_string())
}

fn restruct_element(element: &Element) -> Node {
    let new_element = match element.tag.as_str() {
        "table" => restruct_table_element(element),
        _ => restruct_arbitrary_element(element),
    };
    Node::Element(new_element)
}

fn restruct_arbitrary_element(element: &Element) -> Element {
    let mut children = Vec::new();
    for child in &element.children {
        children.push(restruct(&child));
    }
    Element::new_with_children(&element.tag, &element.attributes, children)
}

// Ensure TABLE element structure as follows:
//
//   TABLE
//     THEAD
//       TR
//     TBODY
//       TR*
//
fn restruct_table_element(element: &Element) -> Element {
    let mut new_element = Element::new("table", &element.attributes);

    let mut tr_nodes = Vec::new();
    for child in &element.children {
        let mut child_tr_nodes = collect_tr_nodes(child);
        tr_nodes.append(&mut child_tr_nodes);
    }

    if tr_nodes.len() == 0 {
        return new_element;
    }

    let head_tr_node = tr_nodes[0].clone();
    let thead_node = Node::Element(Element::new_with_children(
        "thead",
        &AttributeMap::new(),
        vec![head_tr_node],
    ));
    new_element.children.push(thead_node);

    let mut body_tr_nodes: Vec<Node> = Vec::new();
    for tr_node in tr_nodes.into_iter().skip(1).collect::<Vec<Node>>() {
        body_tr_nodes.push(tr_node.clone());
    }
    let tbody_node = Node::Element(Element::new_with_children(
        "tbody",
        &AttributeMap::new(),
        body_tr_nodes,
    ));
    new_element.children.push(tbody_node);

    new_element
}

fn collect_tr_nodes(node: &Node) -> Vec<Node> {
    match node {
        Node::Element(element) => match element.tag.as_str() {
            "tr" => vec![node.clone()],
            _ => {
                let mut nodes = Vec::new();
                for child in &element.children {
                    let mut children = collect_tr_nodes(&child);
                    nodes.append(&mut children);
                }
                nodes
            }
        },
        Node::Text(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restruct_complete_structure() {
        let original_node = Node::Element(Element::new_with_children(
            "body",
            &AttributeMap::new(),
            vec![
                Node::Element(Element::new_with_children(
                    "table",
                    &AttributeMap::new(),
                    vec![
                        Node::Element(Element::new_with_children(
                            "thead",
                            &AttributeMap::new(),
                            vec![Node::Element(Element::new_with_children(
                                "tr",
                                &AttributeMap::new(),
                                vec![
                                    Node::Element(Element::new_with_children(
                                        "th",
                                        &AttributeMap::new(),
                                        vec![Node::Text("1,1".to_string())],
                                    )),
                                    Node::Element(Element::new_with_children(
                                        "th",
                                        &AttributeMap::new(),
                                        vec![Node::Text("1,2".to_string())],
                                    )),
                                ],
                            ))],
                        )),
                        Node::Element(Element::new_with_children(
                            "tbody",
                            &AttributeMap::new(),
                            vec![Node::Element(Element::new_with_children(
                                "tr",
                                &AttributeMap::new(),
                                vec![
                                    Node::Element(Element::new_with_children(
                                        "td",
                                        &AttributeMap::new(),
                                        vec![Node::Text("2,1".to_string())],
                                    )),
                                    Node::Element(Element::new_with_children(
                                        "td",
                                        &AttributeMap::new(),
                                        vec![Node::Text("2,2".to_string())],
                                    )),
                                ],
                            ))],
                        )),
                    ],
                )),
                Node::Text("Hello".to_string()),
            ],
        ));

        let expected_node = Node::Element(Element::new_with_children(
            "body",
            &AttributeMap::new(),
            vec![
                Node::Element(Element::new_with_children(
                    "table",
                    &AttributeMap::new(),
                    vec![
                        Node::Element(Element::new_with_children(
                            "thead",
                            &AttributeMap::new(),
                            vec![Node::Element(Element::new_with_children(
                                "tr",
                                &AttributeMap::new(),
                                vec![
                                    Node::Element(Element::new_with_children(
                                        "th",
                                        &AttributeMap::new(),
                                        vec![Node::Text("1,1".to_string())],
                                    )),
                                    Node::Element(Element::new_with_children(
                                        "th",
                                        &AttributeMap::new(),
                                        vec![Node::Text("1,2".to_string())],
                                    )),
                                ],
                            ))],
                        )),
                        Node::Element(Element::new_with_children(
                            "tbody",
                            &AttributeMap::new(),
                            vec![Node::Element(Element::new_with_children(
                                "tr",
                                &AttributeMap::new(),
                                vec![
                                    Node::Element(Element::new_with_children(
                                        "td",
                                        &AttributeMap::new(),
                                        vec![Node::Text("2,1".to_string())],
                                    )),
                                    Node::Element(Element::new_with_children(
                                        "td",
                                        &AttributeMap::new(),
                                        vec![Node::Text("2,2".to_string())],
                                    )),
                                ],
                            ))],
                        )),
                    ],
                )),
                Node::Text("Hello".to_string()),
            ],
        ));

        assert_eq!(restruct(&original_node), expected_node);
    }

    #[test]
    fn test_restruct_no_thead_or_tbody() {
        let original_node = Node::Element(Element::new_with_children(
            "body",
            &AttributeMap::new(),
            vec![
                Node::Element(Element::new_with_children(
                    "table",
                    &AttributeMap::new(),
                    vec![
                        Node::Element(Element::new_with_children(
                            "tr",
                            &AttributeMap::new(),
                            vec![
                                Node::Element(Element::new_with_children(
                                    "th",
                                    &AttributeMap::new(),
                                    vec![Node::Text("1,1".to_string())],
                                )),
                                Node::Element(Element::new_with_children(
                                    "th",
                                    &AttributeMap::new(),
                                    vec![Node::Text("1,2".to_string())],
                                )),
                            ],
                        )),
                        Node::Element(Element::new_with_children(
                            "tr",
                            &AttributeMap::new(),
                            vec![
                                Node::Element(Element::new_with_children(
                                    "td",
                                    &AttributeMap::new(),
                                    vec![Node::Text("2,1".to_string())],
                                )),
                                Node::Element(Element::new_with_children(
                                    "td",
                                    &AttributeMap::new(),
                                    vec![Node::Text("2,2".to_string())],
                                )),
                            ],
                        )),
                    ],
                )),
                Node::Text("Hello".to_string()),
            ],
        ));

        let expected_node = Node::Element(Element::new_with_children(
            "body",
            &AttributeMap::new(),
            vec![
                Node::Element(Element::new_with_children(
                    "table",
                    &AttributeMap::new(),
                    vec![
                        Node::Element(Element::new_with_children(
                            "thead",
                            &AttributeMap::new(),
                            vec![Node::Element(Element::new_with_children(
                                "tr",
                                &AttributeMap::new(),
                                vec![
                                    Node::Element(Element::new_with_children(
                                        "th",
                                        &AttributeMap::new(),
                                        vec![Node::Text("1,1".to_string())],
                                    )),
                                    Node::Element(Element::new_with_children(
                                        "th",
                                        &AttributeMap::new(),
                                        vec![Node::Text("1,2".to_string())],
                                    )),
                                ],
                            ))],
                        )),
                        Node::Element(Element::new_with_children(
                            "tbody",
                            &AttributeMap::new(),
                            vec![Node::Element(Element::new_with_children(
                                "tr",
                                &AttributeMap::new(),
                                vec![
                                    Node::Element(Element::new_with_children(
                                        "td",
                                        &AttributeMap::new(),
                                        vec![Node::Text("2,1".to_string())],
                                    )),
                                    Node::Element(Element::new_with_children(
                                        "td",
                                        &AttributeMap::new(),
                                        vec![Node::Text("2,2".to_string())],
                                    )),
                                ],
                            ))],
                        )),
                    ],
                )),
                Node::Text("Hello".to_string()),
            ],
        ));

        assert_eq!(restruct(&original_node), expected_node);
    }
}
