mod common;

use {
    common::{Test, TestResult},
    hyper::StatusCode,
};

#[tokio::test(flavor = "multi_thread")]
async fn request_works() -> TestResult {
    let resp = Test::using_fixture("request.wasm").against_empty().await;
    assert_eq!(resp.status(), StatusCode::OK);
    Ok(())
}
