use std::collections::HashMap;
use std::process::Command;

use anyhow::{anyhow, Context};

use crate::actions::{replace_variables_tag, replace_variables_tag_vec};
use crate::hotkey::ShortcutResult;
use crate::Config;

pub struct CoreAction;

impl CoreAction {
    pub fn debug(
        config: &Config,
        input_str: &str,
        variables: &HashMap<String, String>,
    ) -> anyhow::Result<ShortcutResult> {
        print!("\n\n");
        println!("###########");
        println!("## DEBUG ##");
        println!("###########");
        print!("\nconfig={config:#?}\n\ninput_str={input_str:#?}\nvariables={variables:#?}");
        print!("\n\n###########\n\n");
        Ok(ShortcutResult::Success(input_str.to_string()))
    }

    pub fn set_variable(
        input_str: &str,
        variables: &mut HashMap<String, String>,
        name: &str,
        value: &str,
    ) -> anyhow::Result<ShortcutResult> {
        if name.to_lowercase() == "input" {
            Ok(ShortcutResult::Success(value.to_string()))
        } else {
            variables.insert(
                name.to_lowercase().to_string(),
                replace_variables_tag(value, input_str, variables),
            );
            Ok(ShortcutResult::Success(input_str.to_string()))
        }
    }

    pub fn delete_variable(
        input_str: &str,
        variables: &mut HashMap<String, String>,
        name: &str,
    ) -> anyhow::Result<ShortcutResult> {
        if name.to_lowercase() == "input" {
            Ok(ShortcutResult::Success("".to_string()))
        } else {
            variables.remove(name.to_lowercase().as_str());
            Ok(ShortcutResult::Success(input_str.to_string()))
        }
    }

    pub fn end_program(input_str: &str) -> anyhow::Result<ShortcutResult> {
        Ok(ShortcutResult::EndProgram(input_str.to_string()))
    }

    pub fn go_to_step(
        input_str: &str,
        variables: &HashMap<String, String>,
        step: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let step = replace_variables_tag(step, input_str, variables);
        let step = parse_step(&step)?;

        Ok(ShortcutResult::GoToStep {
            output: input_str.to_string(),
            step,
        })
    }

    pub fn go_to_step_relative(
        input_str: &str,
        variables: &HashMap<String, String>,
        step: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let step = replace_variables_tag(step, input_str, variables);
        let (step, sign_is_positive) = parse_step_relative(&step)?;
        Ok(ShortcutResult::GoToStepRelative {
            output: input_str.to_string(),
            step,
            sign_is_positive,
        })
    }

    pub fn if_else(
        input_str: &str,
        variables: &HashMap<String, String>,
        operation: &str,
        a: &str,
        b: &str,
        step_true: &str,
        step_false: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let operation = replace_variables_tag(operation, input_str, variables);
        let a = replace_variables_tag(a, input_str, variables);
        let b = replace_variables_tag(b, input_str, variables);
        let step_true = replace_variables_tag(step_true, input_str, variables);
        let step_true = parse_step(&step_true)?;
        let step_false = replace_variables_tag(step_false, input_str, variables);
        let step_false = parse_step(&step_false)?;

        let step = if evaluate_condition(&operation, &a, &b)? {
            step_true
        } else {
            step_false
        };
        Ok(ShortcutResult::GoToStep {
            output: input_str.to_string(),
            step,
        })
    }

    pub fn if_else_relative(
        input_str: &str,
        variables: &HashMap<String, String>,
        operation: &str,
        a: &str,
        b: &str,
        step_true: &str,
        step_false: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let operation = replace_variables_tag(operation, input_str, variables);
        let a = replace_variables_tag(a, input_str, variables);
        let b = replace_variables_tag(b, input_str, variables);
        let step_true = replace_variables_tag(step_true, input_str, variables);
        let (step_true, step_true_sign_is_positive) = parse_step_relative(&step_true)?;
        let step_false = replace_variables_tag(step_false, input_str, variables);
        let (step_false, step_false_sign_is_positive) = parse_step_relative(&step_false)?;

        let (step, sign_is_positive) = if evaluate_condition(&operation, &a, &b)? {
            (step_true, step_true_sign_is_positive)
        } else {
            (step_false, step_false_sign_is_positive)
        };
        Ok(ShortcutResult::GoToStepRelative {
            output: input_str.to_string(),
            step,
            sign_is_positive,
        })
    }

    pub fn spawn(
        input_str: &str,
        variables: &HashMap<String, String>,
        command: &str,
        args: &Vec<String>,
    ) -> anyhow::Result<ShortcutResult> {
        let command = replace_variables_tag(command, input_str, variables);
        let args = replace_variables_tag_vec(args, input_str, variables);

        let mut command = Command::new(command);
        args.iter().for_each(|arg| {
            command.arg(arg);
        });
        let command = command.output().expect("Failed to execute command");
        if command.status.success() {
            Ok(ShortcutResult::Success(
                String::from_utf8_lossy(&command.stdout).to_string(),
            ))
        } else {
            Err(anyhow!(
                "Command failed: {}",
                String::from_utf8_lossy(&command.stderr)
            ))
        }
    }

    pub fn increment_variable(
        input_str: &str,
        variables: &mut HashMap<String, String>,
        name: &str,
        amount: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let amount: i64 = replace_variables_tag(amount.to_string().as_str(), input_str, variables)
            .parse::<i64>()
            .unwrap();

        let new_value = if variables.contains_key(name.to_lowercase().as_str()) {
            let value = variables
                .get(name.to_lowercase().as_str())
                .unwrap()
                .parse::<i64>()
                .unwrap();
            (value + amount).to_string()
        } else {
            amount.to_string()
        };
        variables.insert(name.to_string(), new_value);

        Ok(ShortcutResult::Success(input_str.to_string()))
    }
}

fn parse_step(step: &str) -> anyhow::Result<usize> {
    Ok(step
        .parse::<usize>()
        .context("step must be a valid integer")?)
}

fn parse_step_relative(step: &str) -> anyhow::Result<(usize, bool)> {
    let sign_is_positive = !step.contains("-");
    let step = step
        .replace("-", "")
        .replace("+", "")
        .parse::<usize>()
        .context("step must be a valid integer")?;
    Ok((step, sign_is_positive))
}

fn evaluate_condition(operation: &str, a: &str, b: &str) -> anyhow::Result<bool> {
    // String operations
    match operation {
        "str_equals" => return Ok(a == b),
        "str_not_equals" => return Ok(a != b),
        "str_contains" => return Ok(a.contains(b)),
        "str_not_contains" => return Ok(!a.contains(b)),
        "str_starts_with" => return Ok(a.starts_with(b)),
        "str_ends_with" => return Ok(a.ends_with(b)),
        "str_is_empty" => return Ok(a.is_empty()),
        "str_is_not_empty" => return Ok(!a.is_empty()),
        _ => {}
    }

    // Float operations
    let a: f64 = a.parse::<f64>().unwrap();
    let b: f64 = b.parse::<f64>().unwrap();
    match operation {
        "==" => Ok(a == b),
        "!=" => Ok(a != b),
        ">" => Ok(a > b),
        "<" => Ok(a < b),
        ">=" => Ok(a >= b),
        "<=" => Ok(a <= b),
        _ => Err(anyhow::anyhow!("Unknown operation: {}", operation)),
    }
}
