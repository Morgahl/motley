use anyhow::Result;
use http::{Request, Response};

use crate::Entity;

use super::EntitiesBody;

pub trait List<E: Entity> {
    fn list(&self, request: ListRequest) -> Result<ListResponse<E>>;
}

pub type ListHandler<E: Entity> = fn(ListRequest) -> Result<ListResponse<E>>;

impl<E: Entity> List<E> for ListHandler<E> {
    fn list(&self, request: ListRequest) -> Result<ListResponse<E>> {
        self(request)
    }
}

pub type ListRequest = Request<()>;

pub type ListResponse<E: Entity> = Response<EntitiesBody<E>>;
