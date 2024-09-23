use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let context = event.context.clone();
    let invoked_function_arn = event.context.invoked_function_arn;
    println!("*** event.context ***\n\n {:#?}", context);
    Ok(json!({ "message": format!("Hello, this is function {invoked_function_arn}!") }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}
