#!/bin/bash

DAY=$(gum choose {1..3})
DATASET=$(gum choose "mini" "large")
OPTION=$(gum choose "bench" "run" "samply")

release() {
  cargo build --release --bin "$DAY"
  cp target/release/"$DAY" "./$DAY"_release
  BIN=target/release/"$DAY"
  eval ./"$BIN $DATASET"
}

debug() {
  cargo build --bin "$DAY"
  BIN=target/debug/"$DAY"
  eval ./"$BIN $DATASET"
}

bench() {
  debug
  release

  BIN_RELEASE=./target/release/"$DAY"
  BIN_DEBUG=./target/debug/"$DAY"

  mkdir -p results

  printf "\nRunning benchmark for day %s with dataset %s...\n" "$DAY" "$DATASET"
  hyperfine --warmup 3 --shell=none \
    "./$BIN_DEBUG $DATASET" "./$BIN_RELEASE $DATASET" \
    >./results/"$DAY"_results.txt 2>/dev/null
  cat ./results/"$DAY"_results.txt
}

samply_run() {
  cargo build --profile profiling --bin "$DAY"
  samply record ./target/profiling/"$DAY" "$DATASET"
}

clean() {
  rm "$DAY"_debug 2>/dev/null
  rm "$DAY"_release 2>/dev/null
}

if [[ "$OPTION" == "bench" ]]; then
  bench
elif [[ "$OPTION" == "run" ]]; then
  release
elif [[ "$OPTION" == "samply" ]]; then
  samply_run
fi

clean
