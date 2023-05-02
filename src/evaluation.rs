use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub fn replace_variables_tag(
    str: &str,
    input_str: &str,
    variables: &HashMap<String, String>,
) -> String {
    let mut str = str.to_string();
    str = str.replace("{{input}}", input_str);
    for (key, value) in variables {
        str = str.replace(&format!("{{{{{}}}}}", key.to_lowercase()), value);
    }
    str
}

pub fn replace_variables_tag_vec(
    vec_of_str: &Vec<String>,
    input_str: &str,
    variables: &HashMap<String, String>,
) -> Vec<String> {
    vec_of_str
        .iter()
        .map(|str| replace_variables_tag(str, input_str, variables))
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[derive(TS)]
#[ts(export)]
pub enum Comparison {
    Number(NumberComparison),
    String(StringComparison),
}

impl Comparison {
    pub fn from(
        input_str: &str,
        variables: &HashMap<String, String>,
        operation: &str,
        a: &str,
        b: &str,
    ) -> anyhow::Result<Comparison> {
        let operation = replace_variables_tag(operation, input_str, variables);
        let a = replace_variables_tag(a, input_str, variables);
        let b = replace_variables_tag(b, input_str, variables);

        match operation.as_str() {
            "string_equals" => Ok(Comparison::String(StringComparison::Equals { a, b })),
            "string_not_equals" => Ok(Comparison::String(StringComparison::NotEquals { a, b })),
            "string_contains" => Ok(Comparison::String(StringComparison::Contains { a, b })),
            "string_not_contains" => Ok(Comparison::String(StringComparison::NotContains { a, b })),
            "string_starts_with" => Ok(Comparison::String(StringComparison::StartsWith { a, b })),
            "string_ends_with" => Ok(Comparison::String(StringComparison::EndsWith { a, b })),
            "string_is_empty" => Ok(Comparison::String(StringComparison::IsEmpty { a })),
            "string_is_not_empty" => Ok(Comparison::String(StringComparison::IsNotEmpty { a })),
            "==" => Ok(Comparison::Number(NumberComparison::Equal {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            })),
            "!=" => Ok(Comparison::Number(NumberComparison::NotEqual {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            })),
            ">" => Ok(Comparison::Number(NumberComparison::GreaterThan {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            })),
            "<" => Ok(Comparison::Number(NumberComparison::LessThan {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            })),
            ">=" => Ok(Comparison::Number(NumberComparison::GreaterThanOrEqual {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            })),
            "<=" => Ok(Comparison::Number(NumberComparison::LessThanOrEqual {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            })),
            _ => Err(anyhow::anyhow!("Unknown operation: {}", operation)),
        }
    }

    #[allow(dead_code)]
    pub fn to_serialized(self) -> SerializedComparison {
        match self {
            Comparison::Number(NumberComparison::Equal { a, b }) => SerializedComparison {
                operation: "==".to_string(),
                a: a.to_string(),
                b: b.to_string(),
            },
            Comparison::Number(NumberComparison::NotEqual { a, b }) => SerializedComparison {
                operation: "!=".to_string(),
                a: a.to_string(),
                b: b.to_string(),
            },
            Comparison::Number(NumberComparison::GreaterThan { a, b }) => SerializedComparison {
                operation: ">".to_string(),
                a: a.to_string(),
                b: b.to_string(),
            },
            Comparison::Number(NumberComparison::LessThan { a, b }) => SerializedComparison {
                operation: "<".to_string(),
                a: a.to_string(),
                b: b.to_string(),
            },
            Comparison::Number(NumberComparison::GreaterThanOrEqual { a, b }) => {
                SerializedComparison {
                    operation: ">=".to_string(),
                    a: a.to_string(),
                    b: b.to_string(),
                }
            }
            Comparison::Number(NumberComparison::LessThanOrEqual { a, b }) => {
                SerializedComparison {
                    operation: "<=".to_string(),
                    a: a.to_string(),
                    b: b.to_string(),
                }
            }
            Comparison::String(StringComparison::Equals { a, b }) => SerializedComparison {
                operation: "string_equals".to_string(),
                a,
                b,
            },
            Comparison::String(StringComparison::NotEquals { a, b }) => SerializedComparison {
                operation: "string_not_equals".to_string(),
                a,
                b,
            },
            Comparison::String(StringComparison::Contains { a, b }) => SerializedComparison {
                operation: "string_contains".to_string(),
                a,
                b,
            },
            Comparison::String(StringComparison::NotContains { a, b }) => SerializedComparison {
                operation: "string_not_contains".to_string(),
                a,
                b,
            },
            Comparison::String(StringComparison::StartsWith { a, b }) => SerializedComparison {
                operation: "string_starts_with".to_string(),
                a,
                b,
            },
            Comparison::String(StringComparison::EndsWith { a, b }) => SerializedComparison {
                operation: "string_ends_with".to_string(),
                a,
                b,
            },
            Comparison::String(StringComparison::IsEmpty { a }) => SerializedComparison {
                operation: "string_is_empty".to_string(),
                a,
                b: "".to_string(),
            },
            Comparison::String(StringComparison::IsNotEmpty { a }) => SerializedComparison {
                operation: "string_is_not_empty".to_string(),
                a,
                b: "".to_string(),
            },
        }
    }

    pub fn evaluate(self) -> bool {
        match self {
            Comparison::String(StringComparison::Equals { a, b }) => a == b,
            Comparison::String(StringComparison::NotEquals { a, b }) => a != b,
            Comparison::String(StringComparison::Contains { a, b }) => a.contains(&b),
            Comparison::String(StringComparison::NotContains { a, b }) => !a.contains(&b),
            Comparison::String(StringComparison::StartsWith { a, b }) => a.starts_with(&b),
            Comparison::String(StringComparison::EndsWith { a, b }) => a.ends_with(&b),
            Comparison::String(StringComparison::IsEmpty { a }) => a.is_empty(),
            Comparison::String(StringComparison::IsNotEmpty { a }) => !a.is_empty(),
            Comparison::Number(NumberComparison::Equal { a, b }) => a == b,
            Comparison::Number(NumberComparison::NotEqual { a, b }) => a != b,
            Comparison::Number(NumberComparison::GreaterThan { a, b }) => a > b,
            Comparison::Number(NumberComparison::LessThan { a, b }) => a < b,
            Comparison::Number(NumberComparison::GreaterThanOrEqual { a, b }) => a >= b,
            Comparison::Number(NumberComparison::LessThanOrEqual { a, b }) => a <= b,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation")]
#[derive(TS)]
#[ts(export)]
pub enum NumberComparison {
    #[serde(rename = "==")]
    Equal { a: f64, b: f64 },
    #[serde(rename = "!=")]
    NotEqual { a: f64, b: f64 },
    #[serde(rename = "<")]
    LessThan { a: f64, b: f64 },
    #[serde(rename = "<=")]
    LessThanOrEqual { a: f64, b: f64 },
    #[serde(rename = ">")]
    GreaterThan { a: f64, b: f64 },
    #[serde(rename = ">=")]
    GreaterThanOrEqual { a: f64, b: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum NumberOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl NumberOperator {
    #[allow(dead_code)]
    pub fn from(str: &str) -> NumberOperator {
        match str {
            "==" => NumberOperator::Equal,
            "!=" => NumberOperator::NotEqual,
            "<" => NumberOperator::LessThan,
            "<=" => NumberOperator::LessThanOrEqual,
            ">" => NumberOperator::GreaterThan,
            ">=" => NumberOperator::GreaterThanOrEqual,
            _ => panic!("Invalid operator"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            NumberOperator::Equal => "==".to_string(),
            NumberOperator::NotEqual => "!=".to_string(),
            NumberOperator::LessThan => "<".to_string(),
            NumberOperator::LessThanOrEqual => "<=".to_string(),
            NumberOperator::GreaterThan => ">".to_string(),
            NumberOperator::GreaterThanOrEqual => ">=".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "operation")]
#[derive(TS)]
#[ts(export)]
pub enum StringComparison {
    Equals { a: String, b: String },
    NotEquals { a: String, b: String },
    Contains { a: String, b: String },
    NotContains { a: String, b: String },
    StartsWith { a: String, b: String },
    EndsWith { a: String, b: String },
    IsEmpty { a: String },
    IsNotEmpty { a: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum StringOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    IsEmpty,
    IsNotEmpty,
}

impl StringOperator {
    #[allow(dead_code)]
    pub fn from(str: &str) -> StringOperator {
        match str {
            "string_equals" => StringOperator::Equals,
            "string_not_equals" => StringOperator::NotEquals,
            "string_contains" => StringOperator::Contains,
            "string_not_contains" => StringOperator::NotContains,
            "string_starts_with" => StringOperator::StartsWith,
            "string_ends_with" => StringOperator::EndsWith,
            "string_is_empty" => StringOperator::IsEmpty,
            "string_is_not_empty" => StringOperator::IsNotEmpty,
            _ => panic!("Invalid operator"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StringOperator::Equals => "string_equals".to_string(),
            StringOperator::NotEquals => "string_not_equals".to_string(),
            StringOperator::Contains => "string_contains".to_string(),
            StringOperator::NotContains => "string_not_contains".to_string(),
            StringOperator::StartsWith => "string_starts_with".to_string(),
            StringOperator::EndsWith => "string_ends_with".to_string(),
            StringOperator::IsEmpty => "string_is_empty".to_string(),
            StringOperator::IsNotEmpty => "string_is_not_empty".to_string(),
        }
    }
}

pub struct SerializedComparison {
    pub operation: String,
    pub a: String,
    pub b: String,
}
