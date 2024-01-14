#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Quotations \
    --attribute-definitions \
        AttributeName=client_id#project_id,AttributeType=S \
        AttributeName=id,AttributeType=S \
    --key-schema \
        AttributeName=client_id#project_id,KeyType=HASH \
        AttributeName=id,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST \
