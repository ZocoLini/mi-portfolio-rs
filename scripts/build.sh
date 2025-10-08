#!/bin/bash
set -e

MODE=$1

(
    cd frontend || exit 1
    if [ "$MODE" = "release" ]; then
        trunk build --release -d ../dist
    else
        trunk build -d ../dist
    fi
    
    trunk serve -d ../dist
)
