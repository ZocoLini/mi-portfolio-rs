#!/bin/bash
set -e

MODE=$1

mkdir -p dist

(
    cd frontend || exit 1
    if [ "$MODE" = "release" ]; then
        trunk build --release -d ../dist
    else
        trunk build -d ../dist
    fi
)

(
    cd backend || exit 1
    if [ "$MODE" = "release" ]; then
        cargo build --release -p backend
    else
        cargo build -p backend
    fi
)

if [ "$MODE" = "release" ]; then
    exit 0
fi

cd frontend || exit 1
trunk serve -d ../dist &
pid1=$!
cd ..

target/debug/backend &
pid2=$!

cleanup() {
  echo "Stopping both processes..."
  kill $pid1 $pid2 2>/dev/null
  wait $pid1 $pid2 2>/dev/null
  echo "All processes stopped."
  exit 0
}

trap cleanup SIGINT
wait