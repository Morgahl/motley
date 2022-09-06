use std::marker::PhantomData;

use crate::entity::Identifiable;

use super::http;

pub trait Resource<Entity: Identifiable> {
    const PATH: String;
}

pub trait List<Entity>: Resource<Entity>
where
    Entity: Identifiable,
    Self::Request: http::Request<ListRequestBody<Entity>>,
    Self::Response: http::Response<ListResponseBody<Entity>>,
{
    type Request;
    type Response;

    fn list(&self, request: &Self::Request) -> &Self::Response;
}

pub trait Show<Entity>: Resource<Entity>
where
    Entity: Identifiable,
    Self::Request: http::Request<ShowRequestBody<Entity>>,
    Self::Response: http::Response<ShowResponseBody<Entity>>,
{
    type Request;
    type Response;

    fn show(&self, request: &Self::Request) -> &Self::Response;
}

pub trait Create<Entity>: Resource<Entity>
where
    Entity: Identifiable,
    Self::Request: http::Request<CreateRequestBody<Entity>>,
    Self::Response: http::Response<CreateResponseBody<Entity>>,
{
    type Request;
    type Response;

    fn create(&self, request: &Self::Request) -> &Self::Response;
}

pub trait Update<Entity>: Resource<Entity>
where
    Entity: Identifiable,
    Self::Request: http::Request<UpdateRequestBody<Entity>>,
    Self::Response: http::Response<UpdateResponseBody<Entity>>,
{
    type Request;
    type Response;

    fn update(&self, request: &Self::Request) -> &Self::Response;
}

pub trait Delete<Entity>: Resource<Entity>
where
    Entity: Identifiable,
    Self::Request: http::Request<DeleteRequestBody<Entity>>,
    Self::Response: http::Response<DeleteResponseBody<Entity>>,
{
    type Request;
    type Response;

    fn delete(&self, request: &Self::Request) -> &Self::Response;
}

impl<Entity> http::Request<ListRequestBody<Entity>> for Entity
where
    Entity: List<Entity> + Identifiable + http::RequestParameters,
{
    fn body(&self) -> &ListRequestBody<Entity> {
        todo!()
    }
}

impl<Entity> http::Response<ListResponseBody<Entity>> for Entity
where
    Entity: List<Entity> + Identifiable + http::ResponseParameters,
{
    fn body(&self) -> &ListResponseBody<Entity> {
        todo!()
    }
}

pub struct ListRequestBody<Entity: Identifiable> {
    _phantom: PhantomData<Entity>,
}

pub struct ListResponseBody<Entity: Identifiable> {
    pub entities: Vec<Entity>,
}

impl<Entity> http::Request<ShowRequestBody<Entity>> for Entity
where
    Entity: Show<Entity> + Identifiable + http::RequestParameters,
{
    fn body(&self) -> &ShowRequestBody<Entity> {
        todo!()
    }
}

impl<Entity> http::Response<ShowResponseBody<Entity>> for Entity
where
    Entity: Show<Entity> + Identifiable + http::ResponseParameters,
{
    fn body(&self) -> &ShowResponseBody<Entity> {
        todo!()
    }
}

pub struct ShowRequestBody<Entity: Identifiable> {
    pub id: Entity::Id,
}

pub struct ShowResponseBody<Entity: Identifiable> {
    pub entity: Entity,
}

impl<Entity> http::Request<CreateRequestBody<Entity>> for Entity
where
    Entity: Create<Entity> + Identifiable + http::RequestParameters,
{
    fn body(&self) -> &CreateRequestBody<Entity> {
        todo!()
    }
}

impl<Entity> http::Response<CreateResponseBody<Entity>> for Entity
where
    Entity: Create<Entity> + Identifiable + http::ResponseParameters,
{
    fn body(&self) -> &CreateResponseBody<Entity> {
        todo!()
    }
}

pub struct CreateRequestBody<Entity: Identifiable> {
    pub entity: Entity,
}

pub struct CreateResponseBody<Entity: Identifiable> {
    pub entity: Entity,
}

impl<Entity> http::Request<UpdateRequestBody<Entity>> for Entity
where
    Entity: Update<Entity> + Identifiable + http::RequestParameters,
{
    fn body(&self) -> &UpdateRequestBody<Entity> {
        todo!()
    }
}

impl<Entity> http::Response<UpdateResponseBody<Entity>> for Entity
where
    Entity: Update<Entity> + Identifiable + http::ResponseParameters,
{
    fn body(&self) -> &UpdateResponseBody<Entity> {
        todo!()
    }
}

pub struct UpdateRequestBody<Entity: Identifiable> {
    pub entity: Entity,
}

pub struct UpdateResponseBody<Entity: Identifiable> {
    pub entity: Entity,
}

impl<Entity> http::Request<DeleteRequestBody<Entity>> for Entity
where
    Entity: Delete<Entity> + Identifiable + http::RequestParameters,
{
    fn body(&self) -> &DeleteRequestBody<Entity> {
        todo!()
    }
}

impl<Entity> http::Response<DeleteResponseBody<Entity>> for Entity
where
    Entity: Delete<Entity> + Identifiable + http::ResponseParameters,
{
    fn body(&self) -> &DeleteResponseBody<Entity> {
        todo!()
    }
}

pub struct DeleteRequestBody<Entity: Identifiable> {
    pub id: Entity::Id,
}

pub struct DeleteResponseBody<Entity: Identifiable> {
    _phantom_entity: PhantomData<Entity>,
}
