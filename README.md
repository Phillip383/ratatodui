# Introduction
Ratodoui is a TUI Todo application built with Rust using the Ratatui Framework. Vim like keybindings for traversing the applications different widgets. There is also a command line interface to interact with todo lists and todo's.

![](https://github.com/Phillip383/ratatodui/blob/main/docs/media/ratatodui.gif)

# Using the CLI

The CLI was built using the clap crate, which greatly reduces the friction in building a comprehensive and robust interface with usage tips and required options for subcommands.
Commands exist for creating, deleting and updating lists and todo's.
The print command will print a formatted tree structure of all the lists and their todo's with completion status.

![](https://github.com/Phillip383/ratatodui/blob/main/docs/media/cli_demo.png)

# Building from Source
Rust should be the only prereq required other than cloning the source code. From the root directory run `cargo build`.
