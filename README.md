# Rust API client for openapi

A collection of tutorials generated and maintained by Galaxy Community Members across the world


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://training.galaxyproject.org/training-material/api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ContributorsApi* | [**contributors_id_json_get**](docs/ContributorsApi.md#contributors_id_json_get) | **GET** /contributors/{id}.json | Get individual Contributors
*ContributorsApi* | [**contributors_json_get**](docs/ContributorsApi.md#contributors_json_get) | **GET** /contributors.json | List all Contributors
*FeedbackApi* | [**feedback_csv_get**](docs/FeedbackApi.md#feedback_csv_get) | **GET** /feedback.csv | Obtain user feedback.
*FeedbackApi* | [**feedback_json_get**](docs/FeedbackApi.md#feedback_json_get) | **GET** /feedback.json | Obtain user feedback.
*InternalApi* | [**gtn_bib_get**](docs/InternalApi.md#gtn_bib_get) | **GET** /gtn.bib | Get the full GTN bibliography library
*InternalApi* | [**videos_json_get**](docs/InternalApi.md#videos_json_get) | **GET** /videos.json | List videos
*MaterialsApi* | [**topics_json_get**](docs/MaterialsApi.md#topics_json_get) | **GET** /topics.json | List topics
*MaterialsApi* | [**topics_topic_id_json_get**](docs/MaterialsApi.md#topics_topic_id_json_get) | **GET** /topics/{topicId}.json | Get information about a specific topic
*MaterialsApi* | [**topics_topic_id_tutorials_tutorial_id_material_json_get**](docs/MaterialsApi.md#topics_topic_id_tutorials_tutorial_id_material_json_get) | **GET** /topics/{topicId}/tutorials/{tutorialId}/{material}.json | Get information about a specific topic


## Documentation For Models

 - [Topic](docs/Topic.md)
 - [TopicDetail](docs/TopicDetail.md)
 - [TopicDetailSubtopicsInner](docs/TopicDetailSubtopicsInner.md)
 - [TrainingMaterial](docs/TrainingMaterial.md)
 - [User](docs/User.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



