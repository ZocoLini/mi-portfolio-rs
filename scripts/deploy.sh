#!/bin/bash
set -euo pipefail

SERVER="ubuntu@15.204.8.56"

bash scripts/build.sh release

# Deploying the frontend
(
  ssh $SERVER "
        rm -rf /tmp/portfolio
    " || exit "SSH commands failed"

  rsync -avz dist/ $SERVER:/tmp/dist

  ssh $SERVER "
        rm -rf /home/ubuntu/www/portfolio/dist && mv /tmp/dist /home/ubuntu/www/portfolio/dist
    " || exit "SSH commands failed"
)

# Deploying the backend
(
  scp target/release/backend $SERVER:/home/ubuntu/www/portfolio/backend
)

# Restart the containers
(
  ssh $SERVER "
        cd /home/ubuntu/www/portfolio/ && bash scripts/restart-portfolio.sh
    " || exit "SSH commands failed"
)
