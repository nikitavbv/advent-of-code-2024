#!/bin/bash
set -e

. "$HOME/.cargo/env"
if ! command -v cargo 2>&1 >/dev/null
then
    . "$HOME/.cargo/env"
fi

cargo check
