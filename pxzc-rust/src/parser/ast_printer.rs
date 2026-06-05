use crate::parser::expr::Expr;

pub (crate) enum PrintMode {
    Infix,
    RPN
}

pub(crate) fn print(expr: &Expr, mode: &PrintMode) -> String {
    match expr {
        Expr::Binary { left, operator, right } =>  match mode {
            PrintMode::Infix => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    print(left, mode),
                    print(right, mode)
                )
            },
            PrintMode::RPN => {
                format!(
                    "{} {} {}",
                    print(left, mode),
                    print(right, mode),
                    operator.lexeme
                )
            }
        }
        Expr::Unary { operator, operand } => match mode {
            PrintMode::Infix => format!("({} {})", operator.lexeme, print(operand, mode)),
            PrintMode::RPN => format!("{} {}", print(operand, mode), operator.lexeme)
        }
        Expr::Grouped { expression: expr } => match mode {
            PrintMode::Infix => format!("(group {})", print(expr, mode)),
            PrintMode::RPN => format!("({})", print(expr, mode))
        }
        Expr::Literal { value } => value.to_string(),
        Expr::Variable { name } => name.lexeme.to_string(),
        Expr::Assign { name, value } => match mode{
            PrintMode::Infix => format!("({} = {})", name.lexeme, print(value, mode)),
            PrintMode::RPN => format!("{} {} =", print(value, mode), name.lexeme)
        }
    }
}