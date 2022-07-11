<h1 align="center">Ignore</h1>

<p align="center">
    <img width="200" src="assets/logo.png">
</p>

<h3 align="center">A Gitignore Generator</h3>

---

# Example

```
ignore --template default:Rust global:Linux > .gitignore
``` 

> see `ignore list` to show all available templates

# Custom templates

By default, templates are taken from the [this repository](https://github.com/github/gitignore)

To add custom template just create `[name].gitignore`
file inside `ignore where` directory.

Files inside folders will be prefixed with the parent folder name (except for root templates).

For example, if you have `custom.gitignore` file inside `extras/` folder,
you can use it like this:

```
ignore --template extras:custom
```

| Location                       | Name            |
|--------------------------------|-----------------|
| `custom.gitignore`             | `custom`        |
| `extras/custom.gitignore`      | `extras:custom` |
| `extras/misc/custom.gitignore` | `misc:custom`   |

# Help

```
USAGE:
    ignore [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help                      Print help information
    -t, --template <template>...    The templates to ignore
    -V, --version                   Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    list      List all available templates
    update    Update the default templates database
    where     Print the templates path
```