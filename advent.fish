#!/usr/bin/env fish

function advent -a year -a day
    # All in one launcher
    export RUST_LOG=debug
    cargo run --bin advent -- $year $day
    set --local advent_bin (string join _ advent $year $day)
    cargo watch --clear --exec "check --quiet" --exec "test --bin $advent_bin" --exec "run --quiet --bin $advent_bin"
end

advent $argv