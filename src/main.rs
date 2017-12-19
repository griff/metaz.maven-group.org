extern crate addr2line;
extern crate bodyparser;
extern crate chrono;
#[macro_use]
extern crate hyper;
extern crate iron;
extern crate lru;
extern crate maxminddb;
extern crate mime_guess;
extern crate mount;
extern crate owning_ref;
extern crate persistent;
extern crate plugin;
#[macro_use]
extern crate prometheus;
extern crate protobuf;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate urlencoded;
extern crate url;
extern crate uuid;
extern crate zip;

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
use router::Router;
use urlencoded::UrlEncodedQuery;

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}
mod forward;
mod staticassets;
mod plcrash;
mod symbolicate;

use staticassets::{Static, request_path};

#[derive(Clone, Serialize, Deserialize)]
struct VersionInfo {
    short_version: String,
    dsym: Option<String>,
    draft: bool,
    prerelease: bool,
}

struct Profile {
    reader: Reader,
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
            let major_os : String = major_os_split.join(".");
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
    let reader = maxminddb::Reader::read(db.to_vec()).expect("Read geo db");
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

    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        Ok(Response::with((status::Ok, "OK")))
    }, "index");
    router.get("/symbolicate", |_: &mut Request| {
        Ok(Response::with((status::Ok, "OK")))
    }, "symbolicate");
    router.post("/symbolicate", symbolicate::Symbolicate{}, "symbolicate");
    mount.mount("/crash", router);

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
