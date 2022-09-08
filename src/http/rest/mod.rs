pub mod mock;

use std::{fmt::Display, marker::PhantomData};

use anyhow::Result;

use crate::Identifiable;

use super::{ContentType, ContentTyped, Request, Response, StatusCode};

pub trait Resource<Entity: Identifiable + ContentTyped> {
    fn list_path(&self) -> String;
    fn list(&self, request: ListRequest) -> Result<ListResponse<Entity>>;

    fn show_path(&self) -> String;
    fn show(&self, request: ShowRequest<Entity>) -> Result<ShowResponse<Entity>>;

    fn create_path(&self) -> String;
    fn create(&self, request: CreateRequest<Entity>) -> Result<CreateResponse<Entity>>;

    fn update_path(&self) -> String;
    fn update(&self, request: UpdateRequest<Entity>) -> Result<UpdateResponse<Entity>>;

    fn delete_path(&self) -> String;
    fn delete(&self, request: DeleteRequest<Entity>) -> Result<DeleteResponse>;
}

pub struct ListRequest(Request<(), (), PagingQueryParams, ()>);

impl ListRequest {
    pub fn new(query_params: PagingQueryParams) -> Self {
        Self(Request {
            headers: (),
            path_params: (),
            query_params,
            body: (),
        })
    }
}

impl Display for ListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListRequest {{ query_params: {} }}", self.0.query_params)
    }
}

impl Default for ListRequest {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub struct ListResponse<Entity: ContentTyped>(
    Response<ContentTypeHeader<Entity>, EntitiesBody<Entity>>,
);

impl<Entity: ContentTyped> ListResponse<Entity> {
    pub fn new(
        headers: ContentTypeHeader<Entity>,
        status_code: StatusCode,
        body: EntitiesBody<Entity>,
    ) -> Self {
        Self(Response {
            headers,
            status_code,
            body,
        })
    }
}

impl<Entity: ContentTyped + Display> Display for ListResponse<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ListResponse {{ headers: {}, status_code: {}, body: {} }}",
            self.0.headers, self.0.status_code, self.0.body
        )
    }
}

impl<Entity: ContentTyped + Default> Default for ListResponse<Entity> {
    fn default() -> Self {
        Self::new(Default::default(), StatusCode::Ok, Default::default())
    }
}

pub struct ShowRequest<Entity: Identifiable>(
    Request<(), EntityIdentificationParameters<Entity>, (), ()>,
);

impl<Entity: Identifiable> ShowRequest<Entity> {
    pub fn new(path_params: EntityIdentificationParameters<Entity>) -> Self {
        Self(Request {
            headers: (),
            path_params,
            query_params: (),
            body: (),
        })
    }
}

impl<Entity: Identifiable> Display for ShowRequest<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShowRequest {{ path_params: {} }}", self.0.path_params)
    }
}

impl<Entity: Identifiable + Default> Default for ShowRequest<Entity> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub struct ShowResponse<Entity: ContentTyped>(
    Response<ContentTypeHeader<Entity>, EntityBody<Entity>>,
);

impl<Entity: ContentTyped> ShowResponse<Entity> {
    pub fn new(
        headers: ContentTypeHeader<Entity>,
        status_code: StatusCode,
        body: EntityBody<Entity>,
    ) -> Self {
        Self(Response {
            headers,
            status_code,
            body,
        })
    }
}

impl<Entity: ContentTyped + Display> Display for ShowResponse<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ShowResponse {{ headers: {}, status_code: {}, body: {} }}",
            self.0.headers, self.0.status_code, self.0.body
        )
    }
}

impl<Entity: ContentTyped + Default> Default for ShowResponse<Entity> {
    fn default() -> Self {
        Self::new(Default::default(), StatusCode::Ok, Default::default())
    }
}

pub struct CreateRequest<Entity: ContentTyped>(
    Request<ContentTypeHeader<Entity>, (), (), EntityBody<Entity>>,
);

impl<Entity: ContentTyped> CreateRequest<Entity> {
    pub fn new(headers: ContentTypeHeader<Entity>, body: EntityBody<Entity>) -> Self {
        Self(Request {
            headers,
            path_params: (),
            query_params: (),
            body,
        })
    }
}

impl<Entity: ContentTyped + Display> Display for CreateRequest<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CreateRequest {{ headers: {}, body: {} }}",
            self.0.headers, self.0.body
        )
    }
}

impl<Entity: ContentTyped + Default> Default for CreateRequest<Entity> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

pub struct CreateResponse<Entity: ContentTyped>(
    Response<ContentTypeHeader<Entity>, EntityBody<Entity>>,
);

impl<Entity: ContentTyped> CreateResponse<Entity> {
    pub fn new(
        headers: ContentTypeHeader<Entity>,
        status_code: StatusCode,
        body: EntityBody<Entity>,
    ) -> Self {
        Self(Response {
            headers,
            status_code,
            body,
        })
    }
}

impl<Entity: ContentTyped + Display> Display for CreateResponse<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CreateResponse {{ headers: {}, status_code: {}, body: {} }}",
            self.0.headers, self.0.status_code, self.0.body
        )
    }
}

impl<Entity: ContentTyped + Default> Default for CreateResponse<Entity> {
    fn default() -> Self {
        Self::new(Default::default(), StatusCode::Created, Default::default())
    }
}

pub struct UpdateRequest<Entity: Identifiable + ContentTyped>(
    Request<
        ContentTypeHeader<Entity>,
        EntityIdentificationParameters<Entity>,
        (),
        EntityBody<Entity>,
    >,
);

impl<Entity: Identifiable + ContentTyped> UpdateRequest<Entity> {
    pub fn new(
        headers: ContentTypeHeader<Entity>,
        path_params: EntityIdentificationParameters<Entity>,
        body: EntityBody<Entity>,
    ) -> Self {
        Self(Request {
            headers,
            path_params,
            query_params: (),
            body,
        })
    }
}

impl<Entity: Identifiable + ContentTyped + Display> Display for UpdateRequest<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UpdateRequest {{ headers: {}, path_params: {}, body: {} }}",
            self.0.headers, self.0.path_params, self.0.body
        )
    }
}

impl<Entity: Identifiable + ContentTyped + Default> Default for UpdateRequest<Entity> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}

pub struct UpdateResponse<Entity: ContentTyped>(
    Response<ContentTypeHeader<Entity>, EntityBody<Entity>>,
);

impl<Entity: ContentTyped> UpdateResponse<Entity> {
    pub fn new(
        headers: ContentTypeHeader<Entity>,
        status_code: StatusCode,
        body: EntityBody<Entity>,
    ) -> Self {
        Self(Response {
            headers,
            status_code,
            body,
        })
    }
}

impl<Entity: ContentTyped + Display> Display for UpdateResponse<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UpdateResponse {{ headers: {}, status_code: {}, body: {} }}",
            self.0.headers, self.0.status_code, self.0.body
        )
    }
}

impl<Entity: ContentTyped + Default> Default for UpdateResponse<Entity> {
    fn default() -> Self {
        Self::new(Default::default(), StatusCode::Ok, Default::default())
    }
}

pub struct DeleteRequest<Entity: Identifiable>(
    Request<(), EntityIdentificationParameters<Entity>, (), ()>,
);

impl<Entity: Identifiable> DeleteRequest<Entity> {
    pub fn new(path_params: EntityIdentificationParameters<Entity>) -> Self {
        Self(Request {
            headers: (),
            path_params,
            query_params: (),
            body: (),
        })
    }
}

impl<Entity: Identifiable> Display for DeleteRequest<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeleteRequest {{ path_params: {} }}", self.0.path_params)
    }
}

impl<Entity: Identifiable + Default> Default for DeleteRequest<Entity> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub struct DeleteResponse(Response<(), ()>);

impl DeleteResponse {
    pub fn new(status_code: StatusCode) -> Self {
        Self(Response {
            headers: (),
            status_code,
            body: (),
        })
    }
}

impl Display for DeleteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeleteResponse {{ status_code: {} }}",
            self.0.status_code
        )
    }
}

#[derive(Default)]
pub struct ContentTypeHeader<Entity: ContentTyped> {
    pub content_type: ContentType,
    _phantom_entity: PhantomData<Entity>,
}

impl<Entity: ContentTyped> Display for ContentTypeHeader<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ContentTypeHeader {{ content_type: {} }}",
            self.content_type
        )
    }
}

#[derive(Default)]
pub struct EntityIdentificationParameters<Entity: Identifiable> {
    pub entity_id: Entity::Id,
}

impl<Entity: Identifiable> Display for EntityIdentificationParameters<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EntityIdentificationParameters {{ entity_id: {} }}",
            self.entity_id
        )
    }
}

#[derive(Default)]
pub struct PagingQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Display for PagingQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PagingQueryParams {{ limit: {:?}, offset: {:?} }}",
            self.limit, self.offset
        )
    }
}

#[derive(Default)]
pub struct EntitiesBody<Entity> {
    pub entities: Vec<Entity>,
}

impl<Entity: Display> Display for EntitiesBody<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.entities.len() {
            0 => write!(f, "EntitiesBody {{ entities: [] }}"),
            1..=3 => write!(
                f,
                "EntitiesBody {{ entities: [{}] }}",
                self.entities
                    .iter()
                    .take(3) // limit to 3 entities
                    .map(|entity| format!("{}", entity))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => write!(
                f,
                "EntitiesBody {{ entities: [{} ... {}] }}",
                self.entities[0],
                self.entities[self.entities.len() - 1]
            ),
        }
    }
}

#[derive(Default)]
pub struct EntityBody<Entity> {
    pub entity: Entity,
}

impl<Entity: Display> Display for EntityBody<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EntityBody {{ entity: {} }}", self.entity)
    }
}
