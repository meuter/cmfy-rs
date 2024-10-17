# cmfy

[![Build](https://github.com/meuter/cmfy-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/meuter/cmfy-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cmfy)](https://crates.io/crates/cmfy)
[![Docs.rs](https://docs.rs/cmfy/badge.svg)](https://docs.rs/cmfy)
[![Crates.io](https://img.shields.io/crates/d/cmfy)](https://crates.io/crates/cmfy)
[![Crates.io](https://img.shields.io/crates/l/cmfy)](https://github.com/meuter/cmfy/blob/main/LICENSE)

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
  view     Open images from completed prompts in a browser
  help     Print this message or the help of the given subcommand(s)

Options:
  -s, --server <SERVER>  hostname of the server [env: COMFY_SERVER=] [default: localhost]
  -p, --port <PORT>      port of the server [env: COMFY_PORT=] [default: 8188]
  -h, --help             Print help
  -V, --version          Print version
```

## Installation

### From crates.io

- Install rust as per [these instructions](https://www.rust-lang.org/tools/install)
- Install `cmfy`:
  ```
  cargo install cmfy
  ```

### From source

- Install rust as per [these instructions](https://www.rust-lang.org/tools/install)
- Clone the repo:
  ```
  git clone https://github.com/meuter/cmfy
  ```
- Install `cmfy`:
  ```
  cd cmfy
  cargo install --path .
  ```

### From Github Release

- Download the prebuilt binaries from the [release](https://github.com/meuter/cmfy-rs/releases) page.



