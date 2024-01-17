# DRAFT

## Code of Conduct

Help us keep Sync open and inclusive. Please read and follow our Code of Conduct.

## Getting started

To setup a working develop enviroment we configured some options to facilitate your contributions. We prepared a Nix develop enviroment with everything needed and custom commands to simplify your development and contributions. If you already use
nix (please make sure flakes are enabled) just run `nix develop` in the directory to start the enviroment (first time will require some minutes to install and the dependecies). Additionally we configurated a devcontainer so you can start working on codespaces (we enabled direnv, so there is no need to run the command).

If you prefer to go with manually install just be sure to have Rust toolchain (with rustfmt) installed and cargo-make installed. Then you can run commands with `cargo make <command>`. See the complete list of commands [here](https://github.com/mossida/sync/blob/main/Makefile.local.toml).

## Coding standards

Please make sure your code is properly formatted with `rustfmt`. We also use clippy to be sure that common coding mistakes are prevented. To check if everything is ok please run `cargo make check` or check command (if in nix enviroment) before every commit.

## Submitting a Pull Request

You should start with a correct branch name:

type/issueid-description

If your don't have the issue id you directly write the description (Remember that a issue is preferred).

Type can be one of the conventional commits standards. So for example

feat/5459-add-dynamic-fetching
fix/2342-replace-handler-from-actor

### Commits

We strictly follow the convetional commits standard (there is a check in CI process) to ensure that all pushed commits respect a common standard.

For every commit please provide also a body to explain the reason of the changes you made (helps to speed up PRs and to have a clean commit history).

Use the commit history in your favour: Small and self-contained commits allow the reviewer to see exactly how you solved the problem. By reading the commit history of the PR, the reviewer can already understand what they'll be reviewing, even before seeing a single line of code.

### Create a PR

The title of your pull request should be clear and descriptive (please follow convetional commits). It should summarize the changes you are making in a concise manner.

Provide a detailed description of the changes you are making. Explain the reasoning behind the changes, the problem it solves, and the impact it may have on the codebase. Keep in mind that a reviewer was not working on your task, so you should explain why you wrote the code the way you did.

Describe the scene and provide everything that will help to understand the background and a context for the reviewers by adding related GitHub issues to the description, and links to the related PRs, projects or third-party documentation. If there are any potential drawbacks or trade-offs to your changes, be sure to mention them too.