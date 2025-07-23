import logging
import sys
import os
import boto3

sys.path.append('/usr/lib/freecad/lib')

import FreeCAD
import Part
import Mesh

logger = logging.getLogger(__name__)

base_tmp_storage_path = '/tmp/'
s3_web_ready_path = 'parts/web_ready/'

s3 = boto3.client("s3")
dynamodb_resource = boto3.resource('dynamodb')

parts_table = dynamodb_resource.Table('Parts')

def lambda_handler(event, context):
    logging.basicConfig(level=logging.INFO)
    logger.info("Event:", event)
    logger.info("Context:", context)

    print("Event", event)
    print("Context:", context)

    for record in event["Records"]:
        s3_bucket = record["s3"]["bucket"]["name"]
        s3_key = record["s3"]["object"]["key"]

        file_parts = s3_key.split('/')
        user_id = file_parts[-3]
        part_id = file_parts[-2]
        file_name_with_format = file_parts[-1]
        file_name, file_format = file_name_with_format.split('.')
        tmp_storage_path = base_tmp_storage_path + user_id + '/' + part_id + '/'
        s3_file_path = s3_web_ready_path + user_id + '/' + part_id + '/'

        os.makedirs(tmp_storage_path, exist_ok=True)

        download_file_from_s3(s3_bucket, s3_key, tmp_storage_path + file_name_with_format)

        convert_step_to_stl(tmp_storage_path + file_name_with_format, tmp_storage_path + file_name + '.stl')

        write_file_to_s3(tmp_storage_path, s3_bucket, s3_file_path, file_name + '.stl')

        # Update dynamodb part model
        stl_file_name = file_name + '.stl'
        s3_render_key = f"{s3_file_path}{stl_file_name}"
        file_metadata = {
            "name": stl_file_name,
            "key": s3_render_key,
        }
        update_dynamodb_render_file(user_id, part_id, file_metadata)

    body = {
        "message": "Success"
    }

    response = {
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json",
        },
        "body": "hello",
        "isBase64Encoded": False
    }

    return response


def download_file_from_s3(s3_bucket, s3_file_key, local_file_path):
    try:
        s3.download_file(s3_bucket, s3_file_key, local_file_path)
    except Exception as e:
        print(f"Error downloading file: {e}")


def convert_step_to_stl(input_file, output_file):
    # Load the STEP file
    shape = Part.Shape()
    shape.read(input_file)

    # Export to STL
    mesh = Mesh.Mesh()
    mesh.addFacets(shape.tessellate(2.0))
    mesh.write(output_file)


def write_file_to_s3(local_path, s3_bucket, s3_path, file):
    try:
        s3.upload_file(local_path + file, s3_bucket, s3_path + file)
    except Exception as e:
        print(f"Something went wrong while saving file to S3: {e}")


def update_dynamodb_render_file(customer_id, part_id, file_metadata):
    try:
        parts_table.update_item(
            Key={
                'pk': customer_id,
                'sk': part_id
            },
            UpdateExpression="SET render_file = :file",
            ExpressionAttributeValues={
                ":file": file_metadata
            }
        )
    except Exception as e:
        print(f"Error updating DynamoDB: {e} for customer {customer_id} and part: {part_id}")

