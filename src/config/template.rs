use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TemplateDefinition {

    #[serde(rename = "@name")]
    pub id: String,
    
}

