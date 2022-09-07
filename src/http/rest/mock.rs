use std::fmt::Display;
use std::marker::PhantomData;

use anyhow::Result;

use crate::Identifiable;

use super::{
    ContentTyped, Create, CreateRequest, Delete, DeleteResponse, EntityRequest, EntityResponse,
    List, ListRequest, ListResponse, Show, Update, UpdateRequest,
};

pub trait ListMock {}

pub trait ShowMock {}

pub trait CreateMock {}

pub trait UpdateMock {}

pub trait DeleteMock {}

#[derive(Default, Debug)]
pub struct MockResource<Entity: Identifiable + ContentTyped> {
    path: String,
    _entity: PhantomData<Entity>,
}

impl<Entity: Identifiable + ContentTyped> MockResource<Entity> {
    pub fn new(path: String) -> Self {
        Self {
            path,
            _entity: PhantomData,
        }
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }

    fn id_path(&self) -> String {
        format!("{}/:id", self.path)
    }
}

impl<Entity: Identifiable + ContentTyped + Default> List<Entity> for MockResource<Entity>
where
    Entity: ListMock,
{
    fn list_path(&self) -> String {
        self.path.to_string()
    }

    fn list(&self, _request: ListRequest) -> Result<ListResponse<Entity>> {
        Ok(ListResponse::default())
    }
}

impl<Entity: Identifiable + ContentTyped + Default> Show<Entity> for MockResource<Entity>
where
    Entity: ShowMock,
{
    fn show_path(&self) -> String {
        self.id_path()
    }

    fn show(&self, _request: EntityRequest<Entity>) -> Result<EntityResponse<Entity>> {
        Ok(EntityResponse::default())
    }
}

impl<Entity: Identifiable + ContentTyped + Default> Create<Entity> for MockResource<Entity>
where
    Entity: CreateMock,
{
    fn create_path(&self) -> String {
        self.path.to_string()
    }

    fn create(&self, _request: CreateRequest<Entity>) -> Result<EntityResponse<Entity>> {
        Ok(EntityResponse::default())
    }
}

impl<Entity: Identifiable + ContentTyped + Default> Update<Entity> for MockResource<Entity>
where
    Entity: UpdateMock,
{
    fn update_path(&self) -> String {
        self.id_path()
    }

    fn update(&self, _request: UpdateRequest<Entity>) -> Result<EntityResponse<Entity>> {
        Ok(EntityResponse::default())
    }
}

impl<Entity: Identifiable + ContentTyped> Delete<Entity> for MockResource<Entity>
where
    Entity: DeleteMock,
{
    fn delete_path(&self) -> String {
        self.id_path()
    }

    fn delete(&self, _request: EntityRequest<Entity>) -> Result<DeleteResponse> {
        Ok(DeleteResponse::default())
    }
}

impl<Entity: Identifiable + ContentTyped> Display for MockResource<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MockResource<{}> {{ path: {:?} }}",
            std::any::type_name::<Entity>(),
            self.path,
        )
    }
}
