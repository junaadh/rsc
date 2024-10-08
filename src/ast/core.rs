use super::Decleration;

#[derive(Debug, Clone)]
pub struct Ast {
    pub program: Program,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Decleration>,
}
