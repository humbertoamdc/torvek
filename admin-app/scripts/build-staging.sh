#!/bin/bash

env $(cat env/staging.env | xargs) trunk build --release;
