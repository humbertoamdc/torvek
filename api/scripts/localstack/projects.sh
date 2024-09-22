#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Projects \
    --attribute-definitions \
        AttributeName=customer_id,AttributeType=S \
        AttributeName=id,AttributeType=S \
    --key-schema \
        AttributeName=customer_id,KeyType=HASH \
        AttributeName=id,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST \
