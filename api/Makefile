PROFILE := unnamed
REGION := us-east-1


# === App ===
run:
	./scripts/run-dev.sh

# === Infra ===
create-stacks:
	make create-stack stack=dynamodb
	make create-stack stack=s3-bucket
	make create-stack stack=cloudfront
	make create-stack stack=s3-permissions

# Parameters
# 	- stack: string = Name of stack to create.
create-stack:
	aws --profile $(PROFILE) --region $(REGION) cloudformation deploy \
		--capabilities CAPABILITY_NAMED_IAM \
		--template-file infra/$$stack.yaml \
		--stack-name $$stack \

lambda:
	make create-stack stack=lambda
	cargo lambda build --release;
	cargo lambda deploy --profile $(PROFILE) --region $(REGION) \
		--iam-role arn:aws:iam::120386585975:role/ApiLambdaRole
	sleep 5
	aws --profile $(PROFILE) --region $(REGION) lambda update-function-configuration \
		--function-name api \
        --environment "Variables={RUN_MODE=lambda}" \
        --no-cli-pager

sync-configs:
	aws --profile $(PROFILE) --region $(REGION) s3 sync config s3://unnamed-config-files
