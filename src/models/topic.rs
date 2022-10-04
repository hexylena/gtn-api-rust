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
pub struct Topic {
    /// The internal topic name (used in URLs)
    #[serde(rename = "name")]
    pub name: String,
    /// The human readable title
    #[serde(rename = "title")]
    pub title: String,
    /// A human readable description of the topic's contents.
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "maintainers")]
    pub maintainers: Vec<crate::models::User>,
}

impl Topic {
    pub fn new(name: String, title: String, summary: String, maintainers: Vec<crate::models::User>) -> Topic {
        Topic {
            name,
            title,
            summary,
            maintainers,
        }
    }
}

