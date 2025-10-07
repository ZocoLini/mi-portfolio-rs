#!/bin/bash

SERVER="200.234.230.139"

trunk build --release

ssh root@$SERVER "
        rm -rf /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"

scp -r dist/* root@$SERVER:/var/www/lebastudios.org/portfolio

ssh root@$SERVER "
        chown -R www-data:www-data /var/www/lebastudios.org/portfolio
" || error_exit "SSH commands failed"