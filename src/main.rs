#![warn(unused_extern_crates)]
#[macro_use]
extern crate hyper;
extern crate iron;
extern crate maxminddb;
extern crate mime_guess;
extern crate mount;
#[macro_use]
extern crate prometheus;
extern crate serde;
extern crate serde_json;
extern crate urlencoded;
extern crate url;

use std::path::Path;
use std::collections::HashMap;

use iron::{AfterMiddleware, Chain, Iron, IronResult, Plugin, Request, Response};
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::mime::Mime;
use maxminddb::{MaxMindDBError, Reader};
use maxminddb::geoip2::City;
use mount::Mount;
use prometheus::{CounterVec, Encoder, TextEncoder};
use urlencoded::UrlEncodedQuery;

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
mod forward;
mod staticassets;
mod version_info;

use staticassets::{Static, request_path};
use version_info::VersionInfo;

struct Profile {
    reader: Reader<&'static [u8]>,
    counter: CounterVec,
    versions: HashMap<String, VersionInfo>,
}

impl AfterMiddleware for Profile {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let path = request_path(req);
        if path != Path::new("appcast.xml") && path != Path::new("appcast-prerelease.xml") {
            return Ok(res)
        }
        let mut os = None;
        let mut version = None;
        let mut channel = "release";
        if path == Path::new("appcast-prerelease.xml") {
            channel = "prerelease";
        }
        match req.get_ref::<UrlEncodedQuery>() {
            Ok(ref hashmap) => {
                if let Some(os_version) = hashmap.get("osVersion") {
                    os = os_version.first().map(|s| s.clone());
                }
                if let Some(app_version) = hashmap.get("appVersion") {
                    version = app_version.first().map(|s| s.clone());
                }
                println!("Parsed GET request query string:\n {:?}", hashmap)
            },
            Err(ref e) => println!("{:?}", e)
        };
        if let (Some(os), Some(version)) = (os, version) {
            let major_os_split : Vec<&str> = os.split(".").take(2).collect();
            let major_os = match major_os_split.first().map(|v| v.parse::<u32>()) {
                Some(Ok(v)) if v >= 11 => v.to_string(),
                _ => major_os_split.join(".")
            };
            let mut short_version = "self-compiled";
            if let Some(app) = self.versions.get(&version) {
                short_version = &app.short_version;
            }
            let ip = req.remote_addr.ip();
            let mut country_iso = String::from("unknown");
            println!("lookup {:?}", ip);
            let city_res : Result<City, MaxMindDBError> = self.reader.lookup(ip);
            if let Ok(city) = city_res {
                if let Some(country) = city.country {
                    if let Some(iso) = country.iso_code {
                        country_iso = iso;
                    }
                }
            }
            self.counter.with_label_values(&[&os, &major_os, &version, short_version, &country_iso, channel]).inc();
            println!("Incrementing {} {} {} {} {} {}", os, major_os, version, short_version, country_iso, channel);
        }
        Ok(res)
    }
}

fn main() {
    let versions_str = include_str!("../site/_data/versions.json");
    let versions : HashMap<String, VersionInfo> = serde_json::from_str(versions_str).expect("Read versions");
    let db = include_bytes!("../GeoLite2-City.mmdb");
    let reader = maxminddb::Reader::from_source(&db[..]).expect("Read geo db");
    let profile = Profile {
        reader: reader,
        counter: register_counter_vec!(
            "profile_total",
            "Total number of appcast requests with profile",
            &["os_version", "major_os_version", "app_version",
                "short_app_version", "country", "channel"]
        ).expect("Register counter"),
        versions: versions,
    };

    let mut mount = Mount::new();

    // Serve the shared JS/CSS at /
    mount.mount("/", Static::new(Path::new("site/_site")));

    let mut chain = Chain::new(mount);
    chain.link_before(forward::XForwardedMiddleware);
    chain.link_after(profile);
    let listener = Iron::new(chain).http("127.0.0.1:3000").expect("Listen");
    println!("Listening on {}", listener.socket);

    let encoder = TextEncoder::new();
    let metrics = Iron::new(move |_: &mut Request| {
        let metric_familys = prometheus::gather();
        let mut buffer = vec![];
        encoder.encode(&metric_familys, &mut buffer).expect("Encode metrics");
        Ok(Response::with((status::Ok,
            Header(ContentType(encoder.format_type().parse::<Mime>().expect("Metric header"))),
            buffer)))
    }).http("127.0.0.1:9898").expect("Metrics listen");
    println!("Listening for metrics on {}", metrics.socket);
}
