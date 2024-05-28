use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::ExprNode;

/// Represents a contional expression
/// if, elif, and else statements
/// `
/// if <condition> {
/// ...
/// } elif <condition> {
/// } else {
/// }
/// `
#[derive(Clone, Debug)]
pub struct Conditional {
    /// The condition expression
    /// If true the body will execute
    /// If false it will either go to 
    /// the `else_clause` if it exists
    condition: Option<Rc<RefCell<ExprNode>>>,

    /// The body of the code. Should 
    /// be a CodeBlock expression
    body: Rc<ExprNode>,

    /// If Some() -> has else clause
    /// if None -> no else clause
    else_clause: Option<Rc<RefCell<ExprNode>>>,
}

impl Conditional {
    pub fn new(
        condition: Option<Rc<RefCell<ExprNode>>>,
        body: Rc<ExprNode>,
        else_clause: Option<Rc<RefCell<ExprNode>>>,
    ) -> Conditional {
        Conditional {
            condition,
            body,
            else_clause,
        }
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cond = match &self.condition {
            Some(ref condition) => format!("if {}", condition.borrow()).to_string(),
            None => "".into(),
        };

        let mut str = String::from(
            format!("{} {}\n{}", 
                cond,
                '{', 
                self.body
            )
        );

        match &self.else_clause {
            Some(expr) => {
                str += format!("{} else {}", '}', expr.borrow()).as_str();
            }
            None => {
                str += "}\n";
            }
        }


        write!(f, "{str}")
    }
}
