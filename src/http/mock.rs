use anyhow::Result;
use http::StatusCode;

use super::{Request, Response};

pub type Handler<ReqBody, RespBody> = fn(Request<ReqBody>) -> Result<Response<RespBody>>;

pub trait Route<
    GetReqBody = (),
    GetRespBody = (),
    PostReqBody = (),
    PostRespBody = (),
    PutReqBody = (),
    PutRespBody = (),
    DeleteReqBody = (),
    DeleteRespBody = (),
    PatchReqBody = (),
    PatchRespBody = (),
    HeadReqBody = (),
    HeadRespBody = (),
    OptionsReqBody = (),
    OptionsRespBody = (),
    TraceReqBody = (),
    TraceRespBody = (),
    ConnectReqBody = (),
    ConnectRespBody = (),
> where
    GetReqBody: Default,
    GetRespBody: Default,
    PostReqBody: Default,
    PostRespBody: Default,
    PutReqBody: Default,
    PutRespBody: Default,
    DeleteReqBody: Default,
    DeleteRespBody: Default,
    PatchReqBody: Default,
    PatchRespBody: Default,
    HeadReqBody: Default,
    HeadRespBody: Default,
    OptionsReqBody: Default,
    OptionsRespBody: Default,
    TraceReqBody: Default,
    TraceRespBody: Default,
    ConnectReqBody: Default,
    ConnectRespBody: Default,
{
    const GET: Handler<GetReqBody, GetRespBody> = default_handler;
    const POST: Handler<PostReqBody, PostRespBody> = default_handler;
    const PUT: Handler<PutReqBody, PutRespBody> = default_handler;
    const DELETE: Handler<DeleteReqBody, DeleteRespBody> = default_handler;
    const PATCH: Handler<PatchReqBody, PatchRespBody> = default_handler;
    const HEAD: Handler<HeadReqBody, HeadRespBody> = default_handler;
    const OPTIONS: Handler<OptionsReqBody, OptionsRespBody> = default_handler;
    const TRACE: Handler<TraceReqBody, TraceRespBody> = default_handler;
    const CONNECT: Handler<ConnectReqBody, ConnectRespBody> = default_handler;
}

fn default_handler<ReqBody, RespBody>(_: Request<ReqBody>) -> Result<Response<RespBody>>
where
    RespBody: Default,
{
    Response::<RespBody>::builder()
        .status(StatusCode::NOT_FOUND)
        .body(RespBody::default())
}

pub trait StaticRoute: Route {
    const PATH: &'static str;
}

pub trait MatchingRoute: Route {
    const ID_NAME: &'static str;
}
