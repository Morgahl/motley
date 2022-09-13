use std::ops::{Deref, DerefMut};

use anyhow::Result;
use http::{
    header::{HeaderName, AUTHORIZATION, CONTENT_TYPE},
    request::Builder as HttpRequestBuilder,
    Error as HttpError, HeaderValue, Method as HttpMethod, Request as HttpRequest, Uri as HttpUri,
    Version,
};

pub struct Request<Body>(HttpRequest<Body>);

impl<Body> Request<Body> {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn get<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::GET).uri(uri)
    }

    pub fn put<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::PUT).uri(uri)
    }

    pub fn post<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::POST).uri(uri)
    }

    pub fn delete<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::DELETE).uri(uri)
    }

    pub fn options<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::OPTIONS).uri(uri)
    }

    pub fn head<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::HEAD).uri(uri)
    }

    pub fn connect<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::CONNECT).uri(uri)
    }

    pub fn patch<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::PATCH).uri(uri)
    }

    pub fn trace<U>(uri: U) -> Builder
    where
        HttpUri: TryFrom<U>,
        <HttpUri as TryFrom<U>>::Error: Into<HttpError>,
    {
        Builder::new().method(HttpMethod::TRACE).uri(uri)
    }
}

impl<Body> Deref for Request<Body> {
    type Target = HttpRequest<Body>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Body> DerefMut for Request<Body> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct Builder(HttpRequestBuilder);

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn method<Method>(self, method: Method) -> Self
    where
        Method: Into<HttpMethod>,
    {
        Self(self.0.method(method))
    }

    pub fn uri<Uri>(self, uri: Uri) -> Self
    where
        HttpUri: TryFrom<Uri>,
        <HttpUri as TryFrom<Uri>>::Error: Into<HttpError>,
    {
        Self(self.0.uri(uri))
    }

    pub fn version(self, version: Version) -> Self {
        Self(self.0.version(version))
    }

    pub fn header<Name, Value>(self, name: Name, value: Value) -> Self
    where
        Name: Into<HeaderName>,
        Value: Into<HeaderValue>,
    {
        Self(self.0.header(name, value))
    }

    pub fn extension<Extension>(self, ext: Extension) -> Self
    where
        Extension: Send + Sync + 'static,
    {
        Self(self.0.extension(ext))
    }

    pub fn body<Body>(self, body: Body) -> Result<Request<Body>> {
        Ok(Request(self.0.body(body)?))
    }

    pub fn content_type<ContentType>(self, content_type: ContentType) -> Self
    where
        ContentType: Into<HeaderValue>,
    {
        self.header(CONTENT_TYPE, content_type)
    }

    pub fn authorization<Auth>(self, auth: Auth) -> Self
    where
        Auth: Into<HeaderValue>,
    {
        self.header(AUTHORIZATION, auth)
    }
}

impl Deref for Builder {
    type Target = HttpRequestBuilder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
