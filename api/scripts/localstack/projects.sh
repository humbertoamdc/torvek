#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Projects \
    --attribute-definitions \
        AttributeName=pk,AttributeType=S \
        AttributeName=sk,AttributeType=S \
        AttributeName=lsi1_sk,AttributeType=S \
        AttributeName=lsi2_sk,AttributeType=S \
    --key-schema \
        AttributeName=pk,KeyType=HASH \
        AttributeName=sk,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST \
    --local-secondary-indexes \
      '[
        {
          "IndexName": "LSI1_CreationDateTime",
          "KeySchema": [
            {"AttributeName":"pk", "KeyType":"HASH"},
            {"AttributeName":"lsi1_sk", "KeyType":"RANGE"}
          ],
          "Projection": {
            "ProjectionType": "ALL"
          }
        }
      ]' \
    --global-secondary-indexes \
      '[
        {
          "IndexName": "GSI2_ProjectName",
          "KeySchema": [
            {"AttributeName":"pk", "KeyType":"HASH"},
            {"AttributeName":"lsi2_sk", "KeyType":"RANGE"}
          ],
          "Projection": {
            "ProjectionType": "ALL"
          }
        }
      ]'
