#!/bin/sh
docker run --name minerva-mongo \
       -e MONGO_INITDB_ROOT_USERNAME=root \
       -e MONGO_INITDB_ROOT_PASSWORD=mongo \
       -p 27017:27017 \
       -d mongo:6
