use crate::{private::Expression, Expr, SimpleExpr};

use super::DateTimePart;

pub trait BqExpr: Expression {
    fn interval(quantity: i64, unit: DateTimePart) -> SimpleExpr {
        SimpleExpr::Custom(format!("INTERVAL {quantity} {}", unit.as_str()))
    }
}

impl BqExpr for SimpleExpr {}
impl BqExpr for Expr {}
