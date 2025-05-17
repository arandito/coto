# coto

[![License](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/github/v/release/arandito/coto)](https://github.com/arandito/coto/releases)
[![CI](https://github.com/arandito/coto/actions/workflows/build.yml/badge.svg)](https://github.com/arandito/coto/actions)

A CLI tool to generate AWS boto3 code snippets using the OpenAI API and written in Rust.

> **Status:** This is version **0.1.0**—still in development. Major (1.x) releases are forthcoming.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
  - [Interactive Setup](#interactive-setup)
  - [Manual Configuration](#manual-configuration)
  - [`coto config`](#coto-config)
- [Commands](#commands)
  - [`coto setup`](#coto-setup)
  - [`coto config`](#coto-config)
  - [`coto gen`](#coto-gen)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Features

- Generate Python boto3 scripts for AWS operations from natural language prompts.
- Configure default OpenAI model, AWS region, and AWS profile.
- Interactive and manual configuration methods.
- Flexible output: print to stdout or write to files.

## Prerequisites

- A Unix-like shell (Linux, macOS) or PowerShell (Windows)
- A valid OpenAI API key
- AWS credentials configured

## Installation

Install `coto` with our standalone installers:

```bash
# On macOS and Linux.
curl -LsSf https://antoara.com/coto/install.sh | sh
```

Windows standalone installer will be added in a future version. You can access our Windows binaries [here](https://github.com/arandito/coto/releases).

## Configuration

### Interactive Setup

Run the setup command to initialize your configuration file at `$XDG_CONFIG_HOME/coto/config.toml` (e.g., `~/.config/coto/config.toml`):

```bash
coto setup
```

You will be prompted to enter:

- OpenAI API key
- Default OpenAI model (e.g., `gpt-4o-mini`)
- AWS region (e.g., `us-west-2`)
- AWS CLI profile (e.g., `default`)

### Manual Configuration

Create or edit the config file directly:

```toml
# ~/.config/coto/config.toml

api_key = "your-openai-api-key"
model   = "gpt-4o-mini"
region  = "us-west-2"
profile = "default"
```

### `coto config`

Manage individual settings in your config file without opening it manually. You can:

```bash
# Show current configuration
coto config show

# Set a value
coto config set model gpt-4

# Unset a value
coto config unset profile
```

## Commands

### `coto setup`

Interactively configure your OpenAI key, default model, AWS region, and profile.

```bash
coto setup
```

### `coto config`

Manage individual settings in your config file (alias for config commands shown above).

```bash
# Show settings
coto config show

# Set or unset values
coto config set <key> <value>
```

### `coto gen`

Generate a boto3 code snippet from a natural language prompt.

```bash
coto gen [OPTIONS]
```

**Options:**

- `-p, --prompt <PROMPT>` Natural language description of desired AWS operation. _(required)_
- `-r, --region <REGION>` AWS region to use (overrides config).
- `-P, --profile <PROFILE>` AWS CLI profile to use (overrides config).
- `-m, --model <MODEL>` OpenAI model to use (overrides config).
- `-o, --output <FILE>` Write code snippet to file instead of printing.
- `-d, --dry-run` Show request details without calling the API.

## Examples

```bash
# Print a snippet to stdout
coto gen -p "list all S3 buckets in us-west-2"

# Write snippet to a file
coto gen -p "upload the file ~/profile.png to the coto-test S3 bucket" -o upload.py

# Override defaults
coto gen -p "delete the EC2 instance named coto-vm" --model gpt-4.1 --region us-east-1 --profile myprofile
```

## Contributing

Contributions, issues, and feature requests are welcome!
Feel free to check [issues](https://github.com/arandito/coto/issues) or submit a pull request.

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.
