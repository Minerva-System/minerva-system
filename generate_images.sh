#!/bin/bash
declare -a TARGETS=(
    "minerva_rest"
    "minerva_runonce"
    "minerva_session"
    "minerva_users"
)

# Rust language targets
for TARGET in "${TARGETS[@]}"
do
    docker image build -f build/Dockerfile \
	   --target $TARGET \
	   -t luksamuk/${TARGET}:latest .
done

# FrontEnd target
docker image build -f build/Dockerfile.frontend \
       -t luksamuk/minerva_frontend:latest \
       .

# PgAdmin
docker image build -f build/Dockerfile.pgadmin \
       -t luksamuk/minerva_pgadmin:latest \
       build
