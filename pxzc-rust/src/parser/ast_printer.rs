use crate::parser::expr::Expr;

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary { left, operator, right } => {
            format!("({} {} {})", operator.lexeme(), print(left), print(right))
        }
        Expr::Unary { operator, operand } => {
            format!("({} {})", operator.lexeme(), print(operand))
        }
        Expr::Grouped { expression: expr } => {
            format!("(group {})", print(expr))
        }
        Expr::Literal { token } => token.to_string(),
    }
}