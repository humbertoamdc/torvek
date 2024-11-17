ACCOUNT_ID := <PROD_ACCOUNT>
PROFILE := torvek
REGION := us-east-1

STAGING_ACCOUNT_ID := <STAGING_ACCOUNT>
STAGING_PROFILE := torvek-staging
STAGING_REGION := us-east-1


build-image:
	docker build -t file_converter --platform linux/arm64 .

# Parameters
#	- env:	 string = Environment (staging | prod).
ecr-login:
	@if [ "$(env)" = "prod" ]; then \
		profile=$(PROFILE); \
		region=$(REGION); \
		accountId=$(ACCOUNT_ID); \
	else \
		profile=$(STAGING_PROFILE); \
		region=$(STAGING_REGION); \
		accountId=$(STAGING_ACCOUNT_ID); \
	fi; \
	aws --profile $$profile --region $$region ecr get-login-password \
	| docker login --username AWS --password-stdin $$accountId.dkr.ecr.us-east-1.amazonaws.com

# Only need to run this the first time.
# Parameters
#	- env:	 string = Environment (staging | prod).
create-ecr:
	@if [ "$(env)" = "prod" ]; then \
		profile=$(PROFILE); \
		region=$(REGION); \
	else \
		profile=$(STAGING_PROFILE); \
		region=$(STAGING_REGION); \
	fi; \
	aws --profile $$profile --region $$region ecr create-repository \
		--repository-name file_converter \
		--image-scanning-configuration scanOnPush=true \
		--image-tag-mutability MUTABLE

# Parameters
#	- env:	 string = Environment (staging | prod).
push-image:
	@if [ "$(env)" = "prod" ]; then \
		profile=$(PROFILE); \
		region=$(REGION); \
		accountId=$(ACCOUNT_ID); \
	else \
		profile=$(STAGING_PROFILE); \
		region=$(STAGING_REGION); \
		accountId=$(STAGING_ACCOUNT_ID); \
	fi; \
	docker tag file_converter $$accountId.dkr.ecr.$$region.amazonaws.com/file_converter:latest; \
	docker push $$accountId.dkr.ecr.$$region.amazonaws.com/file_converter:latest
