#!/bin/sh

# Used by rmwrs to determine the home directory. If this is set,
# rmwrs will use this "fake" home directory.
export RMWRS_TEST_HOME="$(dirname "$(readlink -f "$0")")/rmw_test_home"

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi

BASENAME_TEST_WASTE_FOLDER=".rmwrs-Trash-test"
TEST_WASTE_FOLDER="${RMWRS_TEST_HOME}/${BASENAME_TEST_WASTE_FOLDER}"

CFG_FILE="${CARGO_MANIFEST_DIR}/tests/bin_test.conf"
OLD_PWD="${PWD}"

test_result_want_fail() {
  set +x
  if [ $1 = 0 ]; then
    echo "\n  --:Test failure:--"
    exit 1
  fi

  echo "  -:Test passed:-"
  set -x
}

set -x

mkdir -p "${RMWRS_TEST_HOME}" || exit $?
cd "${RMWRS_TEST_HOME}" || exit $?

TEST_CMD="${CARGO_MANIFEST_DIR}/target/debug/${CARGO_PKG_NAME}"

touch foo bar || exit $?

$TEST_CMD  foo bar || exit $?

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
# ${CARGO_BUILD_TARGET_DIR}/${CARGO_PKG_NAME} -c "${CFG_FILE}" -v -z ${TEST_WASTE_FOLDER}/files/foo || exit $?
#
# It would be best if we didn't need to use the literal string "/target/debug/"
# in the command below.
$TEST_CMD -c "${CFG_FILE}" -v -z ${TEST_WASTE_FOLDER}/files/foo || exit $?
test -e foo || exit $?
test ! -e ${TEST_WASTE_FOLDER}/files/foo || exit $?
test ! -e ${TEST_WASTE_FOLDER}/info/foo.trashinfo || exit $?

# restore 'bar' for next test
$TEST_CMD -c "${CFG_FILE}" -v -z ${TEST_WASTE_FOLDER}/files/bar || exit $?

$TEST_CMD -c "${CFG_FILE}" foo bar || exit $?
set +x
echo "\n\nTest restore using absolute path"
set -x

$TEST_CMD -c "${CFG_FILE}" -z "${TEST_WASTE_FOLDER}/files/"* || exit $?
test -e foo || exit $?
test -e bar || exit $?

$TEST_CMD -c "${CFG_FILE}" foo bar || exit $?

set +x
echo "\n\ntest restore using relative path"
echo "pwd is ${PWD}\n\n"
set -x

$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/foo" || exit $?
test -e "foo" || exit $?

$TEST_CMD -c "${CFG_FILE}" foo || exit $?
# test restore using wildcard

$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/"* || exit $?
test -e "foo" || exit $?
test -e "bar" || exit $?

# filenames with spaces
touch "any bar"
$TEST_CMD -c "${CFG_FILE}" "any bar" || exit $?
# The space should have been converted to "%20"
substring="any%20bar"
output=$(cat "${BASENAME_TEST_WASTE_FOLDER}/info/any bar.trashinfo") || exit $?
test "${output#*$substring}" != "$output" || exit $?
$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/any bar" || exit $?
test -e "any bar" || exit $?

# When restoring, file should have time/date string (formatted as
# "_%H%M%S-%y%m%d") appended to it (e.g. 'foo_164353-210508').
#$TEST_CMD -c "${CFG_FILE}" foo || exit $?
#touch foo
#$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/foo" || exit $?
#ls foo_* || exit $?

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi

# Check for invalid attribute in the Waste file (want fail)
$TEST_CMD -c tests/bin_test_invalid.conf
test_result_want_fail $?

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi

exit $?
