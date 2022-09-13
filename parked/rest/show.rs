use anyhow::Result;
use http::{Request, Response};

use crate::Entity;

use super::EntityBody;

pub trait Show<E: Entity> {
    fn show(&self, request: ShowRequest<E>) -> Result<ShowResponse<E>>;
}

pub type ShowHandler<E: Entity> = fn(ShowRequest<E>) -> Result<ShowResponse<E>>;

impl<E: Entity> Show<E> for ShowHandler<E> {
    fn show(&self, request: ShowRequest<E>) -> Result<ShowResponse<E>> {
        self(request)
    }
}

pub type ShowRequest<E: Entity> = Request<()>;

pub type ShowResponse<E: Entity> = Response<EntityBody<E>>;
