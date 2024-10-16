# cmfy

A CLI companion app for Comfy UI

## Usage

```
Usage: cmfy [OPTIONS] <COMMAND>

Commands:
  stats    Displays basic statistics about the server
  history  Lists and optionally clears prompts from history
  queue    Lists and optionally clears prompts from queue
  list     List all prompts from history and queue
  cancel   Cancel currently running prompt
  clear    Clear all prompts from history, queue and currently running prompt
  open     Open ComfyUI in a web browser
  capture  Capture running and pending prompt to file
  get      Display GET request raw json output
  submit   Submits a batch of prompts to the server
  help     Print this message or the help of the given subcommand(s)

Options:
  -s, --server <SERVER>  hostname of the server [env: COMFY_SERVER=] [default: localhost]
  -p, --port <PORT>      port of the server [env: COMFY_PORT=] [default: 8188]
  -h, --help             Print help
  -V, --version          Print version
```

