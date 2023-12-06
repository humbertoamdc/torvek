#!/bin/bash

env $(cat env/prod.env | xargs) trunk build --release;
