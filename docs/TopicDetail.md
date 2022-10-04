# TopicDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The internal topic name (used in URLs) | 
**r#type** | Option<**String**> | Whether it is a science, admin-dev, or contributing topic | [optional]
**title** | **String** | The human readable title | 
**summary** | **String** | A human readable description of the topic's contents. | 
**docker_image** | Option<**String**> | A docker image ID of a container with all of the dependencies required for this topic | [optional]
**subtopics** | Option<[**Vec<crate::models::TopicDetailSubtopicsInner>**](TopicDetail_subtopics_inner.md)> |  | [optional]
**maintainers** | [**Vec<crate::models::User>**](User.md) |  | 
**materials** | Option<[**Vec<crate::models::TrainingMaterial>**](TrainingMaterial.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


