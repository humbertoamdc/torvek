import logging
import sys
import os
import boto3

# import trimesh

sys.path.append('/usr/lib/freecad/lib')

import FreeCAD
import Part
import Mesh

# import Part
# import Mesh

# FILE_PATH = "/tmp/"

logger = logging.getLogger(__name__)

base_tmp_storage_path = '/tmp/'
s3_web_ready_path = 'parts/web_ready/'

s3 = boto3.client("s3")


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
        user_id = file_parts[-2]
        file_name_with_format = file_parts[-1]
        file_name, file_format = file_name_with_format.split('.')
        tmp_storage_path = base_tmp_storage_path + user_id + '/'
        s3_file_path = s3_web_ready_path + user_id + '/'

        os.makedirs(tmp_storage_path, exist_ok=True)

        download_file_from_s3(s3_bucket, s3_key, tmp_storage_path + file_name_with_format)

        convert_step_to_stl(tmp_storage_path + file_name_with_format, tmp_storage_path + file_name + '.stl')

        write_file_to_s3(tmp_storage_path, s3_bucket, s3_file_path, file_name + '.stl')

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

# def convert_step_to_obj(input_file, output_file):
#     # Load the STEP file
#     doc = App.newDocument()
#     Part.insert(input_file, doc.Name)
#
#     # Export to OBJ
#     objs = FreeCAD.ActiveDocument.Objects
#     Mesh.export(objs, output_file)
#
#     # Name for the MTL file (same base name as the OBJ file)
#     mtl_file = output_file.replace('.obj', '.mtl')
#
#     # Create and write the MTL file
#     with open(mtl_file, 'w') as mtl:
#         mtl.write("newmtl WhiteMaterial\n")
#         mtl.write("Ka 1.0 0.5 1.0\n")  # Ambient color
#         mtl.write("Kd 1.0 0.5 1.0\n")  # Diffuse color
#         mtl.write("Ks 0.5 0.5 0.5\n")  # Specular color
#         mtl.write("Ns 10.0\n")  # Specular exponent
#         mtl.write("d 1.0\n")  # Transparency
#         mtl.write("illum 2\n")  # Illumination model
#
#     # Ensure the OBJ file references the MTL file and uses the material
#     with open(output_file, 'r') as obj_file:
#         obj_content = obj_file.read()
#
#     with open(output_file, 'w') as obj_file:
#         # Write MTL file reference
#         obj_file.write(f"mtllib {mtl_file.split('/')[-1]}\n")
#         # Write usemtl statement before the rest of the content
#         obj_file.write("usemtl WhiteMaterial\n")
#         obj_file.write(obj_content)
#

# def convert_obj_to_glb(input_file, output_file):
#     # Load the OBJ file (Trimesh should automatically load the associated MTL file if present)
#     tmesh = trimesh.load(input_file, force='mesh')
#
#     # Check if materials were loaded
#     if hasattr(tmesh.visual, 'material'):
#         print("Material properties found and will be included in the GLTF.")
#     else:
#         print("No material properties found. Ensure the MTL file exists and is correctly referenced in the OBJ file.")
#
#     # Directly use Trimesh to export the mesh to GLTF, handling binary data correctly
#     tmesh.export(output_file, file_type='glb')  # Export as GLB (binary GLTF) for simplicity
#
#     print(f"Conversion complete: '{output_file}'")
#
# model_file = "io1"
# input_stp = FILE_PATH + model_file + '.stp'
# intermediary_obj = FILE_PATH + model_file + '.obj'
# output_gltf = FILE_PATH + model_file + '.glb'
#
# # Convert from STP to OBJ
# convert_step_to_obj(input_stp, intermediary_obj)
#
# # intermediary_obj = "./suzanne.obj"
# # output_gltf = './suzanne.glb'
#
# # Convert from OBJ to GLTF
# convert_obj_to_glb(intermediary_obj, output_gltf)
#
# # Clean up
# os.system(f"rm -rf /tmp/{model_file}*")
