#!/usr/bin/env bash

set -euo pipefail

# Validate argument count
if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <day> [step] [input_file]"
    exit 1
fi

# Retrieve arguments
DAY="$1"
STEP="${2-}"
INPUT_ARG="${3-}"

# Validate day format
if ! [[ "$DAY" =~ ^[0-9]{2}$ ]]; then
    echo "Error: day must be two digits like 01, 02." >&2
    exit 1
fi

# Set paths
DAY_DIR="day-$DAY"
MANIFEST_PATH="$DAY_DIR/Cargo.toml"

# Validate day directory and manifest
if [[ ! -d "$DAY_DIR" || ! -f "$MANIFEST_PATH" ]]; then
    echo "Error: folder '$DAY_DIR' not found." >&2
    exit 1
fi

# Validate step if provided
if [[ -n "$STEP" && ! "$STEP" =~ ^[12]$ ]]; then
    echo "Error: step must be 1 or 2." >&2
    exit 1
fi

# Determine input file
INPUT_FILE=""
if [[ -n "$INPUT_ARG" ]]; then
    INPUT_FILE="$INPUT_ARG"
elif [[ -f "$DAY_DIR/assets/input.txt" ]]; then
    INPUT_FILE="$DAY_DIR/assets/input.txt"
fi

# Validate input file
if [[ -z "$INPUT_FILE" ]]; then
    echo "Error: input file not provided and '$DAY_DIR/assets/input.txt' not found." >&2
    echo "Provide an input file as the third argument." >&2
    exit 1
fi

# Build and run the day's binary
cargo build --quiet --release --manifest-path "$MANIFEST_PATH" >/dev/null 2>&1 || {
    echo "Error: Build failed for $DAY_DIR." >&2
    exit 1
}

# Determine binary path
BINARY_PATH="$DAY_DIR/target/release/day-$DAY"

# Validate binary existence
if [[ ! -x "$BINARY_PATH" ]]; then
    if [[ ! -f "$BINARY_PATH" ]]; then
        echo "Error: built binary not found at '$BINARY_PATH'." >&2
        exit 1
    fi
fi

# Run the binary with the input file
OUTPUT="$("$BINARY_PATH" "$INPUT_FILE" | cat)"

# Filter output by step if provided
if [[ -n "$STEP" ]]; then
    echo "$OUTPUT" | grep -E "^Step n°$STEP:" || {
        echo "No output lines matched 'Step n°$STEP:'" >&2
        exit 2
    }
else
    echo "$OUTPUT"
fi
