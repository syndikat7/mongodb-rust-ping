extern crate getopts;
use getopts::Options;
use mongodb::{bson::doc, sync::Client};
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> mongodb::error::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    // CLI options
    opts.optflag("", "ping", "send mongodb ping");
    opts.optflag("", "hello", "send mongodb ping");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0)
    }

    // Checks
    let port = env::var("MONGODB_PORT_NUMBER");
    if port.is_err() {
        println!("MONGODB_PORT_NUMBER MISSING!");
        std::process::exit(1);
    }
  
    let uri_options = env::var("MONGODB_URI_OPTIONS").unwrap_or("".to_string());

    // Establish connection
    let uport = port.unwrap();
    let uri = format!("mongodb://127.0.0.1:{}/?{}", uport, uri_options);

    println!("Connecting to {}", uri);

    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri)?;

    if matches.opt_present("ping") {
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)?;
        println!("PING OK");
        std::process::exit(0)
    } else if matches.opt_present("hello") {
        let hello = client
            .database("admin")
            .run_command(doc! {"hello": 1}, None)?;
        let is_writable_primary = hello.get_bool("isWritablePrimary").unwrap_or(false);
        let is_secondary = hello.get_bool("secondary").unwrap_or(false);

        if is_writable_primary || is_secondary {
            println!("HELLO OK");
            std::process::exit(0)
        } else {
            println!("HELLO FAIL");
            std::process::exit(1)
        }
    }

    // No option specifiec. print help and exit -1
    print_usage(&program, opts);
    std::process::exit(1)
}
