#!/bin/sh

test_result_want_fail() {
  set +x
  if [ $1 = 0 ]; then
    echo "\n  --:Test failure:--"
    exit 1
  fi

  echo "  -:Test passed:-"
  set -x
}

TEST_WASTE_FOLDER="tests/oxi-rmw-Trash-test"

if test -r ${TEST_WASTE_FOLDER}; then
  rm -rf tests/oxi-rmw-Trash-test
fi

touch foo bar || exit $?

cargo run -- -c tests/bin_test.conf foo bar || exit $?

test -e ${TEST_WASTE_FOLDER}/files/foo || exit $?
test -e ${TEST_WASTE_FOLDER}/info/foo.trashinfo || exit $?

test -e ${TEST_WASTE_FOLDER}/files/bar || exit $?
test -e ${TEST_WASTE_FOLDER}/info/bar.trashinfo || exit $?

if test -r ${TEST_WASTE_FOLDER}; then
  rm -rf ${TEST_WASTE_FOLDER}
fi

# Check for invalid attribute in the Waste file (want fail)
cargo run -- -c tests/bin_test_invalid.conf
test_result_want_fail $?

if test -r ${TEST_WASTE_FOLDER}; then
  rm -rf ${TEST_WASTE_FOLDER}
fi

exit $?
