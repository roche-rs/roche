#[cfg(test)]
mod tests {
    #[async_std::test]
    async fn it_works() {
        let uri = "http://localhost:8080/";
        let string: String = match surf::get(uri).recv_string().await {
            Ok(result) => result,
            Err(_) => "".to_string()
        };
        
        assert!(string.contains("httpbin.org"));
    }
}
