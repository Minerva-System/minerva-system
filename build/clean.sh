#!/bin/bash

declare -a DIRECTORIES=(
    "minerva-broker"
    "minerva-cache"
    "minerva-data"
    "minerva-dispatch"
    "minerva-product"
    "minerva-report"
    "minerva-rest"
    "minerva-rpc"
    "minerva-runonce"
    "minerva-session"
    "minerva-stock"
    "minerva-user"
)

for DIR in "${DIRECTORIES[@]}"
do
	# Backup Cargo file
	cp "./$DIR/Cargo.toml" ./Cargo.toml.bak

	if [ -f "$DIR/src/lib.rs" ]; then
		rm -rf "./$DIR"
		cargo new --lib $DIR
	else
		rm -rf "./$DIR"
		cargo new --bin $DIR
	fi

	mv ./Cargo.toml.bak "./$DIR/Cargo.toml"
done

