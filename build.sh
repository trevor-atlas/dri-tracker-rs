#!/usr/bin/env bash

# Build for github pages
rm -rf ./docs && trunk clean && trunk build --release --public-url dri-tracker-rs && mv dist docs 
