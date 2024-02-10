#!/bin/bash

# S3 Buckets
awslocal s3 mb s3://unnamed-client-files

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Orders \
    --attribute-definitions \
        AttributeName=part_id,AttributeType=S \
        AttributeName=order_status,AttributeType=S \
        AttributeName=created_at,AttributeType=S \
    --key-schema \
        AttributeName=part_id,KeyType=HASH \
    --billing-mod PAY_PER_REQUEST \
    --global-secondary-indexes \
    '[
      {
        "IndexName": "OrdersByStatus",
        "KeySchema": [
          {"AttributeName":"order_status","KeyType":"HASH"},
          {"AttributeName":"created_at","KeyType":"RANGE"}
        ],
        "Projection":{
          "ProjectionType":"ALL"
        }
      }
    ]'
