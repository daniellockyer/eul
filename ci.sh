#!/bin/bash

set -eu
shopt -s nullglob

cargo build --release
cargo test

for fn in data/*; do
    EXACT_COUNT=`gunzip -c $fn | tr ' ' '\n' | sort -u | wc -l`
    CARD_EST=`gunzip -c $fn | ./target/release/eul`
    DIFF=`echo "100 - (100 * $CARD_EST/$EXACT_COUNT)" | bc`
    DIFF_FLOAT=`echo "scale=3;100 - (100 * $CARD_EST/$EXACT_COUNT)" | bc -l`

    echo "file: $fn, exact: $EXACT_COUNT, estimate: $CARD_EST"
    printf "ratio: %.3f%%\n\n" $DIFF_FLOAT

    if [ "$DIFF" -lt -3 ] || [ "$DIFF" -gt 3 ]
    then
        echo "Failure with $FN exact=$EXACT_COUNT estimate=$CARD_EST (diff=$DIFF%)"
        exit 1
    fi
done

for fn in data/*; do
    echo "-------- file: $fn"
    hyperfine "gunzip -c $fn | tr ' ' '\n' | sort -u | wc -l" "gunzip -c $fn | ./target/release/eul"
    printf "\n"
done
