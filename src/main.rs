#[macro_use]
extern crate rocket;

use sea_orm_rocket::Database;

use rocket_okapi::{
    mount_endpoints_and_merged_docs,
    okapi::openapi3::OpenApi,
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig},
    settings::UrlObject,
};

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

mod dto;
mod error;
mod services;
mod pool;
mod post;
mod query;
mod routes;

use pool::Db;

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    let mut building_rocket = rocket::build()
        .attach(Db::init())
        .mount(
            "/apidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("Zeus - Zeus documentation | API Doc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .attach(cors());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    let custom_route_spec = (vec![], custom_openapi_spec());
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
            "/additional" => custom_route_spec,
            "/post" => routes::get_routes_and_docs(&openapi_settings),
    };

    building_rocket.launch().await.map(|_| ())
}

fn cors() -> Cors {
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:8000", "http://127.0.0.1:8000"]);

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}

fn custom_openapi_spec() -> OpenApi {
    use rocket_okapi::okapi::openapi3::*;
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "Zeus".to_owned(),
            description: Some("API Docs for Zeus".to_owned()),
            terms_of_service: Some("https://github.com/bourdeau/zeus#license".to_owned()),
            contact: Some(Contact {
                name: Some("Zeus".to_owned()),
                url: Some("https://github.com/bourdeau/zeus".to_owned()),
                email: None,
                ..Default::default()
            }),
            license: Some(License {
                name: "Apache-2.0".to_owned(),
                url: Some(
                    "https://github.com/bourdeau/zeus/blob/master/LICENSE-APACHE".to_owned(),
                ),
                ..Default::default()
            }),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        },
        servers: vec![Server {
            url: "http://127.0.0.1:8000/".to_owned(),
            description: Some("Localhost".to_owned()),
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
