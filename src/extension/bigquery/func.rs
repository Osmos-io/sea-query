use crate::{Function, FunctionCall, SimpleExpr};

use super::DateTimePart;

#[derive(Debug, Clone)]
pub enum BqFunction {
    // Aggregate Functions

    // Timestamp Functions
    Timestamp,
    TimestampAdd,
    TimestampSub,
    TimestampDiff,
    TimestampTrunc,
    FormatTimestamp,
    ParseTimestamp,
    TimestampSeconds,
    TimestampMillis,
    TimestampMicros,
    UnixSeconds,
    UnixMillis,
    UnixMicros,

    // Mathematical Functions
    Greatest,

    // Json Functions
    JsonExtractScalar,
}

#[derive(Debug, Clone)]
pub struct BqFunc;

impl BqFunc {
    /// Timestamp
    ///
    /// Additional information and examples are available in the [BQ
    /// documentation].
    ///
    /// ```text
    /// TIMESTAMP(string_expression[, time_zone])
    /// TIMESTAMP(date_expression[, time_zone])
    /// TIMESTAMP(datetime_expression[, time_zone])
    /// ```
    ///
    /// ## Description
    /// - `string_expression[, time_zone]`: Converts a STRING expression to a
    ///   TIMESTAMP data type. string_expression must include a timestamp
    ///   literal. If `string_expression` includes a time_zone in the timestamp
    ///   literal, do not include an explicit `time_zone` argument.
    /// - `date_expression[, time_zone]`: Converts a DATE object to a TIMESTAMP
    ///   data type. The value returned is the earliest timestamp that falls
    ///   within the given date.
    /// - `datetime_expression[, time_zone]`: Converts a DATETIME object to a
    ///   TIMESTAMP data type.
    ///
    /// This function supports an optional parameter to specify a time zone. If
    /// no time zone is specified, the default time zone, UTC, is used.
    ///
    /// # Return Type
    /// `TIMESTAMP`
    ///
    /// # Examples
    ///
    /// ```text
    /// SELECT TIMESTAMP("2008-12-25 15:30:00+00") AS timestamp_str;
    /// +-------------------------+
    /// | timestamp_str           |
    /// +-------------------------+
    /// | 2008-12-25 15:30:00 UTC |
    /// +-------------------------+
    /// ```
    ///
    /// [BQ documentation]: https://cloud.google.com/bigquery/docs/reference/standard-sql/timestamp_functions#timestamp
    pub fn timestamp<T, U>(expr: T, tz: Option<U>) -> FunctionCall
    where
        T: Into<SimpleExpr>,
        U: Into<SimpleExpr>,
    {
        match tz {
            Some(tz) => FunctionCall::new(Function::BqFunction(BqFunction::Timestamp))
                .args([expr.into(), tz.into()]),
            None => FunctionCall::new(Function::BqFunction(BqFunction::Timestamp)).arg(expr),
        }
    }

    pub fn timestamp_add<T, U>(ts_expr: T, date_part: U) -> FunctionCall
    where
        T: Into<SimpleExpr>,
        U: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::BqFunction(BqFunction::TimestampAdd))
            .args([ts_expr.into(), date_part.into()])
    }

    pub fn timestamp_sub<T, U>(ts_expr: T, date_part: U) -> FunctionCall
    where
        T: Into<SimpleExpr>,
        U: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::BqFunction(BqFunction::TimestampSub))
            .args([ts_expr.into(), date_part.into()])
    }

    pub fn timestamp_trunc<T>(expr: T, date_part: DateTimePart) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::BqFunction(BqFunction::TimestampTrunc))
            .args([expr.into(), date_part.into()])
    }

    pub fn timestamp_millis<T>(expr: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::BqFunction(BqFunction::TimestampMillis)).arg(expr)
    }

    pub fn greatest<I>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        FunctionCall::new(Function::BqFunction(BqFunction::Greatest)).args(args)
    }

    pub fn json_extract_scalar<T, U>(expr: T, json_path: U) -> FunctionCall
    where
        T: Into<SimpleExpr>,
        U: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::BqFunction(BqFunction::JsonExtractScalar))
            .args([expr.into(), json_path.into()])
    }
}
