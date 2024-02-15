#!/bin/bash

# S3 Buckets
awslocal s3 mb s3://unnamed-client-files

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Orders \
    --attribute-definitions \
        AttributeName=id,AttributeType=S \
        AttributeName=part_id,AttributeType=S \
        AttributeName=status,AttributeType=S \
        AttributeName=created_at,AttributeType=S \
    --key-schema \
        AttributeName=id,KeyType=HASH \
    --billing-mod PAY_PER_REQUEST \
    --global-secondary-indexes \
    '[
      {
        "IndexName": "OrderForPart",
        "KeySchema": [
          {"AttributeName":"part_id","KeyType":"HASH"}
        ],
        "Projection":{
          "ProjectionType": "KEYS_ONLY"
        }
      },
      {
        "IndexName": "OrdersByStatus",
        "KeySchema": [
          {"AttributeName":"status","KeyType":"HASH"},
          {"AttributeName":"created_at","KeyType":"RANGE"}
        ],
        "Projection":{
          "ProjectionType":"ALL"
        }
      }
    ]'
