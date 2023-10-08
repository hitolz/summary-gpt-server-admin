pub fn new_summary_key() -> String {
    nanoid::nanoid!(8)
}

// #test
#[cfg(test)]
mod uuid_tests {
    use crate::utils::uuid;

    #[tokio::test]
    async fn test_new_summary_key() {
        for i in 0..10 {
            println!("{} --- {}", i, uuid::new_summary_key())
        }
    }
}
