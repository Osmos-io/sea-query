pub mod func;
pub use func::*;

pub mod expr;
pub use expr::*;

use crate::{SimpleExpr, Iden, Keyword, IntoIden};

#[derive(Debug, Clone, Copy)]
pub enum DateTimePart {
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    IsoWeek,
    Month,
    Quarter,
    Year,
    IsoYear,
}

impl Iden for DateTimePart {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", self.as_str()).unwrap()
    }
}

impl From<DateTimePart> for SimpleExpr {
    fn from(part: DateTimePart) -> Self {
        SimpleExpr::Keyword(Keyword::Custom(part.into_iden()))    
    }
}

impl DateTimePart {
    pub fn as_str(&self) -> &'static str {
        match self {
            DateTimePart::Microsecond => "MICROSECOND",
            DateTimePart::Millisecond => "MILLISECOND",
            DateTimePart::Second => "SECOND",
            DateTimePart::Minute => "MINUTE",
            DateTimePart::Hour => "HOUR",
            DateTimePart::Day => "DAY",
            DateTimePart::Week => "WEEK",
            DateTimePart::IsoWeek => "ISOWEEK",
            DateTimePart::Month => "MONTH",
            DateTimePart::Quarter => "QUARTER",
            DateTimePart::Year => "YEAR",
            DateTimePart::IsoYear => "ISOYEAR",
        }
    }
}
