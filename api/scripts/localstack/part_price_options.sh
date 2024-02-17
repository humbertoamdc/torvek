#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name PartPriceOptions \
    --attribute-definitions \
        AttributeName=part_id,AttributeType=S \
        AttributeName=id,AttributeType=S \
    --key-schema \
        AttributeName=part_id,KeyType=HASH \
        AttributeName=id,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST
