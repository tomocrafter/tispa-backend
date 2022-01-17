#!/bin/bash

# https://dev.to/wassimbj/deploy-your-docker-containers-with-zero-downtime-o3f

service_name=app
nginx_container_name=nginx
rust_version=$(cat rust-toolchain)

reload_nginx() {
  docker exec $nginx_container_name /usr/sbin/nginx -s reload
}

# server health check
server_status() {
  # $1 = first func arg
  local port=$1
  local status=$(curl -is --connect-timeout 5 --show-error http://localhost:$port | head -n 1 | cut -d " " -f2)

  # if status is not a status code (123), means we got an error not an http header
  # this could be a timeout message, connection refused error msg, and so on...
  if [[ $(echo ${#status}) != 3 ]]; then
    echo "503"
  fi

  echo $status
}

update_server() {
  old_container_id=$(docker ps -f name=$service_name -q | tail -n1)
  if [[ -z $old_container_id ]]; then
    echo "OLD ID NOT FOUND, QUIT !"
    exit
  fi

  docker build -t tispa/backend-builder:rust-$rust_version -f Dockerfile.builder .
  exit_status=$?
  if [ $exit_status -eq 1 ]; then
    echo "FAILED TO BUILD BUILDER IMAGE"
    exit
  fi

  # create a new instance of the server
  docker compose up --build -d --no-deps --scale $service_name=2 --no-recreate $service_name
  new_container_id=$(docker ps -f name=$service_name -q | head -n1)

  if [[ -z $new_container_id ]]; then
    echo "NEW ID NOT FOUND, QUIT !"
    exit
  fi
  new_container_port=$(docker port $new_container_id | cut -d " " -f3 | cut -d ":" -f2)

  if [[ -z $new_container_port ]]; then
    echo "PORT NOT FOUND, QUIT !"
    exit
  fi

  # sleep until server is up
  while [[ $(server_status $new_container_port) > "404" ]]; do
    echo "New instance is getting ready..."
    sleep 3
  done

  # ---- server is up ---

  # reload nginx, so it can recognize the new instance
  reload_nginx

  # remove old instance
  docker rm $old_container_id -f

  # reload ngnix, so it stops routing requests to the old instance
  reload_nginx

  echo "DONE !"
}

# call func
update_server
