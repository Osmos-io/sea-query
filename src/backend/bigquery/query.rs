use crate::{BigQueryQueryBuilder, BqFunction, Function, QueryBuilder, SqlWriter};

impl QueryBuilder for BigQueryQueryBuilder {
    fn prepare_query_statement(
        &self,
        query: &crate::SubQueryStatement,
        sql: &mut dyn crate::SqlWriter,
    ) {
        query.prepare_statement(self, sql)
    }

    fn prepare_value(&self, value: &crate::Value, sql: &mut dyn crate::SqlWriter) {
        sql.push_param(value.clone(), self as _);
    }

    fn prepare_function(&self, function: &Function, sql: &mut dyn SqlWriter) {
        match function {
            Function::BqFunction(function) => {
                let fn_name = match function {
                    BqFunction::Timestamp => "TIMESTAMP",
                    BqFunction::TimestampAdd => "TIMESTAMP_ADD",
                    BqFunction::TimestampSub => "TIMESTAMP_SUB",
                    BqFunction::TimestampDiff => "TIMESTAMP_DIFF",
                    BqFunction::TimestampTrunc => "TIMESTAMP_TRUNC",
                    BqFunction::FormatTimestamp => "FORMAT_TIMESTAMP",
                    BqFunction::ParseTimestamp => "PARSE_TIMESTAMP",
                    BqFunction::TimestampSeconds => "TIMESTAMP_SECONDS",
                    BqFunction::TimestampMillis => "TIMESTAMP_MILLIS",
                    BqFunction::TimestampMicros => "TIMESTAMP_MICROS",
                    BqFunction::UnixSeconds => "UNIX_SECONDS",
                    BqFunction::UnixMillis => "UNIX_MILLIS",
                    BqFunction::UnixMicros => "UNIX_MICROS",

                    // Mathematical Functions
                    BqFunction::Greatest => "GREATEST",

                    // Json Functions
                    BqFunction::JsonExtractScalar => "JSON_EXTRACT_SCALAR"
                };

                write!(sql, "{}", fn_name).unwrap()
            }
            _ => self.prepare_function_common(function, sql),
        }
    }
}
