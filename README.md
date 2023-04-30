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
          "name": "fixed_input",
          "input": "Hello world!"
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
          "name": "openai_ask_chatgpt",
          "pre_prompt": "Explain to me the following text by talking like I am a 5 years old"
        },
        {
          "name": "print_console"
        },
        {
          "name": "show_dialog",
          "title": "ChatGPT Explain"
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

The first action will receive an empty string as an input. You may want to start your list of actions with an action that read some input for the next actions.

### Debug

Print the configuration and the provided input. Returns input.

```json
{
  "name": "debug"
}
```

### Fixed Input

Provide some input directly from the configuration file. Returns input.

```json
{
  "name": "debug",
  "input": "Hello world!"
}
```

### Print Console

Print the input to the console. Returns input

```json
{
  "name": "print_console"
}
```

### Show Dialog

Show the input in a native OS dialog box. Returns input.

Parameter `title` is optional, default value is `Action Result`.

```json
{
  "name": "show_dialog",
  "title": "My Dialog Title"
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

Write the input to the clipboard. Returns input.

```json
{
  "name": "write_clipboard"
}
```

### Ask ChatGPT

Get the provided input and ask ChatGPT to answer. You can provide a pre-prompt to ask a specific action for this shortcut.

Parameter `pre_prompt` is optional, default value is no pre-prompt.

```json
{
  "name": "openai_ask_chatgpt",
  "pre_prompt": "Explain to me the following text by talking like I am a 5 years old"
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
- Run a program using the command-line and pass the input as an argument
- Star this GitHub repository (😉)

### License

[The MIT License](./LICENSE)
