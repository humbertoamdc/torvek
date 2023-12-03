PROFILE := unnamed
REGION := us-east-1

SHELL := /bin/bash

deploy:
	./scripts/build-prod.sh
	aws --profile $(PROFILE) --region $(REGION) s3 rm s3://unnamed-admin-portal --recursive
	aws --profile $(PROFILE) --region $(REGION) s3 sync dist/ s3://unnamed-admin-portal
	aws --profile $(PROFILE) --region $(REGION) cloudfront create-invalidation \
		--distribution-id <CLOUDFRONT_DISTRIBUTION_ID> \
		--paths "/*" \
		--no-cli-pager

run:
	./scripts/run-dev.sh


open:
	open -n -a "Google Chrome" --args --disable-web-security --user-data-dir="/tmp/chrome_dev_session" "http://localhost:8081"


aws --profile unnamed --region us-east-1 s3 rm s3://unnamed-admin-portal --recursive