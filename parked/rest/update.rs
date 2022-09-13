use anyhow::Result;
use http::{Request, Response};

use crate::Entity;

use super::EntityBody;

pub trait Update<E: Entity> {
    fn update(&self, request: UpdateRequest<E>) -> Result<UpdateResponse<E>>;
}

pub type UpdateHandler<E: Entity> = fn(UpdateRequest<E>) -> Result<UpdateResponse<E>>;

impl<E: Entity> Update<E> for UpdateHandler<E> {
    fn update(&self, request: UpdateRequest<E>) -> Result<UpdateResponse<E>> {
        self(request)
    }
}

pub type UpdateRequest<E: Entity> = Request<EntityBody<E>>;

pub type UpdateResponse<E: Entity> = Response<EntityBody<E>>;
