pub fn handler() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").get(|_| async {
        let uri = "https://httpbin.org/get";
        let string: String = surf::get(uri).recv_string().await?;
        Ok(string)
    });
    api
}
