#!/bin/bash

cd crates/frontend || exit
trunk build --release

ssh root@lebastudios.org "
        rm -rf /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"

scp -r dist/* root@lebastudios.org:/var/www/lebastudios.org/portfolio

ssh root@lebastudios.org "
        chown -R www-data:www-data /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"