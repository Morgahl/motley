use std::ops::{Deref, DerefMut};

use anyhow::Result;
use http::{
    header::HeaderName, response::Builder as HttpResponseBuilder, HeaderValue,
    Response as HttpResponse, StatusCode as HttpStatusCode, Version,
};
pub struct Response<Body>(HttpResponse<Body>);

impl<Body> Response<Body> {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl<Body> Deref for Response<Body> {
    type Target = HttpResponse<Body>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Body> DerefMut for Response<Body> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct Builder(HttpResponseBuilder);

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status<StatusCode>(self, status: StatusCode) -> Self
    where
        StatusCode: Into<HttpStatusCode>,
    {
        Self(self.0.status(status))
    }

    pub fn version(self, version: Version) -> Self {
        Self(self.0.version(version))
    }

    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        Self(self.0.header(key, value))
    }

    pub fn extension<Extension>(self, ext: Extension) -> Self
    where
        Extension: Send + Sync + 'static,
    {
        Self(self.0.extension(ext))
    }

    pub fn body<Body>(self, body: Body) -> Result<Response<Body>> {
        Ok(Response(self.0.body(body)?))
    }
}

impl Deref for Builder {
    type Target = HttpResponseBuilder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
