## Code of Conduct

Help us keep Sync open and inclusive. Please read and follow our Code of Conduct.

## Getting started

To setup a working develop enviroment we configured some options to facilitate your contributions. We prepared a Nix develop enviroment with everything needed and custom commands to simplify your development and contributions. If you already use
nix (please make sure flakes are enabled) just run `nix develop` in the directory to start the enviroment (first time will require some minutes to install and the dependecies). Additionally we configurated a devcontainer so you can start working on codespaces (we enabled direnv, so there is no need to run the command).

If you prefer to go with manually install just be sure to have Rust toolchain (with rustfmt) installed and cargo-make installed. Then you can run commands with `cargo make <command>`. See the complete list of commands [here](https://github.com/mossida/sync/blob/main/Makefile.local.toml).

## Coding standards

Please make sure your code is properly formatted with rustfmt. We also use clippy to be sure that common coding mistakes are prevented. To check if everything is ok please run cargo make check or check (if in nix enviroment) before every commit.