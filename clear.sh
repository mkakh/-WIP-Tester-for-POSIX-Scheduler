#!/bin/sh

while [ -n "$(ps aux | grep -e "cargo" -e "target/debug/main" -e "target/release/main" | grep -v grep)" ]; do
  for i in $(ps aux | grep -e "cargo" -e "target/debug/main" -e "target/release/main" | grep -v grep | awk '{print $2}'); do 
    sudo kill -9 $i; 
  done
done

while [ -n "$(ps aux | grep -e "tp/" -e "g++" | grep -v grep)" ]; do
  for i in $(ps aux | grep -e "tp/" -e "g++" | grep -v grep | awk '{print $2}'); do
    sudo kill -9 $i; 
  done
done

