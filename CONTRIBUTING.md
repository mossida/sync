
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

In the sync project, while we aim to leverage the rich ecosystem of Rust crates, we also need to be cautious about the introduction of new external dependencies. Before adding a new crate to the project, it's important to consider the following:

- <b>Necessity</b>: Evaluate if the new crate is truly necessary. Can the functionality it provides be efficiently implemented without adding the external dependency? Remember, each new crate increases the complexity and the potential security vulnerabilities of the project.

- <b>Quality and Maintenance</b>: Assess the quality of the crate. Look for crates that are well-maintained, have a good community backing, are well-documented, and have a track record of regular updates and handling of issues.

- <b>License Compatibility</b>: Ensure that the license of the crate is compatible with the sync project's license. It's crucial to maintain legal compliance and respect the intellectual property rights of others.

Before integrating a new crate:

- <b>Consult the Community and Maintainers</b>: Start a discussion with the sync community and maintainers. This can be done through GitHub issues, pull requests, or other communication channels used by the project.

- <b>Provide Justification</b>: Clearly articulate why the crate is needed, its benefits, and how it fits into the existing architecture. Include your assessment of the crate's quality, maintenance status, and license compatibility.

- <b>Seek Consensus</b>: Engage in the discussion to seek consensus. Be open to feedback and alternative suggestions from other contributors and maintainers.

- <b>Document Decisions</b>: Once a decision is made, ensure that it is documented, either in the relevant GitHub issue, pull request, or project documentation. This helps future contributors understand the rationale behind the inclusion of specific crates.

By following these guidelines, we ensure that new external dependencies are added thoughtfully and responsibly, maintaining the integrity, security, and quality of the sync project.

## Additional notes

### Security