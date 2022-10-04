# \ContributorsApi

All URIs are relative to *https://training.galaxyproject.org/training-material/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contributors_id_json_get**](ContributorsApi.md#contributors_id_json_get) | **GET** /contributors/{id}.json | Get individual Contributors
[**contributors_json_get**](ContributorsApi.md#contributors_json_get) | **GET** /contributors.json | List all Contributors



## contributors_id_json_get

> crate::models::User contributors_id_json_get(id)
Get individual Contributors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of user to return | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contributors_json_get

> Vec<crate::models::User> contributors_json_get()
List all Contributors

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

