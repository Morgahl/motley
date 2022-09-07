pub mod rest;

use std::fmt::Display;

use anyhow::{bail, Error, Result};

#[derive(Debug)]
pub struct Request<Headers, PathParams, QueryParams, Body> {
    pub headers: Headers,
    pub path_params: PathParams,
    pub query_params: QueryParams,
    pub body: Body,
}

#[derive(Debug)]
pub struct Response<Headers, Body> {
    pub headers: Headers,
    pub status_code: StatusCode,
    pub body: Body,
}

pub trait ContentTyped {
    const CONTENT_TYPE: ContentType;
}

#[derive(Debug)]
pub enum ContentType {
    Unknown,
    PlainText,
    HTML,
    XML,
    JSON,
    OctetStream,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl TryFrom<&str> for ContentType {
    type Error = Error;
    fn try_from(content_type: &str) -> Result<Self> {
        match content_type {
            "text/plain" => Ok(ContentType::PlainText),
            "text/html" => Ok(ContentType::HTML),
            "text/xml" => Ok(ContentType::XML),
            "application/json" => Ok(ContentType::JSON),
            "application/octet-stream" => Ok(ContentType::OctetStream),
            _ => bail!("Unknown content type: {}", content_type),
        }
    }
}

impl TryFrom<String> for ContentType {
    type Error = Error;
    fn try_from(content_type: String) -> Result<Self> {
        Self::try_from(content_type.as_str())
    }
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Unknown => "".to_string(),
            ContentType::PlainText => "text/plain".to_string(),
            ContentType::HTML => "text/html".to_string(),
            ContentType::XML => "text/xml".to_string(),
            ContentType::JSON => "application/json".to_string(),
            ContentType::OctetStream => "application/octet-stream".to_string(),
        }
    }
}

pub enum StatusCodeCategory {
    Unknown,
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
}

impl PartialEq<StatusCode> for StatusCodeCategory {
    fn eq(&self, other: &StatusCode) -> bool {
        match (self, other) {
            (StatusCodeCategory::Unknown, StatusCode::Unknown) => true,

            // 1xx Informational
            (StatusCodeCategory::Informational, StatusCode::Continue) => true,
            (StatusCodeCategory::Informational, StatusCode::SwitchingProtocols) => true,
            (StatusCodeCategory::Informational, StatusCode::Processing) => true,
            (StatusCodeCategory::Informational, StatusCode::EarlyHints) => true,

            // 2xx Success
            (StatusCodeCategory::Success, StatusCode::Ok) => true,
            (StatusCodeCategory::Success, StatusCode::Created) => true,
            (StatusCodeCategory::Success, StatusCode::Accepted) => true,
            (StatusCodeCategory::Success, StatusCode::NonAuthoritativeInformation) => true,
            (StatusCodeCategory::Success, StatusCode::NoContent) => true,
            (StatusCodeCategory::Success, StatusCode::ResetContent) => true,
            (StatusCodeCategory::Success, StatusCode::PartialContent) => true,
            (StatusCodeCategory::Success, StatusCode::MultiStatus) => true,
            (StatusCodeCategory::Success, StatusCode::AlreadyReported) => true,
            (StatusCodeCategory::Success, StatusCode::IMUsed) => true,

            // 3xx Redirection
            (StatusCodeCategory::Redirection, StatusCode::MultipleChoices) => true,
            (StatusCodeCategory::Redirection, StatusCode::MovedPermanently) => true,
            (StatusCodeCategory::Redirection, StatusCode::Found) => true,
            (StatusCodeCategory::Redirection, StatusCode::SeeOther) => true,
            (StatusCodeCategory::Redirection, StatusCode::NotModified) => true,
            (StatusCodeCategory::Redirection, StatusCode::UseProxy) => true,
            (StatusCodeCategory::Redirection, StatusCode::TemporaryRedirect) => true,
            (StatusCodeCategory::Redirection, StatusCode::PermanentRedirect) => true,

            // 4xx Client Error
            (StatusCodeCategory::ClientError, StatusCode::BadRequest) => true,
            (StatusCodeCategory::ClientError, StatusCode::Unauthorized) => true,
            (StatusCodeCategory::ClientError, StatusCode::Forbidden) => true,
            (StatusCodeCategory::ClientError, StatusCode::NotFound) => true,
            (StatusCodeCategory::ClientError, StatusCode::MethodNotAllowed) => true,
            (StatusCodeCategory::ClientError, StatusCode::NotAcceptable) => true,
            (StatusCodeCategory::ClientError, StatusCode::Conflict) => true,
            (StatusCodeCategory::ClientError, StatusCode::Gone) => true,
            (StatusCodeCategory::ClientError, StatusCode::LengthRequired) => true,
            (StatusCodeCategory::ClientError, StatusCode::PreconditionFailed) => true,
            (StatusCodeCategory::ClientError, StatusCode::PayloadTooLarge) => true,
            (StatusCodeCategory::ClientError, StatusCode::URITooLong) => true,
            (StatusCodeCategory::ClientError, StatusCode::UnsupportedMediaType) => true,
            (StatusCodeCategory::ClientError, StatusCode::RangeNotSatisfiable) => true,
            (StatusCodeCategory::ClientError, StatusCode::ExpectationFailed) => true,
            (StatusCodeCategory::ClientError, StatusCode::ImATeapot) => true,
            (StatusCodeCategory::ClientError, StatusCode::EnhanceYourCalm) => true,
            (StatusCodeCategory::ClientError, StatusCode::MisdirectedRequest) => true,
            (StatusCodeCategory::ClientError, StatusCode::UnprocessableEntity) => true,
            (StatusCodeCategory::ClientError, StatusCode::Locked) => true,
            (StatusCodeCategory::ClientError, StatusCode::FailedDependency) => true,
            (StatusCodeCategory::ClientError, StatusCode::TooEarly) => true,
            (StatusCodeCategory::ClientError, StatusCode::UpgradeRequired) => true,
            (StatusCodeCategory::ClientError, StatusCode::PreconditionRequired) => true,
            (StatusCodeCategory::ClientError, StatusCode::TooManyRequests) => true,
            (StatusCodeCategory::ClientError, StatusCode::RequestHeaderFieldsTooLarge) => true,
            (StatusCodeCategory::ClientError, StatusCode::UnavailableForLegalReasons) => true,

            // 5xx Server Error
            (StatusCodeCategory::ServerError, StatusCode::InternalServerError) => true,
            (StatusCodeCategory::ServerError, StatusCode::NotImplemented) => true,
            (StatusCodeCategory::ServerError, StatusCode::BadGateway) => true,
            (StatusCodeCategory::ServerError, StatusCode::ServiceUnavailable) => true,
            (StatusCodeCategory::ServerError, StatusCode::GatewayTimeout) => true,
            (StatusCodeCategory::ServerError, StatusCode::HTTPVersionNotSupported) => true,
            (StatusCodeCategory::ServerError, StatusCode::VariantAlsoNegotiates) => true,
            (StatusCodeCategory::ServerError, StatusCode::InsufficientStorage) => true,
            (StatusCodeCategory::ServerError, StatusCode::LoopDetected) => true,
            (StatusCodeCategory::ServerError, StatusCode::NotExtended) => true,
            (StatusCodeCategory::ServerError, StatusCode::NetworkAuthenticationRequired) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum StatusCode {
    // exists only as a default value
    Unknown,

    // 1xx Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    // 2xx Success
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    // 3xx Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // 4xx Client Error
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    EnhanceYourCalm = 420,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    // 5xx Server Error
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl StatusCode {
    pub fn is_unknown(&self) -> bool {
        *self == StatusCodeCategory::Unknown
    }

    pub fn is_informational(&self) -> bool {
        *self == StatusCodeCategory::Informational
    }

    pub fn is_success(&self) -> bool {
        *self == StatusCodeCategory::Success
    }

    pub fn is_redirection(&self) -> bool {
        *self == StatusCodeCategory::Redirection
    }

    pub fn is_client_error(&self) -> bool {
        *self == StatusCodeCategory::ClientError
    }

    pub fn is_server_error(&self) -> bool {
        *self == StatusCodeCategory::ServerError
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Unknown
    }
}

impl From<u16> for StatusCode {
    fn from(status_code: u16) -> Self {
        match status_code {
            // 1xx Informational
            100 => StatusCode::Continue,
            101 => StatusCode::SwitchingProtocols,
            102 => StatusCode::Processing,
            103 => StatusCode::EarlyHints,

            // 2xx Success
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            203 => StatusCode::NonAuthoritativeInformation,
            204 => StatusCode::NoContent,
            205 => StatusCode::ResetContent,
            206 => StatusCode::PartialContent,
            207 => StatusCode::MultiStatus,
            208 => StatusCode::AlreadyReported,
            226 => StatusCode::IMUsed,

            // 3xx Redirection
            300 => StatusCode::MultipleChoices,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::Found,
            303 => StatusCode::SeeOther,
            304 => StatusCode::NotModified,
            305 => StatusCode::UseProxy,
            307 => StatusCode::TemporaryRedirect,
            308 => StatusCode::PermanentRedirect,

            // 4xx Client Error
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            406 => StatusCode::NotAcceptable,
            409 => StatusCode::Conflict,
            410 => StatusCode::Gone,
            411 => StatusCode::LengthRequired,
            412 => StatusCode::PreconditionFailed,
            413 => StatusCode::PayloadTooLarge,
            414 => StatusCode::URITooLong,
            415 => StatusCode::UnsupportedMediaType,
            416 => StatusCode::RangeNotSatisfiable,
            417 => StatusCode::ExpectationFailed,
            418 => StatusCode::ImATeapot,
            420 => StatusCode::EnhanceYourCalm,
            421 => StatusCode::MisdirectedRequest,
            422 => StatusCode::UnprocessableEntity,
            423 => StatusCode::Locked,
            424 => StatusCode::FailedDependency,
            425 => StatusCode::TooEarly,
            426 => StatusCode::UpgradeRequired,
            428 => StatusCode::PreconditionRequired,
            429 => StatusCode::TooManyRequests,
            431 => StatusCode::RequestHeaderFieldsTooLarge,
            451 => StatusCode::UnavailableForLegalReasons,

            // 5xx Server Error
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HTTPVersionNotSupported,
            506 => StatusCode::VariantAlsoNegotiates,
            507 => StatusCode::InsufficientStorage,
            508 => StatusCode::LoopDetected,
            510 => StatusCode::NotExtended,
            511 => StatusCode::NetworkAuthenticationRequired,

            // Unknown
            _ => StatusCode::Unknown,
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Unknown => write!(f, "Unknown(???)"),

            // 1xx Informational
            StatusCode::Continue => write!(f, "Continue(100)"),
            StatusCode::SwitchingProtocols => write!(f, "SwitchingProtocols(101)"),
            StatusCode::Processing => write!(f, "Processing(102)"),
            StatusCode::EarlyHints => write!(f, "EarlyHints(103)"),

            // 2xx Success
            StatusCode::Ok => write!(f, "Ok(200)"),
            StatusCode::Created => write!(f, "Created(201)"),
            StatusCode::Accepted => write!(f, "Accepted(202)"),
            StatusCode::NonAuthoritativeInformation => {
                write!(f, "NonAuthoritativeInformation(203)")
            }
            StatusCode::NoContent => write!(f, "NoContent(204)"),
            StatusCode::ResetContent => write!(f, "ResetContent(205)"),
            StatusCode::PartialContent => write!(f, "PartialContent(206)"),
            StatusCode::MultiStatus => write!(f, "MultiStatus(207)"),
            StatusCode::AlreadyReported => write!(f, "AlreadyReported(208)"),
            StatusCode::IMUsed => write!(f, "IMUsed(226)"),

            // 3xx Redirection
            StatusCode::MultipleChoices => write!(f, "MultipleChoices(300)"),
            StatusCode::MovedPermanently => write!(f, "MovedPermanently(301)"),
            StatusCode::Found => write!(f, "Found(302)"),
            StatusCode::SeeOther => write!(f, "SeeOther(303)"),
            StatusCode::NotModified => write!(f, "NotModified(304)"),
            StatusCode::UseProxy => write!(f, "UseProxy(305)"),
            StatusCode::TemporaryRedirect => write!(f, "TemporaryRedirect(307)"),
            StatusCode::PermanentRedirect => write!(f, "PermanentRedirect(308)"),

            // 4xx Client Error
            StatusCode::BadRequest => write!(f, "BadRequest(400)"),
            StatusCode::Unauthorized => write!(f, "Unauthorized(401)"),
            StatusCode::Forbidden => write!(f, "Forbidden(403)"),
            StatusCode::NotFound => write!(f, "NotFound(404)"),
            StatusCode::MethodNotAllowed => write!(f, "MethodNotAllowed(405)"),
            StatusCode::NotAcceptable => write!(f, "NotAcceptable(406)"),
            StatusCode::Conflict => write!(f, "Conflict(409)"),
            StatusCode::Gone => write!(f, "Gone(410)"),
            StatusCode::LengthRequired => write!(f, "LengthRequired(411)"),
            StatusCode::PreconditionFailed => write!(f, "PreconditionFailed(412)"),
            StatusCode::PayloadTooLarge => write!(f, "PayloadTooLarge(413)"),
            StatusCode::URITooLong => write!(f, "URITooLong(414)"),
            StatusCode::UnsupportedMediaType => write!(f, "UnsupportedMediaType(415)"),
            StatusCode::RangeNotSatisfiable => write!(f, "RangeNotSatisfiable(416)"),
            StatusCode::ExpectationFailed => write!(f, "ExpectationFailed(417)"),
            StatusCode::ImATeapot => write!(f, "ImATeapot(418)"),
            StatusCode::EnhanceYourCalm => write!(f, "EnhanceYourCalm(420)"),
            StatusCode::MisdirectedRequest => write!(f, "MisdirectedRequest(421)"),
            StatusCode::UnprocessableEntity => write!(f, "UnprocessableEntity(422)"),
            StatusCode::Locked => write!(f, "Locked(423)"),
            StatusCode::FailedDependency => write!(f, "FailedDependency(424)"),
            StatusCode::TooEarly => write!(f, "TooEarly(425)"),
            StatusCode::UpgradeRequired => write!(f, "UpgradeRequired(426)"),
            StatusCode::PreconditionRequired => write!(f, "PreconditionRequired(428)"),
            StatusCode::TooManyRequests => write!(f, "TooManyRequests(429)"),
            StatusCode::RequestHeaderFieldsTooLarge => {
                write!(f, "RequestHeaderFieldsTooLarge(431)")
            }
            StatusCode::UnavailableForLegalReasons => {
                write!(f, "UnavailableForLegalReasons(451)")
            }

            // 5xx Server Error
            StatusCode::InternalServerError => write!(f, "InternalServerError(500)"),
            StatusCode::NotImplemented => write!(f, "NotImplemented(501)"),
            StatusCode::BadGateway => write!(f, "BadGateway(502)"),
            StatusCode::ServiceUnavailable => write!(f, "ServiceUnavailable(503)"),
            StatusCode::GatewayTimeout => write!(f, "GatewayTimeout(504)"),
            StatusCode::HTTPVersionNotSupported => write!(f, "HTTPVersionNotSupported(505)"),
            StatusCode::VariantAlsoNegotiates => write!(f, "VariantAlsoNegotiates(506)"),
            StatusCode::InsufficientStorage => write!(f, "InsufficientStorage(507)"),
            StatusCode::LoopDetected => write!(f, "LoopDetected(508)"),
            StatusCode::NotExtended => write!(f, "NotExtended(510)"),
            StatusCode::NetworkAuthenticationRequired => {
                write!(f, "NetworkAuthenticationRequired(511)")
            }
        }
    }
}

impl PartialEq<StatusCodeCategory> for StatusCode {
    fn eq(&self, other: &StatusCodeCategory) -> bool {
        other.eq(self)
    }
}
