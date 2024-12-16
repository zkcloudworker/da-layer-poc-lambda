# Test: Deploying Project Untitled POC to AWS Lambda

## Install

```sh
brew install awscli
brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda
aws configure
```

## Build

Run Docker desktop and run the following commands:

```sh
docker build -t lambda-rust-base -f Dockerfile.base .
docker build -t lambda-rust .
```

## Deploy

```sh
mkdir -p ./target/lambda/lambda
docker create --name temp lambda-rust
docker cp temp:/usr/src/app/target/lambda/lambda/bootstrap ./target/lambda/lambda/
docker rm temp
aws lambda delete-function --function-name lambda
cargo lambda deploy --binary-name lambda
```
