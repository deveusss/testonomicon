#![cfg(test)]

use serde_json::json;
use workspaces::prelude::*;

use testonomicon_near::TestScope;
const STATUS_MSG_WASM_FILEPATH: &str = "../testonomicon-near-examples/res/status_message.wasm";

#[tokio::test]
async fn status() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();
    let scope = TestScope::deploy(worker.clone(), STATUS_MSG_WASM_FILEPATH.to_owned()).await?;

    let res = scope
        .call(
            &worker,
            "set_status",
            json!({
                "message": "hello_world",
            }),
        )
        .await
        .unwrap();
    println!("{:?}", res);

    let view_result = scope
        .view(
            &worker,
            "get_status",
            json!({
                "account_id": scope.contract.id(),
            }),
        )
        .await
        .unwrap();

    println!("status: {:?}", view_result);

    Ok(())
}
