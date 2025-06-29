#!/bin/bash

cd crates/frontend || exit
trunk build --release

ssh root@nginx.master.lebastudios.lan "
        rm -rf /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"

scp -r dist/* root@nginx.master.lebastudios.lan:/var/www/lebastudios.org/portfolio

ssh root@nginx.master.lebastudios.lan "
        chown -R www-data:www-data /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"