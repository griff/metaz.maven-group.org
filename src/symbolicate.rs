use std::io::Read;

use bodyparser::{MaxBodyLength, LimitReader};
use iron::{Handler, IronError, IronResult, Request, Response, status};
use iron::headers::ContentType;
use iron::typemap::{Key};
use persistent;
use plugin::{self, Pluggable};
use protobuf;

use plcrash;

struct CrashReport;
impl Key for CrashReport {
    type Value = plcrash::CrashReport;
}

const DEFAULT_BODY_LIMIT: usize = 1024 * 1024 * 10;

impl<'a, 'b> plugin::Plugin<Request<'a, 'b>> for CrashReport {
    type Error = protobuf::ProtobufError;

    fn eval(req: &mut Request) -> Result<plcrash::CrashReport, Self::Error> {
        let max_length = req
                .get::<persistent::Read<MaxBodyLength>>()
                .ok()
                .map(|x| *x)
                .unwrap_or(DEFAULT_BODY_LIMIT);
        let mut read = LimitReader::new(req.body.by_ref(), max_length);
        Ok(plcrash::read_report(&mut read)?)
    }
}

pub struct Symbolicate {
}

impl Handler for Symbolicate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        println!("Content-Type: {:?}", req.headers.get::<ContentType>());
        let report = req.get_ref::<CrashReport>().map_err(|e| {
            println!("Error {:?}", e);
            IronError::new(e, status::BadRequest)
        })?;
        let text = plcrash::text_report(&report);
        Ok(Response::with((status::Ok, text)))
    }
}
