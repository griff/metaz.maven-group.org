use iron::modifiers::Redirect;
use iron::{Handler, Request, IronResult, Response};
use iron::status;
use url::Url;


pub struct Download {
    pub released: Option<iron::Url>,
    pub prereleased: Option<iron::Url>,
}

impl Handler for Download {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let url : Url = req.url.clone().into();
        if url.path() == "/latest.zip" {
            if let Some(url) = self.released.as_ref() {
                return Ok(Response::with((status::Found,
                    format!("Redirecting to {}", url),
                    Redirect(url.clone()))));
            }
        }
        if url.path() == "/prerelease.zip" {
            if let Some(url) = self.prereleased.as_ref() {
                return Ok(Response::with((status::Found,
                    format!("Redirecting to {}", url),
                    Redirect(url.clone()))));
            }
        }
        eprintln!("Downloads not found: {}", url.path());
        Ok(Response::with((status::NotFound, "NOT FOUND")))
    }
}