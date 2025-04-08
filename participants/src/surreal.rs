mod signin;

use anyhow::Result;
use spin_sdk::http::{Request, Method, Response};

pub struct SurrealDB {
    host: String,
    user: String,
    password: String,
    namespace: String,
    database: String,
    token: String
}

impl SurrealDB {
    pub fn builder(host: &str) -> SurrealDBBuilder {
        SurrealDBBuilder {
            host: host.to_string(),
            user: String::new(),
            password: String::new(),
            namespace: String::new(),
            database: String::new(),
        }
    }

    pub async fn signin(&mut self) -> Result<()> {
        let signin_request = signin::SigninRequest {
            user: self.user.clone(),
            password: self.password.clone(),
            namespace: self.namespace.clone(),
            database: self.database.clone(),
        };

        let uri = format!("{}/{}", self.host, "signin");

        let request = Request::builder()
            .method(Method::Post)
            .uri(&uri)
            .header("Accept","application/json")
            .body(serde_json::to_string(&signin_request)?)
            .build();

        let response: Response = spin_sdk::http::send(request).await?;

        let signin_response: signin::SigninResponse = serde_json::from_slice(response.body())?;

        self.token = signin_response.token;

        Ok(())
    }
}

pub struct SurrealDBBuilder {
    host: String,
    user: String,
    password: String,
    namespace: String,
    database: String,
}

impl SurrealDBBuilder {
    pub fn build(self) -> SurrealDB {
        SurrealDB {
            host: self.host,
            user: self.user,
            password: self.password,
            namespace: self.namespace,
            database: self.database,
            token: String::new();
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = user.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    pub fn namespace(mut self, namespace: &str) -> Self {
        self.namespace = namespace.to_string();
        self
    }

    pub fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }
}
