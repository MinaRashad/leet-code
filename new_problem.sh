#!/usr/bin/env bash

read -p "Paste the URL : " URL
problem=$(echo $URL| tr '/' '\n' )

# get the element before the last
problem=$(echo $problem | awk '{print $(NF-1)}')

mkdir $problem

touch $problem/$problem.rs
