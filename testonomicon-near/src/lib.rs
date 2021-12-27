use workspaces::prelude::*;
use workspaces::DevNetwork;
use workspaces::{Contract, Network, Worker};

pub struct TestScope {
    pub wasm_path: String,
    pub contract: Contract,
}

impl TestScope {
    pub async fn deploy(
        worker: Worker<impl DevNetwork>,
        wasm_path: String,
    ) -> anyhow::Result<TestScope> {
        let wasm = std::fs::read(wasm_path.clone()).unwrap();
        let contract = worker.dev_deploy(wasm).await?;
        let scope = TestScope::new(wasm_path, contract);
        Ok(scope)
    }
    pub(crate) fn new(wasm_path: String, contract: Contract) -> Self {
        TestScope {
            wasm_path,
            contract: contract,
        }
    }

    pub async fn call(
        &self,
        worker: &Worker<impl Network>,
        function: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<String> {
        let outcome = self
            .contract
            .call(worker, function)
            .args_json(args)
            .unwrap()
            .transact()
            .await;
        let result = format!("{:?}", outcome);
        Ok(result)
    }

    pub async fn view(
        &self,
        worker: &Worker<impl Network>,
        function: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<String> {
        let result: String = self
            .contract
            .view(&worker, function, args.to_string().into_bytes())
            .await
            .unwrap()
            .json()
            .unwrap();
        Ok(result)
    }
}
