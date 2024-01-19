#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Parts \
    --attribute-definitions \
        AttributeName=client_id#project_id#quotation_id,AttributeType=S \
        AttributeName=id,AttributeType=S \
        AttributeName=status,AttributeType=S \
        AttributeName=created_at,AttributeType=S \
    --key-schema \
        AttributeName=client_id#project_id#quotation_id,KeyType=HASH \
        AttributeName=id,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST \
    --global-secondary-indexes \
    '[
      {
        "IndexName": "PartsByStatus",
        "KeySchema": [
          {"AttributeName":"status","KeyType":"HASH"},
          {"AttributeName":"created_at","KeyType":"RANGE"}
        ],
        "Projection":{
          "ProjectionType":"ALL"
        }
      }
    ]'
