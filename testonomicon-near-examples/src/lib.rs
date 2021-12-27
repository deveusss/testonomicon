#![cfg(test)]

use serde_json::json;
use testonomicon::assertions::string::StrAssertions;
use testonomicon::*;
use testonomicon_near::TestScope;
const STATUS_MSG_WASM_FILEPATH: &str = "../testonomicon-near-examples/res/status_message.wasm";

#[tokio::test]
async fn status() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();
    let scope = TestScope::deploy(worker.clone(), STATUS_MSG_WASM_FILEPATH.to_owned()).await?;

    let _call_result = scope
        .call(
            &worker,
            "set_status",
            json!({
                "message": "Hello From Testonomicon",
            }),
        )
        .await
        .unwrap();

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

    //assertion(Some("description").that(&view_result).contains("Hello From Testonomicon");
    assert()
        .that(&view_result)
        .is_equal_to("Hello From Testonomicon".to_owned());

    assert()
        .that(&view_result)
        .contains("Hello From Testonomicon");
    Ok(())
}
