# Shortcut Hero

Bind keyboard shortcuts to any actions

## Releases

See [releases](/rigwild/apidoc-markdown/releases) for pre-built binaries.

For debian or ubuntu based distributions, you need the following dependencies.

```sh
# Required for https://github.com/obv-mikhail/InputBot#build-dependencies
sudo apt install libx11-dev libxtst-dev libudev-dev libinput-dev
```

## Build from sources

### Windows

```sh
git clone git@github.com:rigwild/shortcut-hero.git
cd shortcut-hero
cargo build --release

# If you have UPX, to compress the executable
upx --best --lzma -o target/release/shortcut-hero.upx.exe target/release/shortcut-hero.exe
```

### Debian or Ubuntu based distributions

```sh
# Required for https://github.com/obv-mikhail/InputBot#build-dependencies
sudo apt install libx11-dev libxtst-dev libudev-dev libinput-dev

git clone git@github.com:rigwild/shortcut-hero.git
cd shortcut-hero
cargo build --release

# If you have UPX, to compress the executable
upx --best --lzma -o target/release/shortcut-hero.upx.bin target/release/shortcut-hero
```

## Configuration

Create a file named `shortcut-hero.json` in your current working directory, it will be created if not found.

Here is a [configuration file example](./shortcut-hero.example.json).

```json
{
  "openai_api_key": "sk-...",
  "keyboard_shortcuts": [
    {
      "keys": ["DKey"],
      "actions": [
        {
          "action": "set_variable",
          "name": "user",
          "value": "rigwild"
        },
        {
          "action": "set_variable",
          "name": "city",
          "value": "Bordeaux"
        },
        {
          "action": "debug"
        }
      ]
    },
    {
      "keys": ["LControlKey", "BKey"],
      "actions": [
        {
          "action": "read_clipboard"
        },
        {
          "action": "debug"
        },
        {
          "action": "show_dialog",
          "title": "Hello World!",
          "body": "{{input}}"
        }
      ]
    }
  ]
}
```

### `openai_api_key`

An [OpenAI API key](https://platform.openai.com/account/api-keys), only required if you use the [Ask ChatGPT](#ask-chatgpt) action.

### `keyboard_shortcuts`

#### `keyboard_shortcuts.keys`

List of keyboard keys to all be pressed at the same time to trigger the associated
actions. [List of available keys](https://github.com/rigwild/shortcut-hero/blob/f462afe44c1751fb49dd021fa8427c74ffe7ee47/src/hotkey.rs#L99-L225).

#### `keyboard_shortcuts.actions`

List of [actions](#actions) to run when triggering this shortcut.

## Actions

Actions are synchronous functions that take some input and return some output, they can do anything.

The actions will run in the order they are defined. The result of each action is provided to the next action as an input (variable `input`).

If an action requires any parameter, you can use variables enclosed in tags `{{input}}` or `{{my_variable}}`, they will be replaced everywhere with the associated value.

Variables names are case-insensitive.

The first action in the list will receive an empty string as an input. You may want to start your list of actions with an action that read some data as input for the next actions.

### Debug

Print the configuration, the provided input and the list of variables. Returns input.

```json
{
  "action": "debug"
}
```

### Set Variable

Set the value of a variable. Do not use tags like `{{input}}` or `{{my_variable}}` for the name of the variable, use the variable name directly `input` or `my_variable`. Otherwise, value can contain tags like `{{input}}` or `{{my_variable}}`.

Returns input.

Set the variable `city` to `Bordeaux`.

```json
{
  "action": "set_variable",
  "name": "city",
  "value": "Bordeaux"
}
```

Set the variable `input` to `Hello, I am rigwild! How are you? Bordeaux is a great place to live!`.\
With `input = rigwild`, `city = Bordeaux`

```json
{
  "action": "set_variable",
  "name": "input",
  "value": "Hello, I am {{input}}! How are you? {{city}} is a great place to live!"
}
```

### Delete Variable

Delete a variable. Returns input (if the deleted variable is `input`, returns nothing).

```json
{
  "action": "delete_variable",
  "name": "my_variable"
}
```

### Sleep

Wait for a given duration. Returns input.

- Parameter `duration_ms` must be a string containing a valid positive integer, in milliseconds.

Wait for 1 second.

```json
{
  "action": "sleep",
  "duration_ms": "1000"
}
```

Wait for 5 seconds.\
With `wait_time = 5000`

```json
{
  "action": "sleep",
  "duration_ms": "{{wait_time}}"
}
```

### End Program

End the program.

```json
{
  "action": "end_program"
}
```

### Go To Step

Go to a given step in the list of actions (starts at 0). Returns input.

Will error out if the step is out of bounds.

- Parameter `step` must be a string containing a valid positive integer.

Go to step 0.

```json
{
  "action": "go_to_step",
  "step": "0"
}
```

Go to step 5.\
With `my_step = 5`

```json
{
  "action": "go_to_step",
  "step": "{{my_step}}"
}
```

### Go To Step Relative

Go to a given step in the list of actions relative from the current step. Returns input.

Will error out if the step is out of bounds.

- Parameter `step` must be a string containing a valid positive integer.

Go 2 steps ahead relative from the current step.

```json
{
  "action": "go_to_step_relative",
  "step_relative": "2"
}
```

Go 1 step behind relative from the current step.

```json
{
  "action": "go_to_step_relative",
  "step_relative": "-1"
}
```

Go 5 steps behind relative from the current step.\
With `my_step = -5`

```json
{
  "action": "go_to_step_relative",
  "step_relative": "{{my_step}}"
}
```

### Print Console

Print the input to the console. Returns input.

- Parameter `content` is optional, default value is `{{input}}`.

Print `Hello world!` to the console.

```json
{
  "action": "print_console",
  "content": "Hello world!"
}
```

Print `Hello world!` to the console.\
With `input = Hello world!`

```json
{
  "action": "print_console"
}
```

Print `Hello world! I am rigwild! I live in Bordeaux` to the console.\
With `input = rigwild`, `city = Bordeaux`

```json
{
  "action": "print_console",
  "content": "Hello world! I am {{input}}! I live in {{city}}"
}
```

### Show Dialog

Show the input in a native OS dialog box. Returns input.

- Parameter `title` is optional, default value is `Action Result`.
- Parameter `body` is optional, default value is `{{input}}`.

Show a dialog with title `Action Result` and body `Hello World!`.\
With `input = Hello world!`

```json
{
  "action": "show_dialog"
}
```

Show a dialog with title `My Dialog Title` and body `Hello world!`.\
With `input = Hello world!`

```json
{
  "action": "show_dialog",
  "title": "My Dialog Title"
}
```

Show a dialog with title `Result of operation "12 * 11"` and body `Result from action is: 132`.\
With `input = 132`, `math_equation = 12 * 11`

```json
{
  "action": "show_dialog",
  "title": "Result of operation \"{{math_equation}}\"",
  "body": "Result from action is: {{input}}"
}
```

### Read Clipboard

Read the content of the clipboard. Returns content of the clipboard.

```json
{
  "action": "read_clipboard"
}
```

### Write Clipboard

Write to the clipboard. Returns input.

- Parameter `content` is optional, default value is `{{input}}`.

Write `Hello world!` to the clipboard.

```json
{
  "action": "write_clipboard",
  "content": "Hello world!"
}
```

Write `Hello world!` to the clipboard.\
With `input = Hello world!`

```json
{
  "action": "write_clipboard"
}
```

Write `The quick brown fox jumps over the lazy dog` to the clipboard.\
With `animal = dog`.

```json
{
  "action": "write_clipboard",
  "content": "The quick brown fox jumps over the lazy {{animal}}"
}
```

### Spawn

Spawn a system command. Returns the result of the command.

- Parameter `args` is optional, default value is empty list.

Evaluate a JavaScript program with Node.js.\
With `input = console.log('Hello world!')`.

```json
{
  "action": "spawn",
  "command": "/usr/bin/node",
  "args": ["-e", "{{input}}"]
}
```

```sh
/usr/bin/node -e "console.log('Hello world!')"
```

Find the files that are bigger than 1 MB in a directory.\
With `input = ~/`.

```json
{
  "action": "spawn",
  "command": "find",
  "args": ["find", "{{input}}", "-type", "f", "-size", "+1M", "-exec", "ls", "-lh", "{}", "\\;"]
}
```

```sh
find ~/ -type f -size +1M -exec ls -lh {} \;
```

Execute an arbitrary command (dangerous).\
With `command = rm`, `path = /some/example`.

```json
{
  "action": "spawn",
  "command": "{{command}}",
  "args": ["-rf", "{{path}}"]
}
```

```sh
rm -rf /some/example
```

### Ask ChatGPT

Ask something to ChatGPT. Returns the answer from ChatGPT.

- Parameter `pre_prompt` is optional, default value is no pre-prompt.
- Parameter `prompt` is optional, default value is `{{input}}`.

Ask ChatGPT to answer.\
With `input = Who are you?!`.

```json
{
  "action": "ask_chatgpt"
}
```

Ask ChatGPT to answer `Who are you?!` with fixed pre-prompt `Explain to me the following text by talking like I am a 5 years old`.\
With `input = Who are you?!`.

```json
{
  "action": "ask_chatgpt",
  "pre_prompt": "Explain to me the following text by talking like I am a 5 years old",
  "prompt": "{{input}}"
}
```

Ask ChatGPT to play a theater game with dynamic participants and pre-prompt.\
With:

- `character_assistant = rigwild`
- `character_me = Louna`
- `character_me_sentence = Can you buy me some peaches too?`

```json
{
  "action": "openai_ask_chatgpt",
  "pre_prompt": "You are playing a theater game where you are a character in a made-up story. You are in a scene, you are called {{character_assistant}}. You say: \"I am going to the store to buy some apples.\"",
  "prompt": "- {{character_me}}: \"Hey {{character_assistant}}! {{character_me_sentence}}\""
}
```

## Create your own actions

You can create your own actions by forking this repository and adding your own actions in the [actions](./src/actions) directory.
You will need to add your action in the [`Action` enum](./src/actions/mod.rs) to make it available for a shortcut.

**Note:** Your action's name will be converted to and from `snake_case` for the configuration file.

Be creative! ✌️

### Ideas of actions

- Read the current OS time
- Get the current weather from some API
- Get the price of an item on Amazon by scrapping the website
- Get the current price of a cryptocurrency
- Send a message on Telegram using a bot
- Post in a Discord channel using a bot
- Open a URL in the default browser
- Read the content of a file
- Write the input to a file
- Star this GitHub repository (😉)

### License

[The MIT License](./LICENSE)
