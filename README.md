# call-me-maybe

> call-me-maybe is a small CLI tool to notify you of the completion of a command

By default, the tools consumes stdin for a message's content and a title is generated for each message. The title can be 
specified via command line argument or generated automatically through a template given in configuration.

## Basic usage:

```shell
echo "Hello" | call-me-maybe
```

or (for the help message)

```shell
call-me-maybe -h
```

## Installation

### From crates.io

`cargo install call-me-maybe`


### From source

From within the source's directory: `cargo install --path .`

## Supported methods of notification

- [Pushrover](https://pushover.net/)
- [Libnotify](https://wiki.archlinux.org/title/Desktop_notifications)

## Configuration

The default configuration should be provided in either `~/.callmemaybe` or `~/.callmemaybe.toml`.

### Base configuration

The `title_fmt` key allows formatting of the title of each message sent. The format string accepts any [chrono strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
placeholder as well as `$host` for the computer's hostname. If left unspecified, the format string is `$host %a-%b-%Y`.

### Communicator basic configuration

Every communicator supports a priority number (key `priority`). The communicators are tried in order of largest priority to smallest. The command will try all communicators until one succeeds.

### Pushover

The following keys are required:
- `app_token`: the application token to select an application
- `user_token`: your user's token



