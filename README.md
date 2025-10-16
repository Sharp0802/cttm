# CTTM - Compile Time Template for Rust

CTTM is a simple yet powerful Rust library
that allows you to process template files at compile time.

It converts your templates into native Rust code,
eliminating runtime overhead for template parsing.

## Features

- Compile-Time Processing
- Simple Syntax
- Zero Dependencies at Runtime

## How It Works

- You create template files with a `.ct` extension.
  These files contain a mix of plain text and embedded Rust logic.
- A `build.rs` script in your project uses CTTM
  to find and compile all `.ct` files into a single Rust module.
- In your application code,
  you use the `cttm::import!()` macro to include the generated module.
- You can then call your templates as if they were regular Rust functions.

## Getting Started

### 1. Update `Cargo.toml`

```toml
[build-dependencies]
cttm = { version = "*", features = ["build"] }

[dependencies]
cttm = "*"
```

### 2. Create `build.rs`

```rust
// build.rs
use std::process::exit;

fn main() {
    // Compile all template files ending in .ct in the "templates" directory
    if let Err(e) = cttm::compile_all("templates/**/*.ct") {
        eprintln!("{}", e);
        exit(1);
    }
}
```

### 3. Write Your Templates

Create a directory for your templates (e.g., `templates/`).
A template file consists of two parts separated by `---`:

1. *Prologue*: Defines the function signature (arguments).
2. *Body*: The template content.

**Example:** `templates/greeting.ct`

```
name: &str
---
#if name == "World" {
    Hello, World!
#} else {
    Hi, ${name}!
#}
```

**Example:** `templates/star.ct`

```
n: usize
---
#for i in 1..=n {
${ "*".repeat(i) }
#}
```

### 4. Use Templates in Your Code

In your `main.rs` or any other module,
import the generated template functions and call them.

```rust
// src/main.rs
use std::io::stdout;

// Import the generated module (cttm::tpl::*)
::cttm::import!();

fn main() {
    // Call the greeting.ct template
    cttm::tpl::greeting(&mut stdout(), "Rust").unwrap();
    println!(); // for a newline

    // Call the star.ct template
    cttm::tpl::star(&mut stdout(), 5).unwrap();
}
```

The module path will be `cttm::tpl::<template_name>`,
where `<template_name>` is the filename of your template without the `.ct` extension.

## Template Syntax

- **Prologue**: The section before `---` declares the arguments for the generated function.
- **Rust Code Lines**: Any line starting with `#` is treated as a line of Rust code (e.g., for loops, if/else conditions, variable assignments).
- **Expression Interpolation**: Use `${...}` to embed the result of a Rust expression into the output.
- **Escaping**: To print a literal `#` at the beginning of a line, escape it with a backslash: `\#`. To print a literal `$`, use `\$`.

## License

This project is licensed under the MIT License.
