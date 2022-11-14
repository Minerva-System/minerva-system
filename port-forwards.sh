#!/bin/bash

SESSION="k8sfwd"

tmux kill-session -t $SESSION
tmux new-session -d -s $SESSION

tmux split-window -v -t "$SESSION:0.0" # PostgreSQL
tmux split-window -v -t "$SESSION:0.1" # MongoDB
tmux split-window -v -t "$SESSION:0.2" # Redis
                               # 0.3 # RabbitMQ

tmux send-keys -t "$SESSION:0.0" "kubectl port-forward -n minerva deployment/postgresql-deployment 5432:5432" Enter
tmux send-keys -t "$SESSION:0.1" "kubectl port-forward -n minerva deployment/mongodb-deployment 27017:27017" Enter
tmux send-keys -t "$SESSION:0.2" "kubectl port-forward -n minerva statefulset/redis 6379:6379" Enter
tmux send-keys -t "$SESSION:0.3" "kubectl port-forward -n minerva deployment/rabbitmq-deployment 5672:5672" Enter

exec tmux attach-session -t $SESSION

