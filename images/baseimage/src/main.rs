use std::env;
use dotenv::dotenv;
use async_std::task;
mod app;

fn main() -> Result<(), std::io::Error> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);
    task::block_on(async {
        tide::log::start();
        let mut app = tide::new();
        app.at("/").nest({
            app::functions::handler()
        });
        println!("      Running server on: http://localhost:{}/", port);
        app.listen(address).await?;
        Ok(())
    })
}
