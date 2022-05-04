#!/bin/sh
docker run --name minerva-micro \
       -e POSTGRES_USER=postgres \
       -e POSTGRES_PASSWORD=postgres \
       -e POSTGRES_DB=minerva \
       -p 5432:5432 \
       -d postgres:14
