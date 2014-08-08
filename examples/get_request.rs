extern crate iron;
extern crate urlencoded;

use iron::{Server, Iron, Chain, Request, Response, Status, Continue, FromFn};
use urlencoded::UrlEncodedMixin;
use std::io::net::ip::Ipv4Addr;

fn handler(req: &mut Request, _res: &mut Response) -> Status {
    match req.parse_urlencoded_query() {
        Some(hashmap) => {
            println!("Decoded query string: {}", hashmap);
        },
        None => {
            println!("No query parameters.");
        }
    }
    Continue
}

fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(FromFn::new(handler));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
