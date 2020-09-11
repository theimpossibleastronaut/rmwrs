# rmwrs

[![Build
Status](https://travis-ci.com/theimpossibleastronaut/rmwrs.svg?branch=trunk)](https://travis-ci.com/theimpossibleastronaut/rmwrs)

**Not ready for use**

[rust](https://www.rust-lang.org/) port of [rmw](https://remove-to-waste.info/)

## Current state

*rmwrs* is in a very early development state and may change rapidly.
Presently files can be removed to a WASTE directory but there's no
`restore` feature.

Running `cargo run -- <file>` will ReMove a file to a waste folder
(which is specified in the test configuration file) and create a
corresponding `.trashinfo` file into a `waste_parent_info` directory (see
source code for details). Because there's no restore feature yet, if
you'd like to demo or test the program, you should only use files as
arguments that are not essential. For example:

    touch foo bar
    cargo run -- foo bar

Example 2:

    mkdir tmp
    touch tmp/foo tmp/bar
    cargo run -- tmp/*

You'll see that the files were removed to ~/.rmwrs-Trash-test/files
and corresponding .trashinfo files were created in
~/.rmwrs-Trash-test/info. The .trashinfo file uses the
[FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-latest.html).

If you use *rmwrs* on a file that has the same name in a destination
waste folder, it will be renamed by adding a time string formatted
suffix (i.e. "_%H%M%S-%y%m%d").

## Configuration file

*rmwrs* will automatically create a configuration file (rmwrsrc) in
$XDG_CONFIG_HOME (if the environmental variable is set) or `~/.config`.

You can specify an alternate/custom configuration file.

    cargo run -- tmp/* -c <custom-config-file>

## Most Recent List (mrl) file

The names of files that you "remove" will be stored in an mrl file in
$XDG_DATA_HOME/rmwrs (if the environmental variable is set) or
~/.local/share/rmwrs. Each time you run it, the file gets overwritten
with the new filenames.

The mrl file isn't used for anything yet. Later it will be used by the
restore and undo function.

## More Info

To learn more about the design goals for this program, see the
[rmw](https://remove-to-waste.info/) website. The goal of this rust
port is to mimic the features of the C version.
