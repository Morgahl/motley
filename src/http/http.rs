use std::collections::HashMap;

pub trait Request<
    Body,
    Headers = HashMap<String, Vec<String>>,
    PathParameters = HashMap<String, String>,
    QueryParameters = HashMap<String, String>,
>: RequestParameters<Headers, PathParameters, QueryParameters>
{
    fn body(&self) -> &Body;
}

pub trait RequestParameters<
    Headers = HashMap<String, Vec<String>>,
    PathParameters = HashMap<String, String>,
    QueryParameters = HashMap<String, String>,
>
{
    fn headers(&self) -> &Headers;
    fn path_parameters(&self) -> &PathParameters;
    fn query_parameters(&self) -> &QueryParameters;
}

pub trait Response<Body, Headers = HashMap<String, Vec<String>>>:
    ResponseParameters<Headers>
{
    fn body(&self) -> &Body;
}

pub trait ResponseParameters<Headers = HashMap<String, Vec<String>>> {
    fn headers(&self) -> &Headers;
    fn status_code(&self) -> &StatusCode;
}

pub enum StatusCode {
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