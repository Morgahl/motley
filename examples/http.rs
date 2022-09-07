use anyhow::Result;

use motley::http::rest::mock::{
    CreateMock, DeleteMock, ListMock, MockResource, ShowMock, UpdateMock,
};
use motley::http::rest::{
    Create, CreateRequest, Delete, EntityRequest, List, ListRequest, Show, Update, UpdateRequest,
};
use motley::http::{ContentType, ContentTyped};
use motley::Identifiable;

fn main() -> Result<()> {
    let task_resource = MockResource::<Task>::new("/tasks".to_string());
    println!("{}", task_resource);
    println!("{:?}", task_resource);
    println!("{:#?}", task_resource);
    println!(
        "[{}]:\n{:#?}",
        task_resource.list_path(),
        task_resource.list(ListRequest::default())?
    );
    println!(
        "[{}]:\n{:#?}",
        task_resource.show_path(),
        task_resource.show(EntityRequest::default())?
    );
    println!(
        "[{}]:\n{:#?}",
        task_resource.create_path(),
        task_resource.create(CreateRequest::default())?
    );
    println!(
        "[{}]:\n{:#?}",
        task_resource.update_path(),
        task_resource.update(UpdateRequest::default())?
    );
    println!(
        "[{}]:\n{:#?}",
        task_resource.delete_path(),
        task_resource.delete(EntityRequest::default())?
    );
    Ok(())
}

#[derive(Default, Debug)]
struct Task {
    id: u32,
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

impl ListMock for Task {}

impl ShowMock for Task {}

impl CreateMock for Task {}

impl UpdateMock for Task {}

impl DeleteMock for Task {}
