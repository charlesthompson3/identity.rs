#!/usr/bin/env bash
set -e

# Remove stale coverage report
rm -rf coverage
mkdir coverage

NIGHTLY="+nightly-2022-01-08"

# Run tests with profiling instrumentation
echo "Running instrumented unit tests..."
RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="identity-%m.profraw" cargo $NIGHTLY test --tests --all --all-features

# Merge all .profraw files into "identity.profdata"
echo "Merging coverage data..."
PROFRAW=""
for file in $(find . -type f -name "*.profraw");
do
  echo "Found $file"
  PROFRAW="${PROFRAW} $file"
done

cargo $NIGHTLY profdata -- merge ${PROFRAW} -o identity.profdata

# List the test binaries
echo "Locating test binaries..."
BINARIES=""

for file in \
  $( \
    RUSTFLAGS="-Zinstrument-coverage" \
      cargo $NIGHTLY test --tests --all --all-features --no-run --message-format=json \
        | jq -r "select(.profile.test == true) | .filenames[]" \
        | grep -v dSYM - \
  ); \
do
  echo "Found $file"
  BINARIES="${BINARIES} -object $file"
done

# Generate and export the coverage report to lcov format
echo "Generating lcov file..."
cargo $NIGHTLY cov -- export ${BINARIES} \
  --instr-profile=identity.profdata \
  --ignore-filename-regex="/.cargo|rustc|target|tests|/.rustup" \
  --format=lcov --Xdemangler=rustfilt \
  >> coverage/coverage.info


# Ensure intermediate coverage files are deleted
echo "Removing intermediate files..."
find . -name "*.profraw" -type f -delete
find . -name "*.profdata" -type f -delete
