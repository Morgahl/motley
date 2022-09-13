use std::fmt::{Debug, Display};

use anyhow::Result;

use http::StatusCode;
use motley::http::rest2::mock::{
    MockCreateResource, MockDeleteResource, MockListResource, MockShowResource, MockUpdateResource,
    Resource,
};
use motley::http::rest2::{
    CreateResponse, DeleteResponse, ListResponse, ShowResponse, UpdateResponse,
};
use motley::http::{ContentType, ContentTyped};
use motley::Identifiable;

fn main() -> Result<()> {
    let task_resource = Resource::<Task>::new(
        "/tasks".to_string(),
        "task_id".to_string(),
        MockListResource::Response(|| {
            ListResponse::new(Default::default(), StatusCode::OK, Default::default())
        }),
        MockShowResource::Response(|| {
            ShowResponse::new(Default::default(), StatusCode::OK, Default::default())
        }),
        MockCreateResource::Response(|| {
            CreateResponse::new(Default::default(), StatusCode::CREATED, Default::default())
        }),
        MockUpdateResource::Response(|| {
            UpdateResponse::new(Default::default(), StatusCode::OK, Default::default())
        }),
        MockDeleteResource::Response(|| DeleteResponse::new(StatusCode::NO_CONTENT)),
    );
    task_resource.inspect_resource()
}

#[derive(Default, Debug)]
struct Task {
    id: u32,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task {{ id: {} }}", self.id)
    }
}

impl Identifiable for Task {
    type Id = u32;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl ContentTyped for Task {
    const CONTENT_TYPE: ContentType = ContentType::JSON;
}
