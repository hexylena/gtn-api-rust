# \InternalApi

All URIs are relative to *https://training.galaxyproject.org/training-material/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gtn_bib_get**](InternalApi.md#gtn_bib_get) | **GET** /gtn.bib | Get the full GTN bibliography library
[**videos_json_get**](InternalApi.md#videos_json_get) | **GET** /videos.json | List videos



## gtn_bib_get

> String gtn_bib_get()
Get the full GTN bibliography library

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videos_json_get

> serde_json::Value videos_json_get()
List videos

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

