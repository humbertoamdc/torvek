FROM ubuntu:latest

RUN mkdir app && cd app && mkdir lambda_function
COPY requirements.txt /app
COPY src/lambda_function.py /app/lambda_function

ENV DEBIAN_FRONTEND=noninteractive
RUN \
    --mount=type=cache,target=/var/cache/apt \
    apt-get update && \
    apt-get install -y python3 && \
    apt-get install -y python3-pip && \
    apt-get install -y zip && \
    apt-get install -y freecad
ENV DEBIAN_FRONTEND=dialog

#RUN cd /app && pip install \
#        --platform manylinux2014_x86_64 \
#        --target=lambda_function \
#        --implementation cp \
#        --python-version 3.11 \
#        --only-binary=:all: --upgrade \
#        -r requirements.txt

RUN mv /usr/lib/freecad /app/lambda_function
RUN mv /usr/lib/freecad-python3 /app/lambda_function

RUN cd app/lambda_function && \
    zip -r ../deployment_package.zip *

#RUN pip install -r requirements.txt
# RUN pip install -r requirements.txt --target "${LAMBDA_TASK_ROOT}"
#RUN cd app && \
#    pip3 install --target . -r requirements.txt && \
#    zip -r deployment_package.zip *

#FROM amazonlinux:2
#FROM alpine
#
#RUN mkdir app
#COPY requirements.txt /app
#COPY src/lambda_function.py /app
#
#RUN apk update
#RUN apk add zip
#RUN apk add python3
#RUN apk add py3-pip
#
#RUN cd app && \
#    pip3 install --target . -r requirements.txt && \
#    zip -r deployment_package.zip *
