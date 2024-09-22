#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Orders \
    --attribute-definitions \
        AttributeName=customer_id,AttributeType=S \
        AttributeName=status#id,AttributeType=S \
        AttributeName=is_open,AttributeType=S \
    --key-schema \
        AttributeName=customer_id,KeyType=HASH \
        AttributeName=status#id,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST \
    --global-secondary-indexes \
    '[
      {
        "IndexName": "OpenOrders",
        "KeySchema": [
          {"AttributeName":"is_open","KeyType":"HASH"}
        ],
        "Projection":{
          "ProjectionType": "ALL"
        }
      }
    ]'
