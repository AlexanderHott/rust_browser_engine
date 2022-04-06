use std::default::Default;
use std::fmt;

#[derive(PartialEq)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declarations>,
}

#[derive(PartialEq, Eq)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}

#[derive(PartialEq, Eq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
}

#[derive(PartialEq)]
pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
}

#[derive(PartialEq)]
pub enum Unit {
    Em,
    Ex,
    Ch,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Pct,
}

#[derive(PartialEq, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules }
    }
}
impl Default for Stylesheet {
    fn default() -> Self {
        Stylesheet { rules: Vec::new() }
    }
}
impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str("\n\n");
            }
            rule_result.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rule_result)
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
        Rule {
            selectors,
            declarations,
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "    ";
        for selector in &self.selectors {
            if sel_result.len() > 0 {
                sel_result.push_str(", ");
            }
            sel_result.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            decl_result.push_str(tab);
            decl_result.push_str(&format!("{:?}", declaration));
            decl_result.push('\n');
        }

        write!(f, "{} {{\n{}}}", sel_result, decl_result)
    }
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Selector {
        Selector {
            simple,
            combinators,
        }
    }
}

impl Defaut for Selector {
    fn default() -> Self {
        Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
        }
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel));
        }
        write!(f, "{}", result)
    }
}

impl SimpleSelector {
    pub fn new(
        tag_name: Option<String>,
        id: Option<String>,
        classes: Vec<String>,
    ) -> SimpleSelector {
        SimpleSelector {
            tag_name,
            id,
            classes,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        }
    }
}

impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        match self.tag_name {
            Some(ref tag_name) => result.push_str(&format!("{}", tag_name)),
            None => result.push_str("*"),
        }

        match self.id {
            Some(ref id) => result.push_str(&format!("#{}", id)),
            None => {}
        }

        for class in &self.classes {
            result.push_str(&format!(".{}", class));
        }

        write!(f, "{}", result)
    }
}

impl fmt::Default for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Color(ref color) => write!(f, "{:?}", color),
            Value::Length(ref length, ref unit) => write!(f, "{:?} {}", length, unit),
            Value::Other(ref other) => write!(f, "{:?}", other),
        }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
