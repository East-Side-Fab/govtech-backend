mod models;
mod surreal;

use models::Participant;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_participants(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut sdb = surreal::SurrealDB::builder(
        "https://example-06adv5p8vpuu76550jkpukfr50.aws-use1.surreal.cloud",
    )
    .user("bot")
    .password("0xSuper_Secret_Password1!")
    .namespace("hackathon")
    .database("page")
    .build();

    sdb.signin().await?;

    let query_results = sdb.sql("SELECT * FROM participant").await?;
    let participants: Vec<Participant> = query_results
        .into_iter()
        .map(|value| serde_json::from_value::<Vec<Participant>>(value))
        .collect::<Result<Vec<Vec<Participant>>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(serde_json::to_string(&participants)?)
        .build())
}
