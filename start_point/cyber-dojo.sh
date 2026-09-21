set -e

# --------------------------------------------------------------
# Text files under /sandbox are automatically returned...
source ~/cyber_dojo_fs_cleaners.sh
export REPORT_DIR=${CYBER_DOJO_SANDBOX}/report
function cyber_dojo_enter()
{
  # 1. Only return _newly_ generated reports.
  cyber_dojo_reset_dirs ${REPORT_DIR}
}
function cyber_dojo_exit()
{
  # 2. Remove text files we don't want returned.
  cyber_dojo_delete_dirs target # ...
  cyber_dojo_delete_files Cargo.lock  # ...
}
cyber_dojo_enter
trap cyber_dojo_exit EXIT SIGTERM

# ------------------------------------------------------------------------
PATH=/usr/local/cargo/bin:${PATH}
#RUST_BACKTRACE=1

rm -rf report || true
mkdir report 2> /dev/null || true

# clippy reads your code and the tests compile it, so neither needs the
# other to have finished and they run at once. Two cargos sharing ./target
# would take turns on its lock, so clippy is given a target dir of its own,
# outside the sandbox so that nothing it builds comes back as a file.
# cargo fmt stays where it is: it rewrites your source files in place, and
# doing that while a build is reading them would be a race.
readonly HELD=$(mktemp -d)
CARGO_TARGET_DIR=${HELD}/clippy-target cargo clippy \
  > ${HELD}/clippy 2> ${HELD}/clippy.err &
readonly CLIPPY_PID=$!

cargo test --features "strict" -- --nocapture

# Reached only when the tests passed, because set -e stops the run above
# otherwise, and clippy's report must not appear for a run that stopped
# there. Its output is held until here for that reason, rather than being
# appended to the report as it is produced.
wait ${CLIPPY_PID} || CLIPPY_STATUS=$?
cat ${HELD}/clippy
cat ${HELD}/clippy.err >> ./report/style.txt
if [ -n "${CLIPPY_STATUS:-}" ]; then
  exit ${CLIPPY_STATUS}
fi

cargo fmt
