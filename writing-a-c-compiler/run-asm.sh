#!/bin/bash
set -e
if [ -z "$1" ]; then
    echo "Usage: run-asm <file.s> or run-asm <directory>"
    exit 1
fi

TARGET="$1"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

run_asm() {
    local ASM_FILE="$1"
    local BASENAME
    BASENAME=$(basename "${ASM_FILE%.s}")
    local BINARY="$TMP_DIR/$BASENAME"

    local COMPILE_OUTPUT
    if ! COMPILE_OUTPUT=$(gcc "$ASM_FILE" -o "$BINARY" -Wl,--no-warn-execstack 2>&1); then
        echo "❌ $ASM_FILE (compilation failed)"$'\n'"   $COMPILE_OUTPUT"
        return
    fi

    local EXIT_CODE
    "$BINARY" 2>&1 || true
    EXIT_CODE=$?

    echo "✅ $ASM_FILE (exit $EXIT_CODE)"
}

if [ -f "$TARGET" ]; then
    if [[ "$TARGET" != *.s ]]; then
        echo "Error: file '$TARGET' is not a .s file"
        exit 1
    fi
    run_asm "$TARGET"
elif [ -d "$TARGET" ]; then
    for ASM_FILE in "$TARGET"/*.s; do
        [ -f "$ASM_FILE" ] || continue
        run_asm "$ASM_FILE"
    done
else
    echo "Error: '$TARGET' is neither a file nor a directory"
    exit 1
fi