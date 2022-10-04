# \MaterialsApi

All URIs are relative to *https://training.galaxyproject.org/training-material/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**topics_json_get**](MaterialsApi.md#topics_json_get) | **GET** /topics.json | List topics
[**topics_topic_id_json_get**](MaterialsApi.md#topics_topic_id_json_get) | **GET** /topics/{topicId}.json | Get information about a specific topic
[**topics_topic_id_tutorials_tutorial_id_material_json_get**](MaterialsApi.md#topics_topic_id_tutorials_tutorial_id_material_json_get) | **GET** /topics/{topicId}/tutorials/{tutorialId}/{material}.json | Get information about a specific topic



## topics_json_get

> crate::models::Topic topics_json_get()
List topics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Topic**](Topic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## topics_topic_id_json_get

> crate::models::TopicDetail topics_topic_id_json_get(topic_id)
Get information about a specific topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_id** | **String** | ID of the topic | [required] |

### Return type

[**crate::models::TopicDetail**](TopicDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## topics_topic_id_tutorials_tutorial_id_material_json_get

> crate::models::TrainingMaterial topics_topic_id_tutorials_tutorial_id_material_json_get(topic_id, tutorial_id, material)
Get information about a specific topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_id** | **String** | ID of the topic | [required] |
**tutorial_id** | **String** | ID of the tutorial | [required] |
**material** | **String** | 'slides' or 'tutorial', whichever material you want to request. | [required] |

### Return type

[**crate::models::TrainingMaterial**](TrainingMaterial.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

