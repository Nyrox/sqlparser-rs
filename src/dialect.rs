// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Support for custom dialects

pub trait Dialect {
    /// Get a list of keywords for this dialect
    fn keywords(&self) -> Vec<&'static str>;
}

pub struct AnsiSqlDialect {
}

impl Dialect for AnsiSqlDialect {
    fn keywords(&self) -> Vec<&'static str> {
        return vec!["ABS", "ALL", "ALLOCATE", "ALTER", "AND", "ANY", "ARE", "ARRAY", "ARRAY_AGG",
                    "ARRAY_MAX_CARDINALITY", "AS", "ASENSITIVE", "ASYMMETRIC", "AT", "ATOMIC", "AUTHORIZATION",
                    "AVG", "BEGIN", "BEGIN_FRAME", "BEGIN_PARTITION", "BETWEEN", "BIGINT", "BINARY", "BLOB",
                    "BOOLEAN", "BOTH", "BY", "CALL", "CALLED", "CARDINALITY", "CASCADED", "CASE", "CAST", "CEIL",
                    "CEILING", "CHAR", "CHAR_LENGTH", "CHARACTER", "CHARACTER_LENGTH", "CHECK", "CLOB", "CLOSE",
                    "COALESCE", "COLLATE", "COLLECT", "COLUMN", "COMMIT", "CONDITION", "CONNECT", "CONSTRAINT",
                    "CONTAINS", "CONVERT", "CORR", "CORRESPONDING", "COUNT", "COVAR_POP", "COVAR_SAMP", "CREATE",
                    "CROSS", "CUBE", "CUME_DIST", "CURRENT", "CURRENT_CATALOG", "CURRENT_DATE",
                    "CURRENT_DEFAULT_TRANSFORM_GROUP", "CURRENT_PATH", "CURRENT_ROLE", "CURRENT_ROW",
                    "CURRENT_SCHEMA", "CURRENT_TIME", "CURRENT_TIMESTAMP", "CURRENT_TRANSFORM_GROUP_FOR_TYPE",
                    "CURRENT_USER", "CURSOR", "CYCLE", "DATE", "DAY", "DEALLOCATE", "DEC", "DECIMAL", "DECLARE",
                    "DEFAULT", "DELETE", "DENSE_RANK", "DEREF", "DESCRIBE", "DETERMINISTIC", "DISCONNECT",
                    "DISTINCT", "DOUBLE", "DROP", "DYNAMIC", "EACH", "ELEMENT", "ELSE", "END", "END_FRAME",
                    "END_PARTITION", "END-EXEC", "EQUALS", "ESCAPE", "EVERY", "EXCEPT", "EXEC", "EXECUTE",
                    "EXISTS", "EXP", "EXTERNAL", "EXTRACT", "FALSE", "FETCH", "FILTER", "FIRST_VALUE", "FLOAT",
                    "FLOOR", "FOR", "FOREIGN", "FRAME_ROW", "FREE", "FROM", "FULL", "FUNCTION", "FUSION",
                    "GET", "GLOBAL", "GRANT", "GROUP", "GROUPING", "GROUPS", "HAVING", "HOLD", "HOUR", "IDENTITY",
                    "IN", "INDICATOR", "INNER", "INOUT", "INSENSITIVE", "INSERT", "INT", "INTEGER", "INTERSECT",
                    "INTERSECTION", "INTERVAL", "INTO", "IS", "JOIN", "LAG", "LANGUAGE", "LARGE", "LAST_VALUE",
                    "LATERAL", "LEAD", "LEADING", "LEFT", "LIKE", "LIKE_REGEX", "LN", "LOCAL", "LOCALTIME",
                    "LOCALTIMESTAMP", "LOWER", "MATCH", "MAX", "MEMBER", "MERGE", "METHOD", "MIN", "MINUTE",
                    "MOD", "MODIFIES", "MODULE", "MONTH", "MULTISET", "NATIONAL", "NATURAL", "NCHAR", "NCLOB",
                    "NEW", "NO", "NONE", "NORMALIZE", "NOT", "NTH_VALUE", "NTILE", "NULL", "NULLIF", "NUMERIC",
                    "OCTET_LENGTH", "OCCURRENCES_REGEX", "OF", "OFFSET", "OLD", "ON", "ONLY", "OPEN", "OR",
                    "ORDER", "OUT", "OUTER", "OVER", "OVERLAPS", "OVERLAY", "PARAMETER", "PARTITION", "PERCENT",
                    "PERCENT_RANK", "PERCENTILE_CONT", "PERCENTILE_DISC", "PERIOD", "PORTION", "POSITION",
                    "POSITION_REGEX", "POWER", "PRECEDES", "PRECISION", "PREPARE", "PRIMARY",
                    "PROCEDURE", "RANGE", "RANK", "READS", "REAL", "RECURSIVE", "REF", "REFERENCES",
                    "REFERENCING", "REGR_AVGX", "REGR_AVGY", "REGR_COUNT", "REGR_INTERCEPT", "REGR_R2",
                    "REGR_SLOPE", "REGR_SXX", "REGR_SXY", "REGR_SYY", "RELEASE", "RESULT", "RETURN", "RETURNS",
                    "REVOKE", "RIGHT", "ROLLBACK", "ROLLUP", "ROW", "ROW_NUMBER", "ROWS", "SAVEPOINT",
                    "SCOPE", "SCROLL", "SEARCH", "SECOND", "SELECT", "SENSITIVE", "SESSION_USER", "SET",
                    "SIMILAR", "SMALLINT", "SOME", "SPECIFIC", "SPECIFICTYPE", "SQL", "SQLEXCEPTION", "SQLSTATE",
                    "SQLWARNING", "SQRT", "START", "STATIC", "STDDEV_POP", "STDDEV_SAMP", "SUBMULTISET",
                    "SUBSTRING", "SUBSTRING_REGEX", "SUCCEEDS", "SUM", "SYMMETRIC", "SYSTEM", "SYSTEM_TIME",
                    "SYSTEM_USER", "TABLE", "TABLESAMPLE", "THEN", "TIME", "TIMESTAMP", "TIMEZONE_HOUR",
                    "TIMEZONE_MINUTE", "TO", "TRAILING", "TRANSLATE", "TRANSLATE_REGEX", "TRANSLATION",
                    "TREAT", "TRIGGER", "TRUNCATE", "TRIM", "TRIM_ARRAY", "TRUE", "UESCAPE", "UNION", "UNIQUE",
                    "UNKNOWN", "UNNEST", "UPDATE", "UPPER", "USER", "USING", "VALUE", "VALUES", "VALUE_OF",
                    "VAR_POP", "VAR_SAMP", "VARBINARY", "VARCHAR", "VARYING", "VERSIONING", "WHEN", "WHENEVER",
                    "WHERE", "WIDTH_BUCKET", "WINDOW", "WITH", "WITHIN", "WITHOUT", "YEAR"];
    }
}

pub struct GenericSqlDialect {}

impl Dialect for GenericSqlDialect {
    fn keywords(&self) -> Vec<&'static str> {
        return vec![
            "SELECT",
            "FROM",
        "WHERE",
        "LIMIT",
        "ORDER",
        "GROUP",
        "BY",
        "HAVING",
        "UNION",
        "ALL",
        "INSERT",
        "UPDATE",
        "DELETE",
        "IN",
        "IS",
        "NULL",
        "SET",
        "CREATE",
        "EXTERNAL",
        "TABLE",
        "ASC",
        "DESC",
        "AND",
        "OR",
        "NOT",
        "AS",
        "STORED",
        "CSV",
        "PARQUET",
        "LOCATION",
        "WITH",
        "WITHOUT",
        "HEADER",
        "ROW",

        // SQL types
        "CHAR",
        "CHARACTER",
        "VARYING",
        "LARGE",
        "OBJECT",
        "VARCHAR",
        "CLOB",
        "BINARY",
        "VARBINARY",
        "BLOB",
        "FLOAT",
        "REAL",
        "DOUBLE",
        "PRECISION",
        "INT",
        "INTEGER",
        "SMALLINT",
        "BIGINT",
        "NUMERIC",
        "DECIMAL",
        "DEC",
        "BOOLEAN",
        "DATE",
        "TIME",
        "TIMESTAMP",
        
        ];
    }
}
