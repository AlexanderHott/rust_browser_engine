use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData {
            tag_name,
            attributes,
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(" ").collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref s) | NodeType::Comment(ref s) => write!(f, "{}", s),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}", self.tag_name)?;
        for (k, v) in &self.attributes {
            write!(f, " {}=\"{}\"", k, v)?;
        }
        write!(f, ">")
    }
}

pub fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Text(ref s) => println!("{}{:?}", indent, s),
        NodeType::Element(ref e) => {
            println!("{}<{}", indent, e.tag_name);
            for (k, v) in &e.attributes {
                println!("{}  {}=\"{}\"", indent, k, v);
            }
            println!("{}>", indent);
            for c in &n.children {
                pretty_print(c, indent_size + 2);
            }
            println!("{}</{}>", indent, e.tag_name);
        }
        // NodeType::Element(ref e) => println!("{}{:?}", indent, e);
        NodeType::Comment(ref s) => println!("{}<!--{}-->", indent, s),
    }

    // for child in n.children.iter() {
    //     pretty_print(child, indent_size + 2);
    // }

    // match n.node_type {
    //     NodeType::Element(ref e) => println!("{}</{}>", indent, e.tag_name),
    //     _ => (),
    // }
}
