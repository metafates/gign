<h1 align="center">Gign</h1>

<p align="center">
    <img width="200" src="https://github.com/metafates/gign/blob/main/assets/logo.png">
</p>

<h3 align="center">
    A Gitignore Generator
</h3>

## Table of Contents

- [Examples](#examples)
- [Install](#install)
- [Custom templates](#custom-templates)
- [Help](#help)

## Examples

```bash
# This is how you going to use it probably most of the time
gign rust linux -a # -a short for --append

# It's smart!
# This will expand to python, jetbrains and linux
gign pyton jjetbrainz linus > .gitignore

# Append to the repository root-level .gitignore automatically
gign --append haskell

# Do not automatically resolve unknown templates
gign --append --strict macos
# error: template 'macos' not found, did you mean 'global:macOS'?

# Ignore all javascript related templates
gign $(gign list javascript) node > .gitignore

# Search for template with fzf and use it
gign $(gign list | fzf) > .gitignore
``` 

> see `gign list` to show all available templates

## Install

Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```
cargo install gign
```

## Custom templates

By default, templates are taken from https://github.com/github/gitignore

To add custom template just create `[name].gitignore`
file inside `gign where` directory.

Files inside folders will be prefixed with the parent folder name (except for the root templates).

For example, if you have `custom.gitignore` file inside `extras/` folder,
you can use it like this:

```bash
gign --strict extras:custom

# or
gign extras:custom

# or
gign custom
```

| Location                       | Strict Name     |
|--------------------------------|-----------------|
| `custom.gitignore`             | `custom`        |
| `extras/custom.gitignore`      | `extras:custom` |
| `extras/misc/custom.gitignore` | `misc:custom`   |

## Help

```
USAGE:
    gign [OPTIONS] [template]... [SUBCOMMAND]

ARGS:
    <template>...    The templates to ignore

OPTIONS:
    -a, --append     Append to the root-level .gitignore file
    -h, --help       Print help information
    -s, --strict     Do not automatically resolve unknown templates
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    list      List all available templates
    update    Update the default templates database
    where     Print the templates path
```
