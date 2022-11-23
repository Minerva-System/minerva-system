#!/bin/bash

trap 'kill %1; kill %2; kill %3; kill %4' SIGINT

kubectl port-forward -n minerva deployment/postgresql-deployment 5432:5432  &
kubectl port-forward -n minerva deployment/mongodb-deployment 27017:27017   &
kubectl port-forward -n minerva statefulset/redis 6379:6379                 &
kubectl port-forward -n minerva deployment/rabbitmq-deployment 5672:5672    &
kubectl port-forward -n minerva deployment/rabbitmq-deployment 15672:15672

