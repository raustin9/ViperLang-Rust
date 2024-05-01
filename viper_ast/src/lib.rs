use viper_core::span::Span;

#[derive(Debug)]
pub struct AST {
}

pub struct Node<T> {
    span: Span,
    inner: T,
}
