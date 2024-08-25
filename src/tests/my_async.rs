#[tokio::test]
async fn my_async_test() {
    // Simulate an asynchronous operation
    let result = async_operation().await;

    // Assert that the result is as expected
    assert_eq!(result, 42);
}

async fn async_operation() -> i32 {
    // Perform some async work
    42
}
