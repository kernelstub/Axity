#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    String,
    Bool,
    Array(Box<Type>),
    Map(Box<Type>),
    Class(String),
}

