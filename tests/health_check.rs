

#[cfg(test)]
mod tests{
    use emailserver::health_check;
    use emailserver::main;
    // #[tokio::test]
    // async fn health_check_test(){
    //     let response = health_check().await;
    //     assert!(response.status().is_success())
    // }

    #[test]
    fn dummy_test(){
        main();
    }

}