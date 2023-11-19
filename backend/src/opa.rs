use std::time::Duration;

use serde::{Serialize, Deserialize};

const OPA_ENDPOINT: &str = "opa";

pub async fn init_opa() {
    tokio::time::sleep(Duration::from_secs(10)).await;
    let body = r#"
    package school

    default is_student = false

    is_student {
        startswith(input.email, "s")
    }"#;

    let _ = reqwest::Client::new()
        .put(&format!("http://{OPA_ENDPOINT}:8181/v1/policies/policy"))
        .header("Content-Type", "text/plain")
        .body(body).send().await.unwrap();
}

#[derive(Debug,Serialize)]
pub struct Input {
    pub input: IsStudent
}

#[derive(Debug,Deserialize)]
pub struct Response {
    result: bool
}

#[derive(Debug,Serialize)]
pub struct IsStudent {
    pub email: String
}
pub async fn is_student(email: String) -> Result<bool, reqwest::Error>{
    let input = Input {
        input: IsStudent { email }
    };

    let response = reqwest::Client::new()
        .post(&format!("http://{OPA_ENDPOINT}:8181/v1/data/school/is_student"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&input).unwrap())
        .send().await?;
    let Response {result} = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{result:?}");

    Ok(result)
}
