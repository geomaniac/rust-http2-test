extern crate env_logger;
extern crate futures;
extern crate httpbis;
extern crate url;

use std::env;
use std::process;

use futures::future::Future;

fn main() {
    env_logger::init();
    let args = env::args();
    let args: Vec<_> = args.collect();
    if args.len() != 2 {
        println!("usage: {} <url>", &args[0]);
        process::exit(1);
    }

    let url = &args[1];
    let url = url::Url::parse(&url).expect("parse url");

    let host = url.host_str().expect("URL must have host");
    let port = url.port().unwrap_or(80);

    loop {
        let mut line = String::new();
        ::std::io::stdin()
            .read_line(&mut line)
            .expect("correct input");
        let client = httpbis::Client::new_plain(host, port, Default::default()).expect("client");

        let resp = client
            .start_get(url.path(), host)
            .collect()
            .wait()
            .expect("execute request");

        print!("{}", resp.dump());
    }
}
