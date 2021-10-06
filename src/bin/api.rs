#![deny(warnings)]
use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    IntoResponse, Request, Response,
};
use log::{error, LevelFilter};
use simple_logger::SimpleLogger;
use std::env;

extern crate line_bot_sdk_rust as line;
use line::bot::LineBot;

use qq_sodan_doko::model::error::ApplicationError;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

fn api(r: &Request) -> Result<(String, &'static str), ApplicationError> {
    let content_type_json = "application/json";
    match r.uri().path() {
        "/line_callback" => {
            use qq_sodan_doko::application::line_callback::main;

            // use qq_sodan_doko::application::line_callback::{main, Event};
            let bot = LineBot::new(
                env::var("LINE_BOT_SECRET").unwrap().as_str(),
                env::var("LINE_BOT_ACCESS_TOKEN").unwrap().as_str(),
            );
            let signature = r
                .headers()
                .get("X-Line-Signature")
                .unwrap()
                .to_str()
                .unwrap();
            let events = bot
                .parse_event_request(signature, std::str::from_utf8(r.body()).unwrap())
                .unwrap();
            for event in events.events {
                main(&bot, event);
            }

            Ok(("".to_string(), content_type_json))
        }
        _ => Err(ApplicationError::NoImplement(format!(
            "no such method {}",
            r.uri().path()
        ))),
    }
}

async fn func(r: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let res = api(&r);
    match res {
        Ok((res, content_type)) => Ok(Response::builder()
            .status(200)
            .header("Content-Type", content_type)
            .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
            .header("Access-Control-Allow-Credential", "true")
            .header(
                "Access-Control-Allow-Origin",
                match r.headers().get("Origin") {
                    Some(o) => o.to_str().unwrap(),
                    None => "",
                },
            )
            .header(
                "Access-Control-Allow-Headers",
                "Origin,Authorization,Accept,X-Requested-With",
            )
            .body(res)?),
        Err(err) => {
            error!("{}", err);
            Ok(Response::builder()
                .status(match err {
                    ApplicationError::NotFound(_) => 404,
                    ApplicationError::PermissionDenied => 403,
                    ApplicationError::Authentication => 401,
                    _ => 500,
                })
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
                .header("Access-Control-Allow-Credential", "true")
                .header(
                    "Access-Control-Allow-Origin",
                    match r.headers().get("Origin") {
                        Some(o) => o.to_str().unwrap(),
                        None => "",
                    },
                )
                .header(
                    "Access-Control-Allow-Headers",
                    "Origin,Authorization,Accept,X-Requested-With",
                )
                .body(err.to_string())?)
        }
    }
}
