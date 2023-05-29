use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::{
        openapi3::Responses,
        schemars::{self, Map},
    },
    response::OpenApiResponderInner,
    OpenApiError,
};

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct Error {
    pub err: String,
    pub msg: Option<String>,
    #[serde(skip)]
    pub http_status_code: u16,
}

impl OpenApiResponderInner for Error {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "Bad Request".to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "Not Found".to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "422".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "Unprocessable Entity".to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "Internal Server Error".to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Error `{}`: {}",
            self.err,
            self.msg.as_deref().unwrap_or("<no message>")
        )
    }
}

impl std::error::Error for Error {}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(self.http_status_code))
            .ok()
    }
}

impl From<rocket::serde::json::Error<'_>> for Error {
    fn from(err: rocket::serde::json::Error) -> Self {
        use rocket::serde::json::Error::*;
        match err {
            Io(io_error) => Error {
                err: "IO Error".to_owned(),
                msg: Some(io_error.to_string()),
                http_status_code: 422,
            },
            Parse(_raw_data, parse_error) => Error {
                err: "Parse Error".to_owned(),
                msg: Some(parse_error.to_string()),
                http_status_code: 422,
            },
        }
    }
}
