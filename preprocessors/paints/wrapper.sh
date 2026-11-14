#!/bin/bash
echo "DEBUG: Wrapper called with args: $@" >&2
echo "DEBUG: stdin content:" >&2
cat >&2
exec /home/sparika/workspace/mormegil/preprocessors/paints/target/debug/paint-preprocessor "$@"
