use std::string::String;
use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::Client;
use url::form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

use crate::domain::vtuber::repository::VTuberRepository;

pub struct VlueprintVTuberRepository {}

const query: &str = "
prefix vlueprint: <https://vlueprint.org/schema/>
prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>

select ?LABEL ?CHANNEL_ID ?CHANNEL_NAME {
  ?uri rdf:type vlueprint:VirtualBeing.
  ?uri rdfs:label ?LABEL.
  ?uri vlueprint:youtubeChannelId ?CHANNEL_ID.
  ?uri vlueprint:youtubeChannelName ?CHANNEL_NAME.
}
";

#[derive(Deserialize, Serialize)]
struct Query {
    query: String
}

#[derive(Deserialize, Serialize)]
struct VlueprintResponse {
    results: InnerResult
}

#[derive(Deserialize, Serialize)]
struct InnerResult {
    bindings: Vec<Binding>
}

#[derive(Deserialize, Serialize)]
struct Binding {
    CHANNEL_NAME: Value,
    CHANNEL_ID: Value
}

#[derive(Deserialize, Serialize)]
struct Value {
    value: String
}

#[async_trait]
impl VTuberRepository for VlueprintVTuberRepository {
    async fn list(&self) -> Result<Vec<DraftChannel>, Box<dyn std::error::Error>> {
        let encoded: String = Serializer::new(String::new())
            .append_pair("query", query)
            .finish();
        let client = Client::new();
        let url = "https://example.com";
        // let url = format!("https://vlueprint.org/sparql?query={}", query);
        let request = client.get(url).query(&Query { query: String::from(query) }).header("Accept", "application/json");
        let response = reqwest::Client::new()
        .get("https://hyper.rs")
        .send()
        .await?;
        // let response = request.send().await?;
        let body = response.json::<HashMap<String, String>>().await?;
        // let body = client
        //     .get(&url)
        //     .header("Accept", "application/json")
        //     .send()
        //     .await?
        //     .json::<VlueprintResponse>()
        //     .await?;
        Ok(vec![])
    }
}
