use anyhow::Result;

use motley::{
    entity::Identifiable,
    http::{
        http::{Request, Response},
        resource::{
            Create, CreateRequestBody, CreateResponseBody, Delete, DeleteRequestBody,
            DeleteResponseBody, List, ListRequestBody, ListResponseBody, Resource, Show,
            ShowRequestBody, ShowResponseBody, Update, UpdateRequestBody, UpdateResponseBody,
        },
    },
};

fn main() -> Result<()> {
    Ok(())
}

enum Routes {
    Tasks(Task),
}

struct Task {
    id: u32,
}

impl Identifiable for Task {
    type Id = u32;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Resource<Task> for Task {
    const PATH: String = "/tasks".to_string();
}

impl List<Task> for Task {
    type Request = dyn Request<ListRequestBody<Task>>;
    type Response = dyn Response<ListResponseBody<Task>>;

    fn list(&self, request: &Self::Request) -> &Self::Response {
        todo!()
    }
}

impl Show<Task> for Task {
    type Request = dyn Request<ShowRequestBody<Task>>;
    type Response = dyn Response<ShowResponseBody<Task>>;

    fn show(&self, request: &Self::Request) -> &Self::Response {
        todo!()
    }
}

impl Create<Task> for Task {
    type Request = dyn Request<CreateRequestBody<Task>>;
    type Response = dyn Response<CreateResponseBody<Task>>;

    fn create(&self, request: &Self::Request) -> &Self::Response {
        todo!()
    }
}

impl Update<Task> for Task {
    type Request = dyn Request<UpdateRequestBody<Task>>;
    type Response = dyn Response<UpdateResponseBody<Task>>;

    fn update(&self, request: &Self::Request) -> &Self::Response {
        todo!()
    }
}

impl Delete<Task> for Task {
    type Request = dyn Request<DeleteRequestBody<Task>>;
    type Response = dyn Response<DeleteResponseBody<Task>>;

    fn delete(&self, request: &Self::Request) -> &Self::Response {
        todo!()
    }
}
