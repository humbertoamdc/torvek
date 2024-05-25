PROFILE := unnamed
REGION := us-east-1


run:
	docker build -t file_converter .
	docker create --name file_converter file_converter
#	docker cp file_converter:/app/deployment_package.zip .
#	docker cp file_converter:/app/deployment_package.zip .
	docker run -it file_converter

activate-venv:
	source ./venv/bin/activate

build-lambda:
	pip install \
		--platform manylinux2014_x86_64 \
		--target=lambda_function \
		--implementation cp \
		--python-version 3.11 \
		--only-binary=:all: --upgrade \
		-r requirements.txt
	cp /src/lambda_function.py /lambda_function
	cd /lambda_function && zip -r ../deployment_package.zip .

localstack-lambda:
	awslocal lambda create-function \
		--function-name file_converter \
		--runtime python3.11 \
		--zip-file fileb://deployment_package.zip \
		--handler lambda_function.lambda_handler \
		--role arn:aws:iam::000000000000:role/lambda-role

invoke:
	awslocal lambda invoke --function-name file_converter \
 	  --cli-binary-format raw-in-base64-out \
      --payload '{"body": "{\"num1\": \"10\", \"num2\": \"10\"}" }' \
      output.txt


# IMPORTANT: The commands below are the important ones to deploy the lambda. After we have a
#			 working version in production we can safely delete the commands above.
lambda-ecr:
	docker build -t file_converter --platform linux/amd64 .

create-ecr:
	aws --profile $(PROFILE) --region $(REGION) ecr get-login-password \
	| docker login --username AWS --password-stdin 120386585975.dkr.ecr.us-east-1.amazonaws.com
	aws --profile $(PROFILE) --region $(REGION) ecr create-repository \
		--repository-name file_converter \
		--image-scanning-configuration scanOnPush=true \
		--image-tag-mutability MUTABLE

deploy-ecr:
	docker tag file_converter 120386585975.dkr.ecr.us-east-1.amazonaws.com/file_converter:latest
	docker push 120386585975.dkr.ecr.us-east-1.amazonaws.com/file_converter:latest
