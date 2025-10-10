#!/bin/bash

SERVER="root@200.234.230.139"

bash scripts/build.sh release

# Deploying the frontend
(
    ssh $SERVER "
        rm -rf /tmp/portfolio
    " || error_exit "SSH commands failed"

    scp -r dist/* $SERVER:/tmp/portfolio
    
    ssh $SERVER "
        rm -rf /var/www/bcastellano.com/portfolio && mv /tmp/portfolio /var/www/bcastellano.com/portfolio
    " || error_exit "SSH commands failed"

    ssh $SERVER "
            chown -R www-data:www-data /var/www/bcastellano.com/portfolio
    " || error_exit "SSH commands failed"
)

# Deploying the backend
(
    ssh $SERVER "
        systemctl stop portfolio-backend.service
    " || error_exit "SSH commands failed"

    scp target/release/backend $SERVER:/srv/portfolio-backend

    ssh $SERVER "
        systemctl start portfolio-backend.service
    " || error_exit "SSH commands failed"
)
