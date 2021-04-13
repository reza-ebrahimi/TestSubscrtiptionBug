use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, Object, Schema, Subscription,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use futures::prelude::*;
use std::pin::Pin;
use std::time::Duration;

struct TestInterval {
    interval: Option<tokio::time::Interval>,
}

impl TestInterval {
    pub fn new(millis: u64) -> Self {
        Self {
            interval: Some(tokio::time::interval(Duration::from_millis(millis))),
        }
    }
}

impl Stream for TestInterval {
    type Item = Option<i32>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        println!("[Stream] TestInterval");
        if self.interval.as_mut().unwrap().poll_tick(cx) == std::task::Poll::Pending {
            return std::task::Poll::Pending;
        }
        std::task::Poll::Ready(Some(Some(0)))
    }
}

impl Drop for TestInterval {
    fn drop(&mut self) {
        if let Some(interval) = self.interval.take() {
            drop(interval);
        }

        println!("[DROP] TestInterval");
    }
}

#[derive(Clone, Default)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn int_val(&self) -> i32 {
        0
    }
}

#[derive(Clone, Default)]
pub struct StatSubscription;

#[Subscription]
impl StatSubscription {
    async fn stat(&self, _ctx: &Context<'_>) -> impl Stream<Item = Option<i32>> {
        TestInterval::new(1000)
    }
}

type TestSchema = Schema<QueryRoot, EmptyMutation, StatSubscription>;

async fn graphql_handler(
    schema: web::Data<TestSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

async fn playground_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

async fn graphql_post_handle(schema: web::Data<TestSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(
        QueryRoot::default(),
        EmptyMutation,
        StatSubscription::default(),
    )
    .finish();

    println!("Playground: http://localhost:8001");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(graphql_handler),
            )
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(graphql_post_handle),
            )
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(playground_handler),
            )
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
