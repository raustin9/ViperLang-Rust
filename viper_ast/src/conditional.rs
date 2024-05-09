use std::{fmt::Display, sync::Arc};

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
    condition: Option<Arc<ExprNode>>,

    /// The body of the code. Should 
    /// be a CodeBlock expression
    body: Arc<ExprNode>,

    /// If Some() -> has else clause
    /// if None -> no else clause
    else_clause: Option<Arc<ExprNode>>,
}

impl Conditional {
    pub fn new(
        condition: Option<Arc<ExprNode>>,
        body: Arc<ExprNode>,
        else_clause: Option<Arc<ExprNode>>,
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
            Some(ref condition) => format!("if {}", condition).to_string(),
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
                str += format!("{} else {}", '}', expr).as_str();
            }
            None => {
                str += "}\n";
            }
        }


        write!(f, "{str}")
    }
}
