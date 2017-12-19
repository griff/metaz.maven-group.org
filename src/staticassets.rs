use std::iter::FromIterator;
use std::path::{Component, PathBuf, Path};
use std::convert::AsRef;
use std::error::Error;
use std::fmt;

use iron::{IronResult, Request, Response, IronError, Handler, Url, status};
use iron::mime::{Mime, SubLevel, TopLevel};
use iron::modifiers::Redirect;
use mime_guess::guess_mime_type_opt;
use mount::OriginalUrl;
use url;
use url::percent_encoding::percent_decode;

use assets;

#[inline]
fn decode_percents(string: &&str) -> String {
    percent_decode(string.as_bytes()).decode_utf8().unwrap().into_owned()
}

fn normalize_path(path: &Path) -> PathBuf {
    path.components().fold(PathBuf::new(), |mut result, p| {
        match p {
            Component::Normal(x) => {
                result.push(x);
                result
            }
            Component::ParentDir => {
                result.pop();
                result
            },
            _ => result
        }
    })
}

pub fn request_path(request: &Request) -> PathBuf {
    let decoded_req_path = PathBuf::from_iter(request.url.path().iter().map(decode_percents));
    normalize_path(&decoded_req_path)
}

pub struct RequestedPath {
    path: PathBuf,
    path_str: String,
}

impl RequestedPath {
    pub fn new<P: AsRef<Path>>(root_path: P, request: &Request) -> RequestedPath {
        let mut result = root_path.as_ref().to_path_buf();
        result.extend(&request_path(request));
        RequestedPath {
            path_str: result.to_string_lossy().into_owned(),
            path: result
        }
    }

    pub fn should_redirect(&self, request: &Request) -> bool {
        // As per servo/rust-url/serialize_path, URLs ending in a slash have an
        // empty string stored as the last component of their path. Rust-url
        // even ensures that url.path() is non-empty by appending a forward slash
        // to URLs like http://example.com
        // Some middleware may mutate the URL's path to violate this property,
        // so the empty list case is handled as a redirect.
        let has_trailing_slash = match request.url.path().last() {
            Some(&"") => true,
            _ => false,
        };

        assets::get(&self.path_str).is_none() &&
            assets::get(&format!("{}/index.html", self.path_str)).is_some() &&
            !has_trailing_slash
    }

    pub fn get_file(self) -> Option<&'static [u8]> {
        if let Some(data) = assets::get(&self.path_str) {
            return Some(data);
        }

        let index_path = self.path.join("index.html");

        assets::get(&index_path.to_string_lossy())
    }
}

fn mime_for_path(path: &Path) -> Mime {
    guess_mime_type_opt(path)
        .unwrap_or_else(|| Mime(TopLevel::Text, SubLevel::Plain, vec![]))
}

pub struct Static {
    root: PathBuf,
}

impl Static {
    pub fn new<P: Into<PathBuf>>(root: P) -> Static {
        Static {
            root: root.into(),
        }
    }

    pub fn error(&self, status: status::Status) ->  Option<&'static [u8]>
    {
        let path = self.root.join(format!("{}.html", status.to_u16()));
        assets::get(&path.to_string_lossy())
    }
}

impl Handler for Static {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let requested_path = RequestedPath::new(&self.root, req);

        // If the URL ends in a slash, serve the file directly.
        // Otherwise, redirect to the directory equivalent of the URL.
        if requested_path.should_redirect(req) {
            // Perform an HTTP 301 Redirect.
            let mut original_url: url::Url = match req.extensions.get::<OriginalUrl>() {
                None => &req.url,
                Some(original_url) => original_url,
            }.clone().into();

            // Append the trailing slash
            //
            // rust-url automatically turns an empty string in the last
            // slot in the path into a trailing slash.
            original_url.path_segments_mut().unwrap().push("");
            let redirect_path = Url::from_generic_url(original_url).unwrap();

            return Ok(Response::with((status::MovedPermanently,
                                      format!("Redirecting to {}", redirect_path),
                                      Redirect(redirect_path))));
        }

        let mime = mime_for_path(&requested_path.path);
        match requested_path.get_file() {
            // If no file is found, return a 404 response.
            None => {
                if let Some(not_found) = self.error(status::NotFound) {
                    Err(IronError::new(NoFile, (status::NotFound, not_found)))
                } else {
                    Err(IronError::new(NoFile, status::NotFound))
                }
            },
            // Won't panic because we know the file exists from get_file.
            Some(data) => {
                Ok(Response::with((status::Ok, mime, data)))
            },
        }
    }
}

/// Thrown if no file is found. It is always accompanied by a NotFound response.
#[derive(Debug)]
pub struct NoFile;

impl Error for NoFile {
    fn description(&self) -> &str { "File not found" }
}

impl fmt::Display for NoFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
