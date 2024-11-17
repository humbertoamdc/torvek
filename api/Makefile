ACCOUNT_ID := 120386585975
PROFILE := torvek
REGION := us-east-1

STAGING_ACCOUNT_ID := 869935083106
STAGING_PROFILE := torvek-staging
STAGING_REGION := us-east-1

CONFIG_FILES_S3_BUCKET := s3://torvek-config-files
STAGING_CONFIG_FILES_S3_BUCKET := s3://torvek-config-files-staging

# === App ===

run:
	./scripts/run-dev.sh

# === Infra ===

# Parameters
#	- env:	 string = Environment (staging | prod).
create-stacks:
	# cd into file_converter/ and deploy ecr
	make lambda
	make create-stack stack=dynamodb env=$$env
	make create-stack stack=s3-buckets env=$$env
	make create-stack stack=certificate-manager env=$$env
	make create-stack stack=api-gateway env=$$env
	make create-stack stack=cloudfront env=$$env

# Parameters
#	- env:	 string = Environment (staging | prod).
delete-stacks:
	make delete-stack stack=cloudfront env=$env
	make delete-stack stack=api-gateway env=$$env
	make delete-stack stack=certificate-manager env=$$env
	make delete-stack stack=s3-buckets env=$$env
	make delete-stack stack=dynamodb env=$$env
	make delete-stack stack=lambdas env=$$env

# Parameters
# 	- stack: string = Name of stack to create.
#	- env:	 string = Environment (staging | prod).
create-stack:
	@if [ "$(env)" = "prod" ]; then \
		profile=$(PROFILE); \
		region=$(REGION); \
		path="prod"; \
	else \
		profile=$(STAGING_PROFILE); \
		region=$(STAGING_REGION); \
		path="staging"; \
	fi; \
	aws --profile $$profile --region $$region cloudformation deploy \
		--capabilities CAPABILITY_NAMED_IAM \
		--template-file infra/aws/$$path/$(stack).yaml \
		--stack-name $(stack)

# Parameters
# 	- stack: string = Name of stack to create.
#	- env:	 string = Environment (staging | prod).
delete-stack:
	@if [ "$(env)" = "prod" ]; then \
		profile=$(PROFILE); \
		region=$(REGION); \
	else \
		profile=$(STAGING_PROFILE); \
		region=$(STAGING_REGION); \
	fi; \
	aws --profile $$profile --region $$region cloudformation delete-stack \
		--stack-name $(stack); \
	aws --profile $$profile --region $$region cloudformation wait stack-delete-complete \
		--stack-name $(stack); \

# Parameters
#	- env:	 string = Environment (staging | prod).
lambda:
	@if [ "$(env)" = "prod" ]; then \
		profile=$(PROFILE); \
		region=$(REGION); \
		accountId=$(ACCOUNT_ID); \
		environment="prod"; \
	else \
		profile=$(STAGING_PROFILE); \
		region=$(STAGING_REGION); \
		accountId=$(STAGING_ACCOUNT_ID); \
		environment="staging"; \
	fi; \
	make create-stack stack=lambdas env=$(env); \
	cargo lambda build --release --target x86_64-unknown-linux-gnu.2.17; \
	cargo lambda deploy --profile $$profile --region $$region api \
		--iam-role arn:aws:iam::$$accountId:role/ApiLambdaRole; \
	sleep 10; \
	aws --profile $$profile --region $$region lambda update-function-configuration \
		--function-name api \
		--environment "Variables={RUN_MODE=lambda,ENV=$$environment}" \
		--no-cli-pager

sync-envs:
	aws --profile $(PROFILE) --region $(REGION) s3 sync env $(CONFIG_FILES_S3_BUCKET)
	aws --profile $(STAGING_PROFILE) --region $(STAGING_REGION) s3 sync env $(STAGING_CONFIG_FILES_S3_BUCKET)
