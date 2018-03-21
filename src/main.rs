extern crate iron;
extern crate mount;
extern crate router;

use iron::status;
use iron::{Iron, Request, Response, IronResult};

use mount::Mount;
use router::Router;

fn dizer_oi(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Oi, mundo!")))
}

fn main() {

    let mut router = Router::new();
    router.get("/oi", dizer_oi, "dizer_oi");
    let mut mount = Mount::new();
    mount.mount("/", router);
    Iron::new(mount).http("0.0.0.0:5000").unwrap();

}

