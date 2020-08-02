#!/bin/sh

touch foo bar &&

cargo run -- -c tests/bin_test.conf foo bar &&

if test -r tests/oxi-rmw-Trash-test; then
  rm -rf tests/oxi-rmw-Trash-test
fi
