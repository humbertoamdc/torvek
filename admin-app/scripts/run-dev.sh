#!/bin/bash

env $(cat env/dev.env | xargs) trunk serve --port 8081;
