# Template Filler

## Overview

Do you use [Handlebars][handlebars] templates for some documents?

`Template Filler` is a graphical application to interactively fill all the
variables in a template file and generate the final document. Its usage is
simple:

- 📂 Open a Handlebars template.
- ✏️ Set values for all the variables found in template.
- 💾 Render template with newly set values to a new file.

## Installation
### Installing from source
#### Installing dependencies

`Template Filler` is written in [Rust][rust] and uses the [GTK][gtk] toolkit:

- A Rust toolchain can easily be installed using [rustup][rustup].
- GTK development libraries can be installed via the package manager of the
  GNU/Linux distribution used to build (e.g. on Fedora: `sudo dnf install
  gtk4-devel`).

#### Building from source

To build from the source code, execute:

```sh
cargo build --release
```

The resulting binary executable file will be ``target/release/template-filler``.

[gtk]: https://gtk-rs.org
[handlebars]: https://handlebarsjs.com/
[rust]: https://rust-lang.org
[rustup]: https://rustup.rs
