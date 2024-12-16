use data_storage::storage;
use lambda_runtime::{tracing, Error, LambdaEvent};
use serde_json::json;
use serde_json::Value;

pub(crate) async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    tracing::info!("lambda: {:?}", payload);

    let mut response = payload.clone();

    let now = std::time::Instant::now();
    tracing::info!("start storage test");
    storage();
    tracing::info!("end storage test");
    let time = now.elapsed().as_secs();
    response["time"] = json!(time);

    Ok(response)
}
