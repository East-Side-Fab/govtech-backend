mod surreal;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_participants(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut sdb = surreal::SurrealDB::builder("https://example-06adv5p8vpuu76550jkpukfr50.aws-use1.surreal.cloud")
        .user("bot")
        .password("0xSuper_Secret_Password1!")
        .namespace("hackathon")
        .database("page")
        .build();

    sdb.signin().await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello World!")
        .build())
}
