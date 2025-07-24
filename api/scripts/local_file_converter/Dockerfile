FROM python:3.11-slim

WORKDIR /usr/src/app

RUN pip install localstack-client trimesh
RUN pip install localstack-client boto3

RUN apt-get update && apt-get install -y freecad

COPY main.py .

CMD ["python", "-u", "./main.py"]
