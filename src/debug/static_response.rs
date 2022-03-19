use std::sync::Arc;

use mime::Mime;
use rc_u8_reader::ArcU8Reader;

use crate::rocket::http::Status;
use crate::rocket::request::Request;
use crate::rocket::response::{self, Responder, Response};
use crate::EntityTag;

#[derive(Debug)]
struct StaticResponseInner {
    mime: String,
    data: Arc<Vec<u8>>,
    etag: String,
}

#[derive(Debug)]
/// To respond a static resource.
pub struct StaticResponse {
    inner: Option<StaticResponseInner>,
}

impl StaticResponse {
    #[inline]
    pub(crate) fn build(
        mime: &Mime,
        data: Arc<Vec<u8>>,
        etag: &EntityTag<'static>,
    ) -> StaticResponse {
        StaticResponse {
            inner: Some(StaticResponseInner {
                mime: mime.to_string(),
                data,
                etag: etag.to_string(),
            }),
        }
    }

    #[inline]
    pub(crate) const fn not_modified() -> StaticResponse {
        StaticResponse {
            inner: None,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for StaticResponse {
    #[inline]
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        let mut response = Response::build();

        if let Some(inner) = self.inner {
            response.raw_header("Etag", inner.etag);
            response.raw_header("Content-Type", inner.mime);

            response.sized_body(inner.data.len(), ArcU8Reader::new(inner.data));
        } else {
            response.status(Status::NotModified);
        }

        response.ok()
    }
}
