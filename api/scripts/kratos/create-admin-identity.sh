#!/bin/bash

curl --request POST -sL \
  --header "Content-Type: application/json" \
  --data '{
  "schema_id": "default",
  "traits": {
    "email": "admin@torvek.com"
  },
  "credentials": {
    "password": {
      "config": {
        "password": "password"
      }
    }
  }
}' http://127.0.0.1:4436/admin/identities
