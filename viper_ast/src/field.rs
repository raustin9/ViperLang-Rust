use std::fmt::Display;


#[derive(Clone, Debug)]
pub struct Field {
    name: String,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
