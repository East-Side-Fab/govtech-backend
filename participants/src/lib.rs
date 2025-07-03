mod models;
mod surreal;

use anyhow::anyhow;
use base64::prelude::*;
use models::Participant;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response, Router};
use spin_sdk::{http_component, variables};

#[http_component]
fn handle_participants(request: Request) -> impl IntoResponse {
    let mut router = Router::new();
    router.add_async("/participants", Method::Get, handle_get_participants);
    router.add_async("/participants", Method::Post, handle_create_participant);
    router.add_async("/participants", Method::Options, handle_options);
    router.handle(request)
}

async fn handle_get_participants(request: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let admin_name = variables::get("admin_name")?;
    let admin_password = variables::get("admin_password")?;
    if !verify_auth(&request, &admin_name, &admin_password) {
        let response = Response::builder().status(401).build();
        return Ok(response);
    }
    let participants = get_participants().await?;
    let response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&participants)?)
        .build();
    Ok(response)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateParticipantRequest {
    pub mail: String,
    pub first_name: String,
    pub last_name: String,
    pub organisation: Option<String>,
}

async fn handle_create_participant(r: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let payload = serde_json::from_slice::<CreateParticipantRequest>(r.body())?;

    let participant = create_participant(payload).await?;

    let response = Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .header(
            "Access-Control-Allow-Origin",
            "https://hackathon.govtech.saarland",
        )
        .header("Access-Control-Allow-Methods", "POST")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type, application/json",
        )
        .header("Access-Control-Allow-Credentials", "true")
        .body(serde_json::to_string(&participant)?)
        .build();
    Ok(response)
}

async fn get_participants() -> anyhow::Result<Vec<Participant>> {
    let mut sdb = surreal::SurrealDB::builder(&variables::get("host")?)
        .user(&variables::get("user")?)
        .password(&variables::get("password")?)
        .namespace(&variables::get("namespace")?)
        .database(&variables::get("database")?)
        .build();

    sdb.signin().await?;

    let query_results = sdb.sql("SELECT * FROM participant").await?;
    let participants: Vec<Participant> = query_results
        .into_iter()
        .map(serde_json::from_value::<Vec<Participant>>)
        .collect::<Result<Vec<Vec<Participant>>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(participants)
}

async fn handle_options(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let response = Response::builder()
        .status(204)
        .header(
            "Access-Control-Allow-Origin",
            "https://hackathon.govtech.saarland",
        )
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        )
        .header("Access-Control-Allow-Credentials", "true")
        .body(vec![])
        .build();
    Ok(response)
}

async fn create_participant(payload: CreateParticipantRequest) -> anyhow::Result<Participant> {
    let mut sdb = surreal::SurrealDB::builder(&variables::get("host")?)
        .user(&variables::get("user")?)
        .password(&variables::get("password")?)
        .namespace(&variables::get("namespace")?)
        .database(&variables::get("database")?)
        .build();

    sdb.signin().await?;

    let data = serde_json::json!({
        "mail": payload.mail,
        "name": {
            "first": payload.first_name,
            "last": payload.last_name,
        },
        "organisation": payload.organisation
    });

    let query = format!(
        "CREATE participant CONTENT {}",
        serde_json::to_string(&data)?
    );

    let query_results = sdb.sql(&query).await?;
    let participant: Option<Participant> = query_results
        .into_iter()
        .map(serde_json::from_value::<Vec<Participant>>)
        .collect::<Result<Vec<Vec<Participant>>, _>>()?
        .into_iter()
        .flatten()
        .last();

    participant.ok_or(anyhow!("Couldn't create participant"))
}

fn verify_auth(req: &Request, user: &str, password: &str) -> bool {
    let Some(auth) = req
        .header("authorization")
        .and_then(|v| v.as_str())
        .and_then(|v| v.split(" ").skip(1).next())
        .and_then(|v| BASE64_STANDARD.decode(v).ok())
    else {
        return false;
    };

    format!("{}:{}", user, password)
        .as_bytes()
        .iter()
        .zip(auth.iter())
        .all(|(&a, &b)| a == b)
}
