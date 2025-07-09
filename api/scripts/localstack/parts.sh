#!/bin/bash

# DynamoDB Tables
awslocal dynamodb create-table \
    --table-name Parts \
    --attribute-definitions \
        AttributeName=pk,AttributeType=S \
        AttributeName=sk,AttributeType=S \
        AttributeName=lsi1_sk,AttributeType=S \
    --key-schema \
        AttributeName=pk,KeyType=HASH \
        AttributeName=sk,KeyType=RANGE \
    --billing-mod PAY_PER_REQUEST \
    --local-secondary-indexes \
      '[
        {
          "IndexName": "LSI1_QuoteAndCreationDateTime",
          "KeySchema": [
            {"AttributeName":"pk", "KeyType":"HASH"},
            {"AttributeName":"lsi1_sk", "KeyType":"RANGE"}
          ],
          "Projection": {
            "ProjectionType": "ALL"
          }
        }
      ]'

# TODO: Move everything below to a general infra file.
# SQS
awslocal sqs create-queue --queue-name file-converter-queue

# S3 Buckets
awslocal s3 mb s3://unnamed-client-files
awslocal s3api put-bucket-notification-configuration \
  --bucket unnamed-client-files \
  --notification-configuration \
    '
    {
      "QueueConfigurations": [
        {
          "QueueArn": "arn:aws:sqs:us-east-1:000000000000:file-converter-queue",
          "Events": ["s3:ObjectCreated:Put", "s3:ObjectCreated:Post"],
          "Filter": {
            "Key": {
              "FilterRules": [
                {
                  "Name": "prefix",
                  "Value": "parts/originals/"
                }
              ]
            }
          }
        }
      ]
    }
    '
