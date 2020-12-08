use std::env;
pub fn handler() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").get(|_| async { 
        let key = "MY_VAR";
        let envvar = match env::var(key) {
            Ok(val) => val,
            Err(_e) => "".to_string(),
        };
        Ok(format!("{}", envvar)) 
    });
    api
}
