#!/bin/bash

mkdir -p ~/db/mongodb/data
mkdor -p ~/db//mongodb/config
docker compose up -d --build --force-recreate --remove-orphans