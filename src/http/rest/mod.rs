pub mod mock;

use std::marker::PhantomData;

use anyhow::Result;

use crate::Identifiable;

use super::{ContentType, ContentTyped, Request, Response};

pub trait List<Entity: ContentTyped> {
    fn list_path(&self) -> String;
    fn list(&self, _request: ListRequest) -> Result<ListResponse<Entity>>;
}

pub trait Show<Entity: Identifiable + ContentTyped> {
    fn show_path(&self) -> String;
    fn show(&self, _request: EntityRequest<Entity>) -> Result<EntityResponse<Entity>>;
}

pub trait Create<Entity: ContentTyped> {
    fn create_path(&self) -> String;
    fn create(&self, _request: CreateRequest<Entity>) -> Result<EntityResponse<Entity>>;
}

pub trait Update<Entity: Identifiable + ContentTyped> {
    fn update_path(&self) -> String;
    fn update(&self, _request: UpdateRequest<Entity>) -> Result<EntityResponse<Entity>>;
}

pub trait Delete<Entity: Identifiable> {
    fn delete_path(&self) -> String;
    fn delete(&self, _request: EntityRequest<Entity>) -> Result<DeleteResponse>;
}

pub type ListRequest = Request<(), (), PagingQueryParams, ()>;

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            path_params: Default::default(),
            query_params: Default::default(),
            body: Default::default(),
        }
    }
}

pub type ListResponse<Entity> = Response<ContentTypeHeader<Entity>, EntitiesBody<Entity>>;

impl<Entity: ContentTyped + Default> Default for ListResponse<Entity> {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            status_code: Default::default(),
            body: Default::default(),
        }
    }
}

pub type EntityRequest<Entity> = Request<(), EntityIdentificationParameters<Entity>, (), ()>;

impl<Entity: Identifiable + Default> Default for EntityRequest<Entity> {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            path_params: Default::default(),
            query_params: Default::default(),
            body: Default::default(),
        }
    }
}

pub type EntityResponse<Entity> = Response<ContentTypeHeader<Entity>, EntityBody<Entity>>;

impl<Entity: ContentTyped + Default> Default for EntityResponse<Entity> {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            status_code: Default::default(),
            body: Default::default(),
        }
    }
}

pub type CreateRequest<Entity> = Request<ContentTypeHeader<Entity>, (), (), EntityBody<Entity>>;

impl<Entity: ContentTyped + Default> Default for CreateRequest<Entity> {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            path_params: Default::default(),
            query_params: Default::default(),
            body: Default::default(),
        }
    }
}

pub type UpdateRequest<Entity> = Request<
    ContentTypeHeader<Entity>,
    EntityIdentificationParameters<Entity>,
    (),
    EntityBody<Entity>,
>;

impl<Entity: Identifiable + ContentTyped + Default> Default for UpdateRequest<Entity> {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            path_params: Default::default(),
            query_params: Default::default(),
            body: Default::default(),
        }
    }
}

pub type DeleteResponse = Response<(), ()>;

impl Default for DeleteResponse {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            status_code: Default::default(),
            body: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ContentTypeHeader<Entity: ContentTyped> {
    pub content_type: ContentType,
    _phantom_entity: PhantomData<Entity>,
}

#[derive(Default, Debug)]
pub struct EntityIdentificationParameters<Entity: Identifiable> {
    pub entity_id: Entity::Id,
}

#[derive(Default, Debug)]
pub struct PagingQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Default, Debug)]
pub struct EntitiesBody<Entity> {
    pub entities: Vec<Entity>,
}

#[derive(Default, Debug)]
pub struct EntityBody<Entity> {
    pub entity: Entity,
}
