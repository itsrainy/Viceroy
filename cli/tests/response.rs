mod common;

use {
    common::{Test, TestResult},
    hyper::StatusCode,
};

#[tokio::test(flavor = "multi_thread")]
async fn response_works() -> TestResult {
    let resp = Test::using_fixture("response.wasm").against_empty().await;
    assert_eq!(resp.status(), StatusCode::OK);
    Ok(())
}
