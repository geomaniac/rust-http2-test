extern crate httpbis;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;


struct ServiceImpl {
    counter: Arc<AtomicUsize>,
}

impl ServiceImpl {
    fn new() -> ServiceImpl {
        ServiceImpl {
            counter: Arc::new(AtomicUsize::new(0))
        }
    }
}

impl httpbis::Service for ServiceImpl {
    fn start_request(&self, req_headers: httpbis::Headers, _req: httpbis::HttpStreamAfterHeaders)
        -> httpbis::Response
    {
        println!("starting request: {:?}", req_headers);

        if req_headers.method() == "POST" {
            self.counter.fetch_add(1, Ordering::Relaxed);
            httpbis::Response::redirect_302("/")
        } else {
            let mut resp_headers = httpbis::Headers::ok_200();
            resp_headers.add("content-type", "text/html; charset=utf-8");

            let page = format!("
<html>
    <head>
        <title>httpbis demo</title>
    </head>
    <body>
        <h3>httpbis demo</h3>
        <div>Counter: {}</div>
        <div>
            <form method='POST' action='/inc'>
                <button type='submit'>Inc</button>
            </form>
        </div>
    </body>
</html>
        ", self.counter.load(Ordering::Relaxed));

            httpbis::Response::headers_and_bytes(resp_headers, page)
        }
    }
}


fn main() {
    let mut server = httpbis::ServerBuilder::new_plain();
    server.set_port(8443);
    server.service.set_service("/", Arc::new(ServiceImpl::new()));
    let server = server.build().expect("server");

    println!("started server");
    println!("check it at: https://localhost:{}/", server.local_addr().port().unwrap());

    loop {
        thread::park();
    }
}
