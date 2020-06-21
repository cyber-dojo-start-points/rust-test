set -e

# ------------------------------------------------------------------------
# cyber-dojo returns text files under /sandbox that are
# created/deleted/changed. In tidy_up you can remove any
# such files you don't want returned to the browser.

trap tidy_up EXIT

function tidy_up()
{
  delete_dirs target
  delete_files Cargo.lock
}

function delete_dirs()
{
  for dirname in "$@"
  do
      rm -rf "${dirname}" 2> /dev/null || true
  done
}

function delete_files()
{
  for filename in "$@"
  do
      rm "${filename}" 2> /dev/null || true
  done
}

# ------------------------------------------------------------------------
PATH=/usr/local/cargo/bin:${PATH}
#RUST_BACKTRACE=1

rm -rf report || true
mkdir report 2> /dev/null || true
cargo test --features "strict" -- --nocapture
cargo clippy 2>> ./report/style.txt
cargo fmt
