if [ ! -f portfolio.sqlite ]; then
    touch portfolio.sqlite
fi

sqlx migrate run
cargo sqlx prepare --workspace