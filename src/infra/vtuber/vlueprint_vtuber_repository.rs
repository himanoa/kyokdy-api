use std::string::String;

use async_trait::async_trait;
use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::domain::vtuber::repository::VTuberRepository;
use crate::domain::channel::model::DraftChannel;

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

#[derive(Deserialize, Serialize, Debug)]
struct Query<'a> {
    query: &'a str,
}

#[derive(Deserialize, Serialize, Debug)]
struct VlueprintResponse {
    results: InnerResult,
}

#[derive(Deserialize, Serialize, Debug)]
struct InnerResult {
    bindings: Vec<Binding>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Binding {
    channel_name: Option<Value>,
    channel_id: Option<Value>
}

#[derive(Deserialize, Serialize, Debug)]
struct Value {
    value: String,
}

#[async_trait]
impl VTuberRepository for VlueprintVTuberRepository {
    async fn list(&self) -> anyhow::Result<Vec<DraftChannel>> {
        let client = Client::new();
        let response = client
            .get("https://vlueprint.org/query")
            .header("Accept", "application/json")
            .query(&Query { query })
            .send()
            .await?
            .json::<VlueprintResponse>()
            .await?;

        Ok(response.results.bindings.iter().flat_map(|b| {
            match (&b.channel_id, &b.channel_name) {
                (Some(cid), Some(cna)) => Ok(DraftChannel { id: cid.value.clone(), name: cna.value.clone() }),
                (Some(_), None) => Err(anyhow!(format!("VTuberRepository: channel name is not found, {:?}", b))),
                (None, Some(_)) => Err(anyhow!(format!("VTuberRepository: channel id is not found, {:?}", b))),
                _ => Err(anyhow!(format!("VTuberRepository: invalid response {:?}", b)))
            }
        })
        .collect::<Vec<DraftChannel>>())
    }
}
