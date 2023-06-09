// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type Action =
  | { action: "debug" }
  | { action: "set_variable"; name: string; value: string }
  | { action: "increment_variable"; name: string; amount: string }
  | { action: "delete_variable"; name: string }
  | { action: "sleep"; duration_ms: string }
  | { action: "end_program" }
  | { action: "go_to_step"; step: string }
  | { action: "go_to_step_relative"; step: string }
  | {
    action: "if_else";
    operation: string;
    a: string;
    b: string;
    step_true: string;
    step_false: string;
  }
  | {
    action: "if_else_relative";
    operation: string;
    a: string;
    b: string;
    step_true: string;
    step_false: string;
  }
  | { action: "spawn"; command: string; args: Array<string> }
  | { action: "print_console"; content: string }
  | { action: "show_dialog"; title: string; body: string }
  | { action: "read_clipboard" }
  | { action: "write_clipboard"; content: string }
  | { action: "ask_chatgpt"; pre_prompt: string; prompt: string };
