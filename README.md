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
          "name": "set_input",
          "content": "Hello World!"
        },
        {
          "name": "debug"
        }
      ]
    },
    {
      "keys": ["LControlKey", "BKey"],
      "actions": [
        {
          "name": "read_clipboard"
        },
        {
          "name": "debug"
        },
        {
          "name": "show_dialog",
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

The actions will run in the order that they are defined. The result of each action is provided to the next action as an input.

If an action requires any parameter, the tag `{{input}}` will be replaced everywhere by the provided input.

The first action in the list will receive an empty string as an input. You may want to start your list of actions with an action that read some input for the next actions.

### Debug

Print the configuration and the provided input. Returns input.

```json
{
  "name": "debug"
}
```

### Clear Input

Remove the input, useful if the next action does not require an input. Returns empty string.

```json
{
  "name": "clear_input"
}
```

### Set Input

Set the input. Returns the new input.

Input before: `anything`\
Input after: `Hello world!`

```json
{
  "name": "set_input",
  "content": "Hello world!"
}
```

Input before: `rigwild`\
Input after: `Hello, I am rigwild! How are you?`

```json
{
  "name": "set_input",
  "content": "Hello, I am {{input}}! How are you?"
}
```

### Print Console

Print the input to the console. Returns original input.

- Parameter `content` is optional, default value is `{{input}}`.

Print `{{input}}` to the console.

```json
{
  "name": "print_console"
}
```

Print `Hello world!` to the console.

```json
{
  "name": "print_console",
  "content": "Hello world!"
}
```

Print `Hello world! I am {{input}}!` to the console.

```json
{
  "name": "print_console",
  "content": "Hello world! I am {{input}}!"
}
```

### Show Dialog

Show the input in a native OS dialog box. Returns original input.

- Parameter `title` is optional, default value is `Action Result`.
- Parameter `body` is optional, default value is `{{input}}`.

Show a dialog with title `Action Result` and body `{{input}}`.

```json
{
  "name": "show_dialog"
}
```

Show a dialog with title `My Dialog Title` and body `{{input}}`.

```json
{
  "name": "show_dialog",
  "title": "My Dialog Title"
}
```

Show a dialog with title `My Dialog Title` and body `Result from action is {{input}}`.

```json
{
  "name": "show_dialog",
  "title": "My Dialog Title",
  "body": "Result from action is {{input}}"
}
```

### Read Clipboard

Read the content of the clipboard. Returns content of the clipboard.

```json
{
  "name": "read_clipboard"
}
```

### Write Clipboard

Write the input to the clipboard. Returns original input.

- Parameter `content` is optional, default value is `{{input}}`.

Write the input `{{input}}` to the clipboard.

```json
{
  "name": "write_clipboard"
}
```

Write `Hello world!` to the clipboard.

```json
{
  "name": "write_clipboard",
  "content": "Hello world!"
}
```

Write `Hello {{input}}!` to the clipboard.

```json
{
  "name": "write_clipboard",
  "content": "Hello {{input}}!"
}
```

### Spawn

Spawn a system command. Returns the result of the command.

- Parameter `args` is optional, default value is empty list.

Pass the input as a script for Node.js to execute.\
Input is `console.log('Hello world!')`.

```json
{
  "name": "spawn",
  "command": "/usr/bin/node",
  "args": ["-e", "{{input}}"]
}
```

```sh
/usr/bin/node -e "console.log('Hello world!')"
```

Find the files that are bigger than 1 MB in a directory.\
Input is `~/`.

```json
{
  "name": "spawn",
  "command": "find",
  "args": ["find", "{{input}}", "-type", "f", "-size", "+1M", "-exec", "ls", "-lh", "{}", "\\;"]
}
```

```sh
find ~/ -type f -size +1M -exec ls -lh {} \;
```

Execute an arbitrary command (dangerous).\
Input is `rm -rf /some/example`.

```json
{
  "name": "spawn",
  "command": "{{input}}",
  "args": ["-rf", "/some/example"]
}
```

```sh
rm -rf /some/example -rf /some/example
```

### Ask ChatGPT

Get the provided input and ask ChatGPT to answer. You can provide a pre-prompt to ask a specific action for this shortcut.

- Parameter `pre_prompt` is optional, default value is no pre-prompt.
- Parameter `prompt` is optional, default value is `{{input}}`.

Ask ChatGPT to answer `{{input}}`.

```json
{
  "name": "ask_chatgpt"
}
```

Ask ChatGPT to answer `{{input}}` with pre-prompt `Explain to me the following text by talking like I am a 5 years old`.

```json
{
  "name": "ask_chatgpt",
  "pre_prompt": "Explain to me the following text by talking like I am a 5 years old",
  "prompt": "{{input}}"
}
```

Ask ChatGPT to play a theater game with the provided input.

```json
{
  "name": "openai_ask_chatgpt",
  "pre_prompt": "You are playing a theater game where you are a character in a made-up story. You are in a scene, you are called rigwild. You say: \"I am going to the store to buy some apples.\"",
  "prompt": "- Louna: \"Hello rigwild, {{input}}\""
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
