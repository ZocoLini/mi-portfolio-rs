#!/bin/bash

cd crates/frontend || exit
trunk build --release
scp -r dist/* root@lebastudios.org:/var/www/lebastudios.org/portfolio

