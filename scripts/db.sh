if [ ! -f portfolio.sqlite ]; then
    touch portfolio.sqlite
fi

cargo sqlx prepare --workspace
sqlx migrate run