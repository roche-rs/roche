use std::env;
use async_std::task;
mod app;

fn main() -> Result<(), std::io::Error> {
    
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
