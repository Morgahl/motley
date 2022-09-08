use std::fmt::Display;
use std::marker::PhantomData;

use anyhow::Result;

use crate::{transport::http::StatusCode, Identifiable};

use super::{
    ContentTyped, CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, ListRequest,
    ListResponse, Resource as RESTResource, ShowRequest, ShowResponse, UpdateRequest,
    UpdateResponse,
};

pub trait Mocked {}

#[derive(Default, Debug)]
pub struct Resource<Entity: Identifiable + ContentTyped + Default> {
    path: String,
    id_path: String,
    list_default: Option<StatusCode>,
    show_default: Option<StatusCode>,
    create_default: Option<StatusCode>,
    update_default: Option<StatusCode>,
    delete_default: Option<StatusCode>,
    _entity: PhantomData<Entity>,
}

impl<Entity: Identifiable + ContentTyped + Default> Resource<Entity> {
    pub fn new(
        path: String,
        id_name: String,
        list_default: Option<StatusCode>,
        show_default: Option<StatusCode>,
        create_default: Option<StatusCode>,
        update_default: Option<StatusCode>,
        delete_default: Option<StatusCode>,
    ) -> Self {
        let id_path = format!("{}/:{}", path, id_name);
        Self {
            path,
            id_path: id_path,
            list_default,
            show_default,
            create_default,
            update_default,
            delete_default,
            _entity: PhantomData,
        }
    }

    pub fn list_path(&self) -> String {
        self.path.to_string()
    }

    pub fn list_default(&self) -> ListResponse<Entity> {
        ListResponse::new(
            Default::default(),
            self.list_default.unwrap_or(StatusCode::MethodNotAllowed),
            Default::default(),
        )
    }

    pub fn show_path(&self) -> String {
        self.id_path.to_string()
    }

    pub fn show_default(&self) -> ShowResponse<Entity> {
        ShowResponse::new(
            Default::default(),
            self.show_default.unwrap_or(StatusCode::MethodNotAllowed),
            Default::default(),
        )
    }

    pub fn create_path(&self) -> String {
        self.path.to_string()
    }

    pub fn create_default(&self) -> CreateResponse<Entity> {
        CreateResponse::new(
            Default::default(),
            self.create_default.unwrap_or(StatusCode::MethodNotAllowed),
            Default::default(),
        )
    }

    pub fn update_path(&self) -> String {
        self.id_path.to_string()
    }

    pub fn update_default(&self) -> UpdateResponse<Entity> {
        UpdateResponse::new(
            Default::default(),
            self.update_default.unwrap_or(StatusCode::MethodNotAllowed),
            Default::default(),
        )
    }

    pub fn delete_path(&self) -> String {
        self.id_path.to_string()
    }

    pub fn delete_default(&self) -> DeleteResponse {
        DeleteResponse::new(self.delete_default.unwrap_or(StatusCode::MethodNotAllowed))
    }
}

impl<Entity: Identifiable + ContentTyped + Default> RESTResource<Entity> for Resource<Entity>
where
    Entity: Mocked,
{
    fn list_path(&self) -> String {
        self.list_path()
    }

    fn list(&self, _: ListRequest) -> Result<ListResponse<Entity>> {
        Ok(self.list_default())
    }

    fn show_path(&self) -> String {
        self.show_path()
    }

    fn show(&self, _: ShowRequest<Entity>) -> Result<ShowResponse<Entity>> {
        Ok(self.show_default())
    }

    fn create_path(&self) -> String {
        self.create_path()
    }

    fn create(&self, _: CreateRequest<Entity>) -> Result<CreateResponse<Entity>> {
        Ok(self.create_default())
    }

    fn update_path(&self) -> String {
        self.update_path()
    }

    fn update(&self, _: UpdateRequest<Entity>) -> Result<UpdateResponse<Entity>> {
        Ok(self.update_default())
    }

    fn delete_path(&self) -> String {
        self.delete_path()
    }

    fn delete(&self, _: DeleteRequest<Entity>) -> Result<DeleteResponse> {
        Ok(self.delete_default())
    }
}

impl<Entity: Identifiable + ContentTyped + Default> Display for Resource<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list_details = match &self.list_default {
            Some(status_code) => format!("List: GET {} -> {}", self.path, status_code),
            None => format!("List: <not implemented> {}", StatusCode::MethodNotAllowed),
        };

        let show_details = match &self.show_default {
            Some(status_code) => format!("Show: GET {} -> {}", self.id_path, status_code),
            None => format!("Show: <not implemented> {}", StatusCode::MethodNotAllowed),
        };

        let create_details = match &self.create_default {
            Some(status_code) => format!("Create: POST {} -> {}", self.path, status_code),
            None => format!("Create: <not implemented> {}", StatusCode::MethodNotAllowed),
        };

        let update_details = match &self.update_default {
            Some(status_code) => format!("Update: PUT {} -> {}", self.id_path, status_code),
            None => format!("Update: <not implemented> {}", StatusCode::MethodNotAllowed),
        };

        let delete_details = match &self.delete_default {
            Some(status_code) => format!("Delete: DELETE {} -> {}", self.id_path, status_code),
            None => format!("Delete: <not implemented> {}", StatusCode::MethodNotAllowed),
        };

        write!(
            f,
            "{}<{}> {{ path: {:?} }}\n    {}\n    {}\n    {}\n    {}\n    {}",
            std::any::type_name::<Self>(),
            std::any::type_name::<Entity>(),
            self.path,
            list_details,
            show_details,
            create_details,
            update_details,
            delete_details,
        )
    }
}
