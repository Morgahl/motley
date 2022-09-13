use anyhow::{bail, Result};
use http::{Method, Request, Response, StatusCode};

fn main() -> Result<()> {
    let api = Api::listen("localhost".to_string(), 8080)?;
    for _ in 0..10 {
        let request = Request::builder()
            .method(Method::GET)
            .uri("http://localhost:8080/users/1/settings/2")
            .body(())?;
        let response = api.serve::<(), ()>(&request)?;
        println!("{:?} {:?}", request, response);
    }
    Ok(())
}

pub struct Api {
    host: String,
    port: u16,

    users: Users,
    posts: Posts,
}

impl Api {
    #[inline]
    pub fn listen(host: String, port: u16) -> Result<Api> {
        Ok(Api {
            host,
            port,
            users: Users::new(),
            posts: Posts::new(),
        })
    }

    #[inline]
    pub fn serve<ReqBody, RespBody>(&self, request: &Request<ReqBody>) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        let mut path_iter = request.uri().path().split('/').skip(1);
        match path_iter.next() {
            Some("users") => self.users.route(&mut path_iter, request),
            Some("posts") => self.posts.route(&mut path_iter, request),
            _ => not_found(),
        }
    }
}

pub struct Users {
    settings: UserSettings,
}

impl Users {
    #[inline]
    pub fn new() -> Users {
        Users {
            settings: UserSettings::new(),
        }
    }

    #[inline]
    pub fn route<'a, ReqBody, RespBody, I: Iterator<Item = &'a str>>(
        &self,
        path_iter: &mut I,
        request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        match path_iter.next() {
            Some(user_id) => match request.method() {
                &Method::GET => self.show(user_id, request),
                &Method::PUT => self.update(user_id, request),
                &Method::DELETE => self.delete(user_id, request),
                _ => match path_iter.next() {
                    Some("settings") => self.settings.route(user_id, path_iter, request),
                    _ => method_not_allowed(),
                },
            },
            None => match request.method() {
                &Method::GET => self.list(request),
                &Method::POST => self.create(request),
                _ => method_not_allowed(),
            },
        }
    }

    #[inline]
    fn list<ReqBody, RespBody>(&self, _request: &Request<ReqBody>) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn show<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn create<ReqBody, RespBody>(&self, _request: &Request<ReqBody>) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn update<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn delete<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(RespBody::default())
            .unwrap())
    }
}

pub struct Posts;

impl Posts {
    #[inline]
    pub fn new() -> Posts {
        Posts
    }

    #[inline]
    pub fn route<'a, ReqBody, RespBody, I: Iterator<Item = &'a str>>(
        &self,
        path_iter: &mut I,
        request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        match path_iter.next() {
            Some(post_id) => match request.method() {
                &Method::GET => self.show(post_id, request),
                &Method::PUT => self.update(post_id, request),
                &Method::DELETE => self.delete(post_id, request),
                _ => method_not_allowed(),
            },
            None => match request.method() {
                &Method::GET => self.list(request),
                &Method::POST => self.create(request),
                _ => method_not_allowed(),
            },
        }
    }

    #[inline]
    fn list<ReqBody, RespBody>(&self, _request: &Request<ReqBody>) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn show<ReqBody, RespBody>(
        &self,
        _post_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn create<ReqBody, RespBody>(&self, _request: &Request<ReqBody>) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn update<ReqBody, RespBody>(
        &self,
        _post_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn delete<ReqBody, RespBody>(
        &self,
        _post_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(RespBody::default())
            .unwrap())
    }
}

pub struct UserSettings;

impl UserSettings {
    #[inline]
    pub fn new() -> UserSettings {
        UserSettings
    }

    #[inline]
    pub fn route<'a, ReqBody, RespBody, I: Iterator<Item = &'a str>>(
        &self,
        user_id: &str,
        path_iter: &mut I,
        request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        match path_iter.next() {
            Some(setting_id) => match request.method() {
                &Method::GET => self.show(user_id, setting_id, request),
                &Method::PUT => self.update(user_id, setting_id, request),
                &Method::DELETE => self.delete(user_id, setting_id, request),
                _ => method_not_allowed(),
            },
            None => match request.method() {
                &Method::GET => self.list(user_id, request),
                &Method::POST => self.create(user_id, request),
                _ => method_not_allowed(),
            },
        }
    }

    #[inline]
    fn list<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn show<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _setting_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn create<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn update<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _setting_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(RespBody::default())
            .unwrap())
    }

    #[inline]
    fn delete<ReqBody, RespBody>(
        &self,
        _user_id: &str,
        _setting_id: &str,
        _request: &Request<ReqBody>,
    ) -> Result<Response<RespBody>>
    where
        RespBody: Default,
    {
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(RespBody::default())
            .unwrap())
    }
}

#[inline]
fn method_not_allowed<RespBody>() -> Result<Response<RespBody>>
where
    RespBody: Default,
{
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(RespBody::default())
        .unwrap())
}

#[inline]
fn not_found<RespBody>() -> Result<Response<RespBody>>
where
    RespBody: Default,
{
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(RespBody::default())
        .unwrap())
}
