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
    
    trunk serve -d ../dist &
    echo "TRUNK process ID: $!"
)

(
    cd backend || exit 1
    if [ "$MODE" = "release" ]; then
        cargo build --release -p backend
    else
        cargo build -p backend
    fi
)

mkdir -p dist/backend
cp target/debug/backend dist/backend/backend
dist/backend/backend &
echo "BACKEND process ID: $!"
