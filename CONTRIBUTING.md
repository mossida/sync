
[//]: # (External links)

[code-of-conduct]: CODE_OF_CONDUCT.md
[github-discussions]: https://github.com/mossida/sync/discussions
[github-issues]: https://github.com/mossida/sync/issues
[github-advisories]: https://github.com/mossida/sync/security/advisories
[rustup-toolchain]: https://rustup.rs/
[conventional-commits]: https://www.conventionalcommits.org/en/v1.0.0/

# Contributing to Sync

Thank you so much for taking the time to contribute to our project! 🎉

We genuinely appreciate your interest and efforts in making a contribution. Your willingness to share your ideas and collaborate with us plays a vital role in the growth and success of our project. Contributions from community members like you bring new perspectives, innovations, and enhancements that are invaluable.

Every contribution, whether big or small, is a significant step towards achieving our collective goals. We're excited about your participation and are eager to see the positive impact of your work. Your involvement is not just beneficial to the project but also an opportunity for us all to learn and grow together.

Once again, thank you for your contribution! We're thrilled to have you as part of our community and look forward to the wonderful things we'll accomplish together.

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
    - [Git commit messages](#git-commit-messages)
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

We are excited for you to contribute to Sync, and we aim to ensure that Sync remains a high-quality project that stays true to its vision and mission. To strike the right balance, we ask that you first open a question on [GitHub discussions][github-discussions] for any ideas you have before creating a GitHub Issue. This approach allows the Sync community to engage in meaningful discussions about the value of the new feature and its alignment with the product's roadmap and vision before a new pull request is introduced.

Engaging in GitHub discussions is also crucial for the Sync lead developers to provide technical input and perspectives on feature design and architecture. This collaborative process helps in shaping features that are well-integrated and beneficial to the project's overall goals. By following this approach, your contributions can be more effectively aligned with the needs and direction of Sync, ensuring that your efforts have the greatest possible impact.

### Conventions

### Your first contribution

### Pull requests

## Code style and guidlines

### Git commit messages

In the sync project, we adhere strictly to the [Conventional Commits][conventional-commits] specification for structuring our commit messages. This standardized format for commit messages ensures clarity and consistency across the project's history. To enforce this, we use `commitlint`, a tool that checks if commit messages meet the Conventional Commits format.

Recommendations for writing commit messages:

- <b>Use conventional commits plugins</b>: To simplify writing commit messages, we recommend using Conventional Commits plugins available for various IDEs. These plugins can help format your commit messages correctly, reducing the chance of errors.

- <b>CI checks for commit messages</b>: Our Continuous Integration (CI) pipeline includes a check that validates all commit messages in a pull request (PR) against the Conventional Commits format.

- <b>Impact on code reviews</b>: PRs with commit messages that fail the CI check will be temporarily skipped for code review (will be marked invalid). We believe that well-written commit messages are essential for efficient code reviews and for making the codebase easier to understand and navigate. Therefore, ensuring your commits adhere to the Conventional Commits format is crucial for receiving timely reviews.

Writing effective commit messages:

- <b>Follow the format</b>: A conventional commit message should typically follow this format: `<type>[optional scope]: <description>`. For example, `feat(database): add new indexing feature`.

- <b>Types</b>: Common types include feat (new feature), fix (bug fix), docs (documentation changes), style (code style changes), refactor (refactoring code), test (adding or modifying tests), and chore (maintenance tasks).

- <b>Scope</b>: Optionally, include a scope to specify the part of the codebase your commit affects.

- <b>Description</b>: The description should be concise yet descriptive, explaining what the commit does and why, if not obvious (you can also add emojies).

By adhering to these guidelines and utilizing tools like Conventional Commits plugins and `commitlint`, you contribute to the maintainability and readability of the sync project. Your effort in writing clear and structured commit messages is greatly appreciated.

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