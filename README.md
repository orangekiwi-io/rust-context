# Lambda context object

Working example from [https://docs.aws.amazon.com/lambda/latest/dg/rust-context.html](https://docs.aws.amazon.com/lambda/latest/dg/rust-context.html)

## Pre-requisites
* An AWS account with all the necessary roles and permissions to deploy lambdas from your local cli.
* [Cargo Lambda](https://www.cargo-lambda.info/) installed
* Build the lambda for the AWS architecture.
    * Running Ubuntu 22.04 LTS under WSL (windows subsystem for linux), I use arm64 in the following command `cargo lambda build --arm64 --release` in the directory of your code.
    * Back in Windows, I run `cargo lambda deploy` to deploy the lambda to AWS Lambda.
 
