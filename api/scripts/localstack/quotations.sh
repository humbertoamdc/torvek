#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Quotations \
    --attribute-definitions \
        AttributeName=project_id,AttributeType=S \
        AttributeName=id,AttributeType=S \
    --key-schema \
        AttributeName=project_id,KeyType=HASH \
        AttributeName=id,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST \
