use std::convert::TryInto;
use std::ops::Deref;

use anyhow::Result;
use http::{
    header::{self, HeaderValue},
    Response as HTTPResponse,
};

use crate::{http::Request as HTTPRequest, Entity};

use super::EntityBody;

pub trait Create<E>
where
    E: Entity,
{
    fn create(&self, request: Request<E>) -> Result<Response<E>>;
}

pub struct Request<E>(HTTPRequest<EntityBody<E>>)
where
    E: Entity;

impl<E> Request<E>
where
    E: Entity,
{
    pub fn new<Body, ContentType>(
        uri: String,
        content_type: ContentType,
        body: Body,
    ) -> Result<Self>
    where
        HeaderValue: From<ContentType>,
        Body: TryInto<EntityBody<E>>,
        Body::Error: std::error::Error + Send + Sync + 'static,
    {
        let body = body.try_into()?;
        let request = HTTPRequest::post(uri)
            .header(header::CONTENT_TYPE, content_type)
            .body(body)?;
        Ok(Self(request))
    }
}

impl<E: Entity> From<HTTPRequest<EntityBody<E>>> for Request<E> {
    fn from(request: HTTPRequest<EntityBody<E>>) -> Self {
        Self(request)
    }
}

impl<E: Entity> Deref for Request<E> {
    type Target = HTTPRequest<EntityBody<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Response<E: Entity> = HTTPResponse<EntityBody<E>>;

pub struct Handler<E: Entity>(fn(Request<E>) -> Result<Response<E>>);

impl<E: Entity> Create<E> for Handler<E> {
    fn create(&self, request: Request<E>) -> Result<Response<E>> {
        self(request)
    }
}

impl<E: Entity> Deref for Handler<E> {
    type Target = fn(Request<E>) -> Result<Response<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
