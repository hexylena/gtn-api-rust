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
pub struct TrainingMaterial {
    /// The internal layout of the material for rendering
    #[serde(rename = "layout")]
    pub layout: String,
    /// The human readable name of the material
    #[serde(rename = "title")]
    pub title: String,
    /// A link to some datasets on Zenodo required for the tutorial
    #[serde(rename = "zenodo_link", skip_serializing_if = "Option::is_none")]
    pub zenodo_link: Option<String>,
    /// A list of questions students will answer when following this tutorial
    #[serde(rename = "questions", skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<String>>,
    /// A list of objectives students will accomplish when following this tutorial
    #[serde(rename = "objectives", skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Vec<String>>,
    /// A list of take aways when completing this training
    #[serde(rename = "key_points", skip_serializing_if = "Option::is_none")]
    pub key_points: Option<Vec<String>>,
    /// An estimate of the amount of time required to complete this tutorial.
    #[serde(rename = "time_estimation", skip_serializing_if = "Option::is_none")]
    pub time_estimation: Option<String>,
    #[serde(rename = "contributors", skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<crate::models::User>>,
    /// Which subtopic (see topic.json information)
    #[serde(rename = "subtopic", skip_serializing_if = "Option::is_none")]
    pub subtopic: Option<String>,
    /// When was this tutorial last modified
    #[serde(rename = "last_modified_at", skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
    /// Internal/URL component for the topic
    #[serde(rename = "topic_name", skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// Internal/URL component for the tutorial
    #[serde(rename = "tutorial_name", skip_serializing_if = "Option::is_none")]
    pub tutorial_name: Option<String>,
    /// Is there a hands-on available for this material
    #[serde(rename = "hands_on", skip_serializing_if = "Option::is_none")]
    pub hands_on: Option<bool>,
    /// Are there slides available for this material
    #[serde(rename = "slides", skip_serializing_if = "Option::is_none")]
    pub slides: Option<bool>,
    /// Are there workflows available for this material
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<bool>,
    /// Are there tours available for this material
    #[serde(rename = "tours", skip_serializing_if = "Option::is_none")]
    pub tours: Option<bool>,
    /// Is there a video being auto-generated from the slides
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
}

impl TrainingMaterial {
    pub fn new(layout: String, title: String) -> TrainingMaterial {
        TrainingMaterial {
            layout,
            title,
            zenodo_link: None,
            questions: None,
            objectives: None,
            key_points: None,
            time_estimation: None,
            contributors: None,
            subtopic: None,
            last_modified_at: None,
            topic_name: None,
            tutorial_name: None,
            hands_on: None,
            slides: None,
            workflows: None,
            tours: None,
            video: None,
        }
    }
}

