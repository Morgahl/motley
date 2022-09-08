use std::fmt::{Debug, Display};

use anyhow::Result;

use motley::http::rest::mock::{Mocked, Resource};
use motley::http::rest::{
    CreateRequest, DeleteRequest, ListRequest, Resource as RESTResource, ShowRequest, UpdateRequest,
};
use motley::http::{ContentType, ContentTyped, StatusCode};
use motley::Identifiable;

fn main() -> Result<()> {
    let task_resource = Resource::<Task>::new(
        "/tasks".to_string(),
        "task_id".to_string(),
        Some(StatusCode::Ok),
        Some(StatusCode::Ok),
        Some(StatusCode::Created),
        Some(StatusCode::Ok),
        Some(StatusCode::NoContent),
    );
    inspect_resource(task_resource)
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

impl Mocked for Task {}

pub fn inspect_resource<Entity: Identifiable + ContentTyped + Default + Debug + Display + Mocked>(
    resource: Resource<Entity>,
) -> Result<()>
where
    Resource<Entity>: RESTResource<Entity>,
{
    println!("{}", resource);
    println!(
        "GET {}\n{}",
        resource.list_path(),
        resource.list(ListRequest::default())?,
    );
    println!(
        "GET {}\n{}",
        resource.show_path(),
        resource.show(ShowRequest::default())?,
    );
    println!(
        "POST {}\n{}",
        resource.create_path(),
        resource.create(CreateRequest::default())?,
    );
    println!(
        "PUT {}\n{}",
        resource.update_path(),
        resource.update(UpdateRequest::default())?,
    );
    println!(
        "DELETE {}\n{}",
        resource.delete_path(),
        resource.delete(DeleteRequest::default())?,
    );
    Ok(())
}
