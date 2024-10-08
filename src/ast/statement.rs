use super::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Option<Expression>),
}
