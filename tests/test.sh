#!/bin/sh

# echo commands, exit on an error
set -ev

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

mkdir -p "${RMWRS_TEST_HOME}"
cd "${RMWRS_TEST_HOME}"

TEST_CMD="${CARGO_MANIFEST_DIR}/target/debug/${CARGO_PKG_NAME}"

touch foo bar

$TEST_CMD  foo bar

test -e ${TEST_WASTE_FOLDER}/files/foo
test -e ${TEST_WASTE_FOLDER}/info/foo.trashinfo

test -e ${TEST_WASTE_FOLDER}/files/bar
test -e ${TEST_WASTE_FOLDER}/info/bar.trashinfo

#
# Test restore
#
# Make sure foo doesn't exist before restoring
test ! -e foo

# https://doc.rust-lang.org/cargo/reference/environment-variables.html
# CARGO_BUILD_TARGET_DIR was an empty variable when I tried this
# ${CARGO_BUILD_TARGET_DIR}/${CARGO_PKG_NAME} -c "${CFG_FILE}" -v -z ${TEST_WASTE_FOLDER}/files/foo
#
# It would be best if we didn't need to use the literal string "/target/debug/"
# in the command below.
$TEST_CMD -c "${CFG_FILE}" -v -z ${TEST_WASTE_FOLDER}/files/foo
test -e foo
test ! -e ${TEST_WASTE_FOLDER}/files/foo
test ! -e ${TEST_WASTE_FOLDER}/info/foo.trashinfo

# restore 'bar' for next test
$TEST_CMD -c "${CFG_FILE}" -v -z ${TEST_WASTE_FOLDER}/files/bar

$TEST_CMD -c "${CFG_FILE}" foo bar
echo -e "\n\nTest restore using absolute path"

$TEST_CMD -c "${CFG_FILE}" -z "${TEST_WASTE_FOLDER}/files/"*
test -e foo
test -e bar

$TEST_CMD -c "${CFG_FILE}" foo bar

echo -e "\n\ntest restore using relative path"
echo -e "pwd is ${PWD}\n\n"

$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/foo"
test -e "foo"

$TEST_CMD -c "${CFG_FILE}" foo
# test restore using wildcard

$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/"*
test -e "foo"
test -e "bar"

# filenames with spaces
touch "any bar"
$TEST_CMD -c "${CFG_FILE}" "any bar"
# The space should have been converted to "%20"
substring="any%20bar"
output=$(cat "${BASENAME_TEST_WASTE_FOLDER}/info/any bar.trashinfo")
test "${output#*$substring}" != "$output"
$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/any bar"
test -e "any bar"

# When restoring, file should have time/date string (formatted as
# "_%H%M%S-%y%m%d") appended to it (e.g. 'foo_164353-210508').
$TEST_CMD -c "${CFG_FILE}" foo
touch foo
$TEST_CMD -c "${CFG_FILE}" -z "${BASENAME_TEST_WASTE_FOLDER}/files/foo"
ls foo_*

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi

# Check for invalid attribute in the Waste file (want fail)
$TEST_CMD -c tests/bin_test_invalid.conf && exit 1

if test -r ${RMWRS_TEST_HOME}; then
  rm -rf ${RMWRS_TEST_HOME}
fi

exit $?
