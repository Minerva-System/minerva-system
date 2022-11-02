#!/bin/bash

docker image push -a luksamuk/minerva_frontend
docker image push -a luksamuk/minerva_rest
docker image push -a luksamuk/minerva_runonce
docker image push -a luksamuk/minerva_user
docker image push -a luksamuk/minerva_session
docker image push -a luksamuk/minerva_dispatch
docker image push -a luksamuk/minerva_pgadmin
