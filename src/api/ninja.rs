use reqwest::{Url};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillGem {
    lines: Vec<SkillGemItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SkillGemItem {
    id: i64,
    name: String,
    #[serde(default)]
    corrupted: bool,
    icon: String,
    #[serde(rename = "gemLevel")]
    gem_level: i8,
    #[serde(rename = "gemQuality", default)]
    gem_quality: i8,
    #[serde(rename = "chaosValue")]
    chaos_value: f64,
    #[serde(rename = "exaltedValue")]
    exalted_value: f64,
}

impl SkillGem {
    pub async fn fetch_or_read_cache() -> Result<Self, anyhow::Error> {
        let json = std::fs::read_to_string("ninja/22-06-27_sentinel_skill_gem.json");
        let skill_gem = match json {
            Ok(json) => {
                let skill_gem = serde_json::from_str::<SkillGem>(&json);
                match skill_gem {
                    Ok(it) => it,
                    Err(e) => {
                        println!("error {}", e);
                        Err(e)?
                    }
                }
            }
            Err(e) => {
                println!("Cache not found: {}", e);
                SkillGem::fetch().await?
            }
        };

        Ok(skill_gem)
    }

    async fn fetch() -> Result<Self, anyhow::Error> {
        let url = "https://poe.ninja/api/data/itemoverview?league=Sentinel&type=SkillGem";
        let url = Url::parse(&*url)?;
        let response = reqwest::get(url).await?.json::<SkillGem>().await?;

        Ok(response)
    }
}