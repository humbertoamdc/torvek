PROFILE := unnamed
REGION := us-east-1

SHELL := /bin/bash

deploy:
	./scripts/build-prod.sh
	aws --profile $(PROFILE) --region $(REGION) s3 rm s3://torvek-admin-web --recursive
	aws --profile $(PROFILE) --region $(REGION) s3 sync dist/ s3://torvek-admin-web
	aws --profile $(PROFILE) --region $(REGION) cloudfront create-invalidation \
		--distribution-id E2G8P0AFMND1XM \
		--paths "/*" \
		--no-cli-pager

run:
	./scripts/run-dev.sh


open:
	open -n -a "Google Chrome" --args --disable-web-security --user-data-dir="/tmp/chrome_dev_session" "http://localhost:8081"
