#!/bin/bash

json_file="tx.json"

# Define variables for the address and port
address="127.0.0.1"
port="3699"

# Build the curl command with proper quoting
curl_cmd="curl -X POST -H \"Content-Type: application/json\" -d @$json_file http://$address:$port"

# Execute the curl command
eval "$curl_cmd"
