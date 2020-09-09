#!/bin/sh

# Used by rmwrs to determine the home directory. If this is set,
# rmwrs will use this "fake" home directory.
export RMWRS_TEST_HOME="$(dirname "$(readlink -f "$0")")/rmw_test_home"

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi

test_result_want_fail() {
  set +x
  if [ $1 = 0 ]; then
    echo "\n  --:Test failure:--"
    exit 1
  fi

  echo "  -:Test passed:-"
  set -x
}

TEST_WASTE_FOLDER="${RMWRS_TEST_HOME}/.rmwrs-Trash-test"

touch foo bar || exit $?

cargo run -- -c tests/bin_test.conf foo bar || exit $?

test -e ${TEST_WASTE_FOLDER}/files/foo || exit $?
test -e ${TEST_WASTE_FOLDER}/info/foo.trashinfo || exit $?

test -e ${TEST_WASTE_FOLDER}/files/bar || exit $?
test -e ${TEST_WASTE_FOLDER}/info/bar.trashinfo || exit $?

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi


# Check for invalid attribute in the Waste file (want fail)
cargo run -- -c tests/bin_test_invalid.conf
test_result_want_fail $?

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi

exit $?
