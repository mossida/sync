
[//]: # (External links)

[code-of-conduct]: CODE_OF_CONDUCT.md
[github-discussions]: https://github.com/mossida/sync/discussions
[github-issues]: https://github.com/mossida/sync/issues
[github-advisories]: https://github.com/mossida/sync/security/advisories
[rustup-toolchain]: https://rustup.rs/

# Contributing to Sync

👍🎉 First off, thanks for taking the time to contribute! 🎉👍

#### Table of contents

- [Code of Conduct](#code-of-conduct)
- [What should i know before starting?](#what-should-i-know-before-starting)
- [How to contribute](#how-to-contribute)
    - [Reporting bugs](#reporting-bugs)
    - [Suggesting features](#suggesting-features)
    - [Conventions](#conventions)
    - [Your first contribution](#your-first-contribution)
    - [Pull requests](#pull-requests)
- [Code style and guidlines](#code-style-and-guidlines)
    - [Enviroment](#enviroment)
    - [Code style](#code-style)
    - [External dependecies](#external-dependecies)
- [Additional notes](#additional-notes)
    - [Security](#security)

## Code of Conduct

Help us keep Sync open and inclusive. Please read and follow our [Code of Conduct][code-of-conduct].

## What should i know before starting?

//////

## How to contribute

### Reporting bugs

### Suggesting features

### Conventions

### Your first contribution

### Pull requests

## Code style and guidlines

### Enviroment

Before you start working on sync, ensure you have the following prerequisites installed on your system:

- <b>Rust</b>: The project is developed in Rust, so you need to have the toolchain installed. You can install it using [rustup][rustup-toolchain].
- <b>Nix (optional)</b>: We configured a Nix develop enviroment to manage dependencies and simplify development. This is not required, you can work using simply cargo.

For those who prefer using Cargo, we've set up cargo-make tasks to simplify the development workflow. To use `cargo-make`, first install it using the following command:

```sh
cargo install cargo-make
```

Once installed, you can run predefined tasks. For example, to build the project, simply run:

```sh
cargo make build
```

Check the Makefile.local.toml file for a list of available tasks.

If you prefer using your custom Cargo commands, you can still do so. Standard Cargo commands work as usual within this project (be aware that some commands require additional flags/options, we suggest using cargo-make).

### Code style

#### Using rustfmt, clippy, and audit
To maintain a high standard of code quality and consistency in the sync project, we use several tools:

<b>rustfmt</b>: This is an automatic code formatter for Rust. It ensures that all our code adheres to the style guidelines. Before submitting your code, make sure it's formatted with `rustfmt`. You can do this by running `cargo fmt`.

<b>cargo-clippy</b>: Clippy is a collection of lints to catch common mistakes and improve your Rust code. You should regularly run Clippy to catch any issues. Use the command `cargo clippy` to check your code with Clippy.

<b>cargo-audit</b>: This tool audits your Cargo.lock file for dependencies with known vulnerabilities. Keeping our dependencies secure is crucial for the integrity of the sync project. Run `cargo audit` to check for any security vulnerabilities in the dependencies.

#### Unified Check Command

To simplify the process of checking your code against these tools, we have a single command that runs rustfmt, Clippy, and cargo-audit consecutively:

```sh
cargo make check
```

This command is especially useful to run before committing your changes. It ensures that your code is not only stylistically consistent but also free from common coding mistakes and security vulnerabilities.

##### Best Practices

- Run cargo make check regularly, ideally before each commit, to catch and resolve issues early.
- If cargo make check reports any warnings or errors, address them before pushing your code.
- In addition to automated checks, always review your code manually for readability, maintainability, and adherence to Rust best practices.

By following these guidelines, you contribute to maintaining the high quality and consistency of the sync codebase. Your attention to code style and quality is greatly appreciated.

### External dependecies

## Additional notes

### Security