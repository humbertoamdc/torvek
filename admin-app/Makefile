PROFILE := torvek
REGION := us-east-1
S3_URI := s3://torvek-web-apps/admins
CLOUDFRONT_DISTRIBUTION := E2G8P0AFMND1XM

STAGING_PROFILE := torvek-staging
STAGING_REGION := us-east-1
STAGING_S3_URI := s3://torvek-web-apps-staging/admins
STAGING_CLOUDFRONT_DISTRIBUTION := E38O58FV2Q21NO

SHELL := /bin/bash

# Parameters
#	- env:	 string = Environment (staging | prod).
deploy:
	@if [ "$(env)" = "prod" ]; then \
		./scripts/build-prod.sh; \
		profile=$(PROFILE); \
		region=$(REGION); \
		s3Uri=$(S3_URI); \
		cloudfrontDistribution=$(CLOUDFRONT_DISTRIBUTION); \
	else \
		./scripts/build-staging.sh; \
		profile=$(STAGING_PROFILE); \
		region=$(STAGING_REGION); \
		s3Uri=$(STAGING_S3_URI); \
		cloudfrontDistribution=$(STAGING_CLOUDFRONT_DISTRIBUTION); \
	fi; \
	aws --profile $$profile --region $$region s3 rm $$s3Uri --recursive; \
	aws --profile $$profile --region $$region s3 sync dist/ $$s3Uri; \
	aws --profile $$profile --region $$region cloudfront create-invalidation \
		--distribution-id $$cloudfrontDistribution \
		--paths "/*" \
		--no-cli-pager

run:
	./scripts/run-dev.sh


open:
	open -n -a "Google Chrome" --args --disable-web-security --user-data-dir="/tmp/chrome_dev_session" "http://localhost:8081"
