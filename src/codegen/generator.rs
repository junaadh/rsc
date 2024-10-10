use crate::{
    ast::{
        Ast, Block, Decleration, Expression, Function, Literal, LiteralKind, PrimaryExpression,
        Program, Statement,
    },
    Res,
};

// add instructions into codegen internal buffer
// use specialized code this ir maybe develop??
pub struct CodeGen;

impl CodeGen {
    pub fn generate(&mut self, ast: &Ast) -> Res<String> {
        self.generate_program(&ast.program)
    }

    fn generate_program(&mut self, program: &Program) -> Res<String> {
        let mut declarations = Vec::new();

        for decl in program.declarations.iter() {
            declarations.push(self.generate_declarations(decl)?);
        }

        Ok(declarations.join("\n"))
    }

    fn generate_declarations(&mut self, declaration: &Decleration) -> Res<String> {
        match declaration {
            Decleration::Fuction(func) => self.generate_function(func),
        }
    }

    fn generate_function(&mut self, function: &Function) -> Res<String> {
        let body = self.generate_block(&function.block)?;

        Ok(format!(
            "\r\t.globl _{name}\n\n_{name}:\n{body}\n",
            name = function.name.ident
        ))
    }

    fn generate_block(&mut self, block: &Block) -> Res<String> {
        let mut statements = Vec::new();

        for stmt in block.statements.iter() {
            statements.push(self.generate_statements(stmt)?);
        }

        Ok(statements.join("\n"))
    }

    fn generate_statements(&mut self, statement: &Statement) -> Res<String> {
        match statement {
            Statement::Return(None) => Ok("\tret".to_string()),
            Statement::Return(Some(val)) => {
                let body = self.generate_expressions(val)?;

                Ok(format!("\tmov\tw0,  {body}\n\tret"))
            }
        }
    }

    fn generate_expressions(&mut self, expression: &Expression) -> Res<String> {
        match expression {
            Expression::Primary(p) => self.generate_primarys(p),
        }
    }

    fn generate_primarys(&mut self, primary: &PrimaryExpression) -> Res<String> {
        match primary {
            PrimaryExpression::Literal(Literal {
                kind: LiteralKind::Int(i),
                ..
            }) => Ok(format!("#{}", i)),
            _ => todo!(),
        }
    }
}
