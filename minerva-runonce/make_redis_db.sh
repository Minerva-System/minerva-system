#!/bin/sh
docker run --name minerva-redis \
       -p 6379:6379 \
       -d redis:7-alpine
