#!/bin/sh

export CPATH="/opt/homebrew/Cellar/z3/4.12.4/include:$CPATH"
export LIBRARY_PATH="/opt/homebrew/Cellar/z3/4.12.4/lib:$LIBRARY_PATH"

cargo run

