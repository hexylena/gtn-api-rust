/*
 * Galaxy Training Network API
 *
 * A collection of tutorials generated and maintained by Galaxy Community Members across the world
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TopicDetailSubtopicsInner {
    /// Subtopic ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Subtopic title (presented in the UI to users)
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A helpful subtopic description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TopicDetailSubtopicsInner {
    pub fn new() -> TopicDetailSubtopicsInner {
        TopicDetailSubtopicsInner {
            id: None,
            title: None,
            description: None,
        }
    }
}

