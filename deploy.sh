#!/bin/bash

cd crates/frontend || exit
trunk build --release

ssh root@91.116.216.159 "
        rm -rf /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"

scp -r dist/* root@91.116.216.159:/var/www/lebastudios.org/portfolio

ssh root@91.116.216.159 "
        chown -R www-data:www-data /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"