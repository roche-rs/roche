pub fn handler() -> tide::Server<()> {    
    let mut api = tide::new();
    api.at("/").get(|_| async { Ok("Hello, world!") });
    api
}