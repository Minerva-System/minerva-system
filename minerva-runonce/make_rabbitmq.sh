#!/bin/sh
docker run --name minerva-rabbitmq \
       -e RABBITMQ_DEFAULT_USER=rabbitmq \
       -e RABBITMQ_DEFAULT_PASS=minerva \
       -p 15672:15672 \
       -p 5672:5672 \
       -d rabbitmq:3-management-alpine
