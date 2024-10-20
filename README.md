# Carplexity

A physics-based vehicular sports game.

# Building

## With Nix 🙂

* `nix develop`

* `cargo run --release`

## Without Nix 🙁

Idk, if you get this building on other platforms, please file a PR to update this README file with instructions.

# Testing

* `cargo test`

# CLI

Run the game executable with subcommands to perform other behavior than just launching the game client. E.g. `carplexity server --port 1234` or `cargo run --release -- server --port 1234` if building from source.

Use `--help` or `-h` on any command to get a list of possible subcommands or information about a specific subcommand.

# Configuration

The game client directory defaults to `~/.config/carplexity` on Linux, `%APPDATA%\carplexity` on Windows, and `Library/Application Support/carplexity` on MacOS. The game client will automatically create this directory if needed. This directory can be set with the `--dir` or `-d` flag.

The game client will look for the configuration file `carplexity.toml` in this directory, or create it if needed.

# Running a server

When the `server` subcommand is run, a server will be started in the same directory as the executable. To change the server data directory, use the `--dir` or `-d` flag.

The server will look for a `carplexity_server.toml` file and a `carplexity_server.sqlite` file in this directory. If they do not exist, you will be prompted to create them. You can also run `carplexity server init` to 

# Running an identity server

If you wish to run a separate identity server to the default, you can use the `ident-server` subcommand similarly to the `server` subcommand. This uses the files `carplexity_ident_server.toml` and `carplexity_ident_server.sqlite`.

# Licensing

This codebase is unlicensed for now and therefore not open source. I would like to open source it in the future but I'm not sure yet about which license to choose.

If you would like to submit a contribution anyway, create an issue or PR and we can discuss it.
