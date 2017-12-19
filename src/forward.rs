use std::net::IpAddr;

use iron::{BeforeMiddleware, IronResult, Request};

header! { (XForwardedHost, "X-Forwarded-Host") => [String] }
header! { (XForwardedPort, "X-Forwarded-Port") => [u16] }
header! { (XForwardedProto, "X-Forwarded-Proto") => [String] }
header! { (XForwardedFor, "X-Forwarded-For") => (IpAddr)+ }

pub struct XForwardedMiddleware;

impl BeforeMiddleware for XForwardedMiddleware {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        macro_rules! h {
            ($x:path, $n:expr) => {{
                // FIXME: https://github.com/hyperium/hyper/issues/891
                let rv = match request.headers.get::<$x>() {
                    Some(x) => x.0.clone(),
                    None => match request.headers.get_raw($n) {
                        Some(raw_val) => panic!("Malformed header: {}: {:?}", $n, raw_val),
                        None => {
                            println!("Missing header: {:?}. Turn off use_proxy_headers or set proxy headers.", $n);
                            return Ok(())
                        }
                    }
                };
                assert!(request.headers.remove::<$x>());
                rv
            }}
        }
        //let host = h!(XForwardedHost, "X-Forwarded-Host");
        //let port = h!(XForwardedPort, "X-Forwarded-Port");
        //let scheme = h!(XForwardedProto, "X-Forwarded-Proto");
        let remote_addr = h!(XForwardedFor, "X-Forwarded-For")[0];

        /*{
            let mut url = request.url.as_mut();
            url.set_host(Some(&host)).unwrap();
            url.set_port(Some(port)).unwrap();
            url.set_scheme(&scheme).unwrap();
        }*/

        request.remote_addr.set_ip(remote_addr);
        Ok(())
    }
}
