# cmfy

[![Build](https://github.com/meuter/cmfy-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/meuter/cmfy-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cmfy)](https://crates.io/crates/cmfy)
[![Docs.rs](https://docs.rs/cmfy/badge.svg)](https://docs.rs/cmfy)
[![Crates.io](https://img.shields.io/crates/d/cmfy)](https://crates.io/crates/cmfy)
[![Crates.io](https://img.shields.io/crates/l/cmfy)](https://github.com/meuter/cmfy/blob/main/LICENSE)

A CLI companion app for Comfy UI

## Installation

<details>
<summary>crates.io</summary>

- Install rust as per [these instructions](https://www.rust-lang.org/tools/install)
- Install `cmfy-cli`:
  ```
  cargo install cmfy-cli
  ```

</details>

<details>
<summary>git</summary>

- Install rust as per [these instructions](https://www.rust-lang.org/tools/install)
- Install `cmfy`:
  ```
  cd cmfy
  cargo install --git https://github.com/meuter/cmfy-rs
  ```

</details>

<details>
<summary>precompiled binaries</summary>

- Download the prebuilt binaries from the [release](https://github.com/meuter/cmfy-rs/releases/latest) page.
- Save it somewhere in your path (don't format to `chmod +x` on Linux or MacOS)

</details>

## Usage

`cmfy` provides several commands:

```txt
Usage: cmfy [OPTIONS] <COMMAND>

Commands:
  stats     Displays basic statistics about client and server
  history   Manipulates history of prompts
  queue     Manipulates the queue of pending prompts
  list      List all prompts from history and queue
  cancel    Cancel currently running prompt
  clear     Clear all prompts from history, queue and currently running prompt
  open      Open ComfyUI in a web browser
  capture   Capture running and pending prompt to file
  submit    Submits a batch of prompts to the server
  view      Open images from completed prompts in a browser
  get       Display GET request raw json output
  listen    Opens a websocket connection to the server, listens for messages and displays them as JSON on the console
  extract   Extracts prompt information from a PNG generated with Comfy UI, and outputs it as JSON
  monitor   Monitors the progress on ongoing prompts
  download  Download images from completed prompts locally
  help      Print this message or the help of the given subcommand(s)

Options:
  -s, --hostname <HOSTNAME>    hostname of the server [env: COMFY_HOSTNAME=172.24.80.1] [default: localhost]
  -p, --port <PORT>            port of the server [env: COMFY_PORT=] [default: 8188]
  -c, --client-id <CLIENT_ID>  client id advertised to the server, allows to spoof an existing client when listening to websocket message. If none is provided, a unique id will be computed [env:
 COMFY_CLIENT_ID=]
  -h, --help                   Print help
  -V, --version                Print version
```

