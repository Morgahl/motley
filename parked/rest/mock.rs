// use std::fmt::Display;
// use std::marker::PhantomData;

// use anyhow::{Error, Result};

// use super::{ContentTyped, Resource as RESTResource};

// pub struct Resource<
//     Entity: ContentTyped + Default,
//     ListResource = (),
//     ShowResource = (),
//     CreateResource = (),
//     UpdateResource = (),
//     DeleteResource = (),
// > {
//     path: String,
//     id_path: String,
//     list: ListResource,
//     show: ShowResource,
//     create: CreateResource,
//     update: UpdateResource,
//     delete: DeleteResource,
//     _entity: PhantomData<Entity>,
// }

// impl<Entity: ContentTyped + Default> Resource<Entity> {
//     pub fn new<List, Show, Create, Update, Delete>(
//         path: String,
//         id_name: String,
//         list: List,
//         show: Show,
//         create: Create,
//         update: Update,
//         delete: Delete,
//     ) -> Resource<Entity, List, Show, Create, Update, Delete> {
//         let id_path = format!("{}/:{}", path, id_name);
//         Resource {
//             path,
//             id_path,
//             list,
//             show,
//             create,
//             update,
//             delete,
//             _entity: PhantomData,
//         }
//     }
// }

// impl<Entity, ListResource, ShowResource, CreateResource, UpdateResource, DeleteResource>
//     Resource<Entity, ListResource, ShowResource, CreateResource, UpdateResource, DeleteResource>
// where
//     Entity: ContentTyped + Default,
//     ListResource: super::ListResource<Entity>,
//     ShowResource: super::ShowResource<Entity>,
//     CreateResource: super::CreateResource<Entity>,
//     UpdateResource: super::UpdateResource<Entity>,
//     DeleteResource: super::DeleteResource<Entity>,
// {
//     pub fn inspect_resource(&self) -> Result<()>
//     where
//         Entity: Display,
//     {
//         println!("{}", self);
//         let list_resp = self.list(ListRequest::default())?;
//         println!("GET {}\n{}", self.list_path(), list_resp);

//         let show_resp = self.show(ShowRequest::default())?;
//         println!("GET {}\n{}", self.show_path(), show_resp);

//         let create_resp = self.create(CreateRequest::default())?;
//         println!("POST {}\n{}", self.create_path(), create_resp);

//         let update_resp = self.update(UpdateRequest::default())?;
//         println!("PUT {}\n{}", self.update_path(), update_resp);

//         let delete_resp = self.delete(DeleteRequest::default())?;
//         println!("DELETE {}\n{}", self.delete_path(), delete_resp);
//         Ok(())
//     }
// }

// impl<Entity: ContentTyped + Default, List, Show, Create, Update, Delete> RESTResource<Entity>
//     for Resource<Entity, List, Show, Create, Update, Delete>
// {
//     fn list_path(&self) -> String {
//         self.path.to_string()
//     }

//     fn show_path(&self) -> String {
//         self.id_path.to_string()
//     }

//     fn create_path(&self) -> String {
//         self.path.to_string()
//     }

//     fn update_path(&self) -> String {
//         self.id_path.to_string()
//     }

//     fn delete_path(&self) -> String {
//         self.id_path.to_string()
//     }
// }

// impl<Entity, List, Show, Create, Update, Delete> ListResource<Entity>
//     for Resource<Entity, List, Show, Create, Update, Delete>
// where
//     Self: RESTResource<Entity>,
//     Entity: ContentTyped + Default,
//     List: ListResource<Entity>,
// {
//     fn list(&self, req: ListRequest) -> Result<ListResponse<Entity>> {
//         self.list.list(req)
//     }
// }

// impl<Entity, List, Show, Create, Update, Delete> ShowResource<Entity>
//     for Resource<Entity, List, Show, Create, Update, Delete>
// where
//     Self: RESTResource<Entity>,
//     Entity: ContentTyped + Default,
//     Show: ShowResource<Entity>,
// {
//     fn show(&self, req: ShowRequest<Entity>) -> Result<ShowResponse<Entity>> {
//         self.show.show(req)
//     }
// }

// impl<Entity, List, Show, Create, Update, Delete> CreateResource<Entity>
//     for Resource<Entity, List, Show, Create, Update, Delete>
// where
//     Self: RESTResource<Entity>,
//     Entity: ContentTyped + Default,
//     Create: CreateResource<Entity>,
// {
//     fn create(&self, req: CreateRequest<Entity>) -> Result<CreateResponse<Entity>> {
//         self.create.create(req)
//     }
// }

// impl<Entity, List, Show, Create, Update, Delete> UpdateResource<Entity>
//     for Resource<Entity, List, Show, Create, Update, Delete>
// where
//     Self: RESTResource<Entity>,
//     Entity: ContentTyped + Default,
//     Update: UpdateResource<Entity>,
// {
//     fn update(&self, req: UpdateRequest<Entity>) -> Result<UpdateResponse<Entity>> {
//         self.update.update(req)
//     }
// }

// impl<Entity, List, Show, Create, Update, Delete> DeleteResource<Entity>
//     for Resource<Entity, List, Show, Create, Update, Delete>
// where
//     Self: RESTResource<Entity>,
//     Entity: ContentTyped + Default,
//     Delete: DeleteResource<Entity>,
// {
//     fn delete(&self, req: DeleteRequest<Entity>) -> Result<DeleteResponse> {
//         self.delete.delete(req)
//     }
// }

// impl<Entity, Listable, Showable, Createable, Updateable, Deleteable> Display
//     for Resource<Entity, Listable, Showable, Createable, Updateable, Deleteable>
// where
//     Entity: ContentTyped + Default,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}<{}> {{ path: {:?}, id_path: {:?} }}",
//             std::any::type_name::<Self>(),
//             std::any::type_name::<Entity>(),
//             self.path,
//             self.id_path,
//         )?;
//         write!(f, "    List: GET {}", self.path)?;
//         write!(f, "    Show: GET {}", self.id_path)?;
//         write!(f, "    Create: POST {}", self.path)?;
//         write!(f, "    Update: PUT {}", self.id_path)?;
//         write!(f, "    Delete: DELETE {}", self.id_path)
//     }
// }

// pub enum MockListResource<
//     Entity,
//     Handler: Fn(&Self, ListRequest) -> Result<ListResponse<Entity>>,
//     ResponseGenerator: Fn() -> ListResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// > where
//     Entity: ContentTyped,
// {
//     Handler(Handler),
//     Response(ResponseGenerator),
//     Error(ErrorGenerator),
// }

// impl<Entity, Handler, ResponseGenerator, ErrorGenerator> ListResource<Entity>
//     for MockListResource<Entity, Handler, ResponseGenerator, ErrorGenerator>
// where
//     Entity: ContentTyped,
//     Handler: Fn(&Self, ListRequest) -> Result<ListResponse<Entity>>,
//     ResponseGenerator: Fn() -> ListResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// {
//     fn list(&self, request: ListRequest) -> Result<ListResponse<Entity>> {
//         match self {
//             Self::Handler(handler) => handler(self, request),
//             Self::Response(response_generator) => Ok(response_generator()),
//             Self::Error(error_generator) => Err(error_generator()),
//         }
//     }
// }

// pub enum MockShowResource<
//     Entity,
//     Handler: Fn(&Self, ShowRequest<Entity>) -> Result<ShowResponse<Entity>>,
//     ResponseGenerator: Fn() -> ShowResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// > where
//     Entity: ContentTyped,
// {
//     Handler(Handler),
//     Response(ResponseGenerator),
//     Error(ErrorGenerator),
// }

// impl<Entity, Handler, ResponseGenerator, ErrorGenerator> ShowResource<Entity>
//     for MockShowResource<Entity, Handler, ResponseGenerator, ErrorGenerator>
// where
//     Entity: ContentTyped,
//     Handler: Fn(&Self, ShowRequest<Entity>) -> Result<ShowResponse<Entity>>,
//     ResponseGenerator: Fn() -> ShowResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// {
//     fn show(&self, request: ShowRequest<Entity>) -> Result<ShowResponse<Entity>> {
//         match self {
//             Self::Handler(handler) => handler(self, request),
//             Self::Response(response_generator) => Ok(response_generator()),
//             Self::Error(error_generator) => Err(error_generator()),
//         }
//     }
// }

// pub enum MockCreateResource<
//     Entity,
//     Handler: Fn(&Self, CreateRequest<Entity>) -> Result<CreateResponse<Entity>>,
//     ResponseGenerator: Fn() -> CreateResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// > where
//     Entity: ContentTyped,
// {
//     Handler(Handler),
//     Response(ResponseGenerator),
//     Error(ErrorGenerator),
// }

// impl<Entity, Handler, ResponseGenerator, ErrorGenerator> CreateResource<Entity>
//     for MockCreateResource<Entity, Handler, ResponseGenerator, ErrorGenerator>
// where
//     Entity: ContentTyped,
//     Handler: Fn(&Self, CreateRequest<Entity>) -> Result<CreateResponse<Entity>>,
//     ResponseGenerator: Fn() -> CreateResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// {
//     fn create(&self, request: CreateRequest<Entity>) -> Result<CreateResponse<Entity>> {
//         match self {
//             Self::Handler(handler) => handler(self, request),
//             Self::Response(response_generator) => Ok(response_generator()),
//             Self::Error(error_generator) => Err(error_generator()),
//         }
//     }
// }

// pub enum MockUpdateResource<
//     Entity,
//     Handler: Fn(&Self, UpdateRequest<Entity>) -> Result<UpdateResponse<Entity>>,
//     ResponseGenerator: Fn() -> UpdateResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// > where
//     Entity: ContentTyped,
// {
//     Handler(Handler),
//     Response(ResponseGenerator),
//     Error(ErrorGenerator),
// }

// impl<Entity, Handler, ResponseGenerator, ErrorGenerator> UpdateResource<Entity>
//     for MockUpdateResource<Entity, Handler, ResponseGenerator, ErrorGenerator>
// where
//     Entity: ContentTyped,
//     Handler: Fn(&Self, UpdateRequest<Entity>) -> Result<UpdateResponse<Entity>>,
//     ResponseGenerator: Fn() -> UpdateResponse<Entity>,
//     ErrorGenerator: Fn() -> Error,
// {
//     fn update(&self, request: UpdateRequest<Entity>) -> Result<UpdateResponse<Entity>> {
//         match self {
//             Self::Handler(handler) => handler(self, request),
//             Self::Response(response_generator) => Ok(response_generator()),
//             Self::Error(error_generator) => Err(error_generator()),
//         }
//     }
// }

// pub enum MockDeleteResource<
//     Entity,
//     Handler: Fn(&Self, DeleteRequest<Entity>) -> Result<DeleteResponse>,
//     ResponseGenerator: Fn() -> DeleteResponse,
//     ErrorGenerator: Fn() -> Error,
// > where
//     Entity: ContentTyped,
// {
//     Handler(Handler),
//     Response(ResponseGenerator),
//     Error(ErrorGenerator),
//     _PhantomEntity(PhantomData<Entity>),
// }

// impl<Entity, Handler, ResponseGenerator, ErrorGenerator> DeleteResource<Entity>
//     for MockDeleteResource<Entity, Handler, ResponseGenerator, ErrorGenerator>
// where
//     Entity: ContentTyped,
//     Handler: Fn(&Self, DeleteRequest<Entity>) -> Result<DeleteResponse>,
//     ResponseGenerator: Fn() -> DeleteResponse,
//     ErrorGenerator: Fn() -> Error,
// {
//     fn delete(&self, request: DeleteRequest<Entity>) -> Result<DeleteResponse> {
//         match self {
//             Self::Handler(handler) => handler(self, request),
//             Self::Response(response_generator) => Ok(response_generator()),
//             Self::Error(error_generator) => Err(error_generator()),
//             Self::_PhantomEntity(_) => unreachable!(),
//         }
//     }
// }
