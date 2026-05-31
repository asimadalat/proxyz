use crate::parser::expr::Expr;

pub(crate) fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary { left, operator, right } => {
            format!("({} {} {})", operator.lexeme, print(left), print(right))
        }
        Expr::Unary { operator, operand } => {
            format!("({} {})", operator.lexeme, print(operand))
        }
        Expr::Grouped { expression: expr } => {
            format!("(group {})", print(expr))
        }
        Expr::Literal { value } => value.to_string(),
        Expr::Variable { name } => name.lexeme.to_string()
    }
}

pub(crate) fn to_rpn(expr: &Expr) -> String {
    match expr {
        Expr::Binary { left, operator, right } => {
            format!("{} {} {}", to_rpn(left), to_rpn(right), operator.lexeme)
        }
        Expr::Unary { operator, operand } => {
            format!("{} {}", to_rpn(operand), operator.lexeme)
        }
        Expr::Grouped { expression: expr } => {
            format!("({})", to_rpn(expr))
        }
        Expr::Literal { value } => value.to_string(),
        Expr::Variable { name } => name.lexeme.to_string()
    }
}