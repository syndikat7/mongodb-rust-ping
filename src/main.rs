use mongodb::{bson::doc, sync::Client};
use std::env;
fn main() -> mongodb::error::Result<()> {
    let port = env::var("MONGODB_PORT_NUMBER");
    if port.is_err(){
      println!("MONGODB_PORT_NUMBER MISSING!");
      std::process::exit(1);
    }
    let uri = format!("mongodb://localhost:{}", port.unwrap());

    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri)?;
    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}
