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

#
# Test restore
#
# Make sure foo doesn't exist before restoring
test ! -e foo || exit $?

# https://doc.rust-lang.org/cargo/reference/environment-variables.html
# CARGO_BUILD_TARGET_DIR was an empty variable when I tried this
# ${CARGO_BUILD_TARGET_DIR}/${CARGO_PKG_NAME} -c tests/bin_test.conf -v -z ${TEST_WASTE_FOLDER}/files/foo || exit $?
#
# It would be best if we didn't need to use the literal string "/target/debug/"
# in the command below.
${CARGO_MANIFEST_DIR}/target/debug/${CARGO_PKG_NAME} -c tests/bin_test.conf -v -z ${TEST_WASTE_FOLDER}/files/foo || exit $?
test -e foo || exit $?
test ! -e ${TEST_WASTE_FOLDER}/files/foo || exit $?
test ! -e ${TEST_WASTE_FOLDER}/info/foo.trashinfo || exit $?

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
