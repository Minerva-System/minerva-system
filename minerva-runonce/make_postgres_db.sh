#!/bin/sh
docker run --name minerva-postgres \
       -e POSTGRES_USER=postgres \
       -e POSTGRES_PASSWORD=postgres \
       -p 5432:5432 \
       -d postgres:15-alpine
