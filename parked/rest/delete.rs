use std::{marker::PhantomData, ops::Deref};

use anyhow::Result;
use http::{Request, Response};

use crate::Entity;

pub trait Delete<E: Entity> {
    fn delete(&self, request: DeleteRequest<E>) -> Result<DeleteResponse>;
}

pub struct DeleteRequest<E: Entity>(Request<()>, PhantomData<E>);

impl<E: Entity> Deref for DeleteRequest<E> {
    type Target = Request<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type DeleteResponse = Response<()>;

pub struct Handler<E: Entity>(
    fn(DeleteRequest<E>) -> Result<DeleteResponse>,
    PhantomData<E>,
)
where
    E: Entity;

impl<E: Entity> Delete<E> for Handler<E> {
    fn delete(&self, request: DeleteRequest<E>) -> Result<DeleteResponse> {
        self(request)
    }
}

impl<E: Entity> Deref for Handler<E> {
    type Target = fn(DeleteRequest<E>) -> Result<DeleteResponse>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
