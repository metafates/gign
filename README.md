<h1 align="center">Ignore</h1>

<p align="center">
    <img width="200" src="assets/logo.png">
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
ignore default:Rust global:Linux > .gitignore

# if you use fish shell you can do this cool trick
ignore global:{Linux, macOS, Windows} > .gitignore

# ignore all javascript related templates
ignore $(ignore list javascript) default:Node > .gitignore

# search for template with fzf and use it
ignore $(ignore list | fzf) > .gitignore

# make it less verbose
ignore --auto c++ windows > .gitignore
``` 

> see `ignore list` to show all available templates

## Install

Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```
cargo install --git https://github.com/metafates/ignore
```

## Custom templates

By default, templates are taken from [this repository](https://github.com/github/gitignore)

To add custom template just create `[name].gitignore`
file inside `ignore where` directory.

Files inside folders will be prefixed with the parent folder name (except for the root templates).

For example, if you have `custom.gitignore` file inside `extras/` folder,
you can use it like this:

```
ignore extras:custom
```

| Location                       | Name            |
|--------------------------------|-----------------|
| `custom.gitignore`             | `custom`        |
| `extras/custom.gitignore`      | `extras:custom` |
| `extras/misc/custom.gitignore` | `misc:custom`   |

## Help

```
USAGE:
    ignore [OPTIONS] [template]... [SUBCOMMAND]

ARGS:
    <template>...    The templates to ignore

OPTIONS:
    -a, --auto       Automatically resolve unknown templates
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    list      List all available templates
    update    Update the default templates database
    where     Print the templates path
```
