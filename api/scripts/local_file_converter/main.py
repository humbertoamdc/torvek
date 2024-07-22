import localstack_client.session as session
import os
import time
import json
import sys
import trimesh

sys.path.append("/usr/lib/freecad/lib")

import FreeCAD
import Part
import Mesh

# queue_url = os.getenv('QUEUE_URL')
queue_url = 'http://sqs.us-east-1.localhost.localstack.cloud:4566/000000000000/file-converter-queue'
s3_bucket = 'unnamed-client-files'
s3_web_ready_path = 'parts/web_ready/'

base_tmp_storage_path = '/tmp/'

s3 = session.client('s3')
sqs = session.client('sqs')


def fetch_messages_from_sqs():
    while True:
        try:
            response = sqs.receive_message(
                QueueUrl=queue_url,
                AttributeNames=['ALL'],
                MaxNumberOfMessages=5,
                WaitTimeSeconds=20,
            )

            messages = response.get('Messages', [])
            if not messages:
                print("No messages in the queue...")
                continue

            for message in messages:
                # print("Received message: %s" % message['Body'])
                body = json.loads(message['Body'])
                file_data = body['Records'][0]['s3']
                s3_key = file_data['object']['key']

                file_parts = s3_key.split('/')
                user_id = file_parts[-2]
                file_name_with_format = file_parts[-1]
                file_name, file_format = file_name_with_format.split('.')
                tmp_storage_path = base_tmp_storage_path + user_id + '/'
                s3_file_path = s3_web_ready_path + user_id + '/'

                os.makedirs(tmp_storage_path, exist_ok=True)

                download_file_from_s3(s3_key, tmp_storage_path + file_name_with_format)

                # convert_step_to_stl(tmp_storage_path + file_name_with_format, tmp_storage_path + file_name + '.stl')
                convert_step_to_obj(tmp_storage_path + file_name_with_format, tmp_storage_path + file_name + '.obj')
                convert_obj_to_glb(tmp_storage_path + file_name + '.obj', tmp_storage_path + file_name + '.glb')

                write_file_to_s3(tmp_storage_path, s3_file_path, file_name + '.glb')
                # write_file_to_s3(tmp_storage_path, s3_file_path, file_name + '.stl')

                sqs.delete_message(
                    QueueUrl=queue_url,
                    ReceiptHandle=message['ReceiptHandle']
                )

        except Exception as e:
            print(f"Something went wrong: {e}")
            time.sleep(5)


def download_file_from_s3(s3_file_key, local_file_path):
    try:
        s3.download_file(Bucket=s3_bucket, Key=s3_file_key, Filename=local_file_path)
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


def convert_step_to_obj(input_file, output_file):
    # Load the STEP file
    doc = App.newDocument()
    Part.insert(input_file, doc.Name)

    # Export to OBJ
    objs = FreeCAD.ActiveDocument.Objects
    Mesh.export(objs, output_file)

    # Name for the MTL file (same base name as the OBJ file)
    mtl_file = output_file.replace('.obj', '.mtl')

    # Create and write the MTL file
    with open(mtl_file, 'w') as mtl:
        mtl.write("newmtl WhiteMaterial\n")
        mtl.write("Ka 1.0 1.0 1.0\n")  # Ambient color (white)
        mtl.write("Kd 1.0 1.0 1.0\n")  # Diffuse color (white)
        mtl.write("Ks 0.5 0.5 0.5\n")  # Specular color
        mtl.write("Ns 10.0\n")  # Specular exponent
        mtl.write("d 1.0\n")  # Transparency
        mtl.write("illum 2\n")  # Illumination model

    # Ensure the OBJ file references the MTL file and uses the material
    with open(output_file, 'r') as obj_file:
        obj_content = obj_file.read()

    with open(output_file, 'w') as obj_file:
        # Write MTL file reference
        obj_file.write(f"mtllib {mtl_file.split('/')[-1]}\n")
        # Write usemtl statement before the rest of the content
        obj_file.write("usemtl WhiteMaterial\n")
        obj_file.write(obj_content)


def convert_obj_to_glb(input_file, output_file):
    # Load the OBJ file (Trimesh should automatically load the associated MTL file if present)
    tmesh = trimesh.load(input_file, force='mesh')

    # Check if materials were loaded
    if hasattr(tmesh.visual, 'material'):
        print("Material properties found and will be included in the GLTF.")
    else:
        print("No material properties found. Ensure the MTL file exists and is correctly referenced in the OBJ file.")

    # Directly use Trimesh to export the mesh to GLTF, handling binary data correctly
    tmesh.export(output_file, file_type='glb')  # Export as GLB (binary GLTF) for simplicity

    print(f"Conversion complete: '{output_file}'")


def write_file_to_s3(local_path, s3_path, file):
    try:
        s3.upload_file(local_path + file, s3_bucket, s3_path + file)
    except Exception as e:
        print(f"Something went wrong while saving file to S3: {e}")


if __name__ == "__main__":
    fetch_messages_from_sqs()
