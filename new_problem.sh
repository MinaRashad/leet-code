#!/usr/bin/env bash

read -p "Paste the URL : " URL

# the problem name will be after problems/
# and the name ends at the next /
problem=$(echo $URL | grep -oP 'problems/\K[^/]*')

cargo new $problem
