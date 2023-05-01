use std::collections::HashMap;

use anyhow::Context;

use crate::actions::replace_variables_tag;
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
        let step = replace_variables_tag(step, input_str, variables)
            .parse::<usize>()
            .context("step must be a valid integer")?;

        Ok(ShortcutResult::GoToStep {
            output: input_str.to_string(),
            step,
        })
    }

    pub fn go_to_step_relative(
        input_str: &str,
        variables: &HashMap<String, String>,
        step_relative: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let step_relative = replace_variables_tag(step_relative, input_str, variables);
        let sign_is_positive = !step_relative.contains("-");
        let step_relative = step_relative.replace("-", "").replace("+", "");
        let step_relative = step_relative
            .parse::<usize>()
            .context("step must be a valid integer")?;
        Ok(ShortcutResult::GoToStepRelative {
            output: input_str.to_string(),
            step_relative,
            sign_is_positive,
        })
    }
}
