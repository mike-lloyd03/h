# h

## What is this?

`h` is a consistent way to get help for Linux commands. When looking for command help, `man` is the typical go-to. But sometimes, you're looking for help for a subcommand or there is no man page available. In those cases, the `--help` flag is often used. Then, to make it scrollable, you might pipe the output to `less`. Now you're typing:

```bash
cargo build --help | less
```

The aim of this project is to simplify this process.

```bash
h cargo build
```

## Configuration

A configuration file can be created at `$XDG_CONFIG_HOME/h/config.toml` (on Linux, this will typically be `~/.config/h/config.toml`).

### Pager

An alternate pager can be used.

```toml
pager = "/usr/bin/nvimpager"
```

### Wrappers

Some programs do not use a `--help` flag and instead have a `help` subcommand or some other alternative (e.g. Go). In those cases, a wrapper can be used which will prepend the search term with the necessary subcommand.

Adding this to `config.toml`:

```toml
[[wrappers]]
cmd = "go (.*)"
replacement = "go help {1}"
use_pager = true
```

Will run:

```bash
go help build | $PAGER
```

#### Syntax

- `cmd`: Is a regex which accepts capture groups (anything inside parenthesis)
- `replacement`: A string to replace the search arguments. A number inside curly braces will be replaced by the corresponding capture group. Capture groups are numbered starting with 1.
- `use_pager`: Pipe the output of the resulting command to the `$PAGER`.
