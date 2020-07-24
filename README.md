# oxi-rmw

[![Build
Status](https://travis-ci.com/theimpossibleastronaut/oxi-rmw.svg?branch=trunk)](https://travis-ci.com/theimpossibleastronaut/oxi-rmw)

**Not ready for use**

[rust](https://www.rust-lang.org/) port of [rmw](https://remove-to-waste.info/)

## Current state

oxi-rmw is in a very early development state and may change rapidly.
Presently files can be removed to a WASTE directory but there's no
`restore` feature.

Running `cargo run -- <file>` will ReMove a file to a waste folder
that's hard-coded in `main.rs`, and create a corresponding `.trashinfo`
file into a `waste_info` directory (see source code for details).
Because there's no restore feature yet, if you'd like to demo or test the
program, you should only use files as arguments that are not essential.
For example:

    touch foo bar
    cargo run -- foo bar

Example 2:

    mkdir tmp
    touch tmp/foo tmp/bar
    cargo run -- tmp/*

You'll see that the files were removed to ~/.oxi-rmw-Trash-test/files
and corresponding .trashinfo files were created in
~/.oxi-rmw-Trash-test/info. The .trashinfo file uses the
[FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-latest.html).

To learn more about the design goals for this program, see the
[rmw](https://remove-to-waste.info/) website. The goal of this rust
port is to mimic the features of the C version.
