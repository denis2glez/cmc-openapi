# \MetricsApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_globalmetrics_quotes_historical**](MetricsApi.md#get_v1_globalmetrics_quotes_historical) | **GET** /v1/global-metrics/quotes/historical | Get aggregate market metrics (historical)
[**get_v1_globalmetrics_quotes_latest**](MetricsApi.md#get_v1_globalmetrics_quotes_latest) | **GET** /v1/global-metrics/quotes/latest | Get aggregate market metrics (latest)



## get_v1_globalmetrics_quotes_historical

> get_v1_globalmetrics_quotes_historical(convert, count, interval, time_end, time_start)
Get aggregate market metrics (historical)

Get an interval of aggregate 24 hour volume and market cap data globally based on time and interval parameters.  **Technical Details** A historic quote for every \"interval\" period between your \"time_start\" and \"time_end\" will be returned. If a \"time_start\" is not supplied, the \"interval\" will be applied in reverse from \"time_end\". If \"time_end\" is not supplied, it defaults to the current time. At each \"interval\" period, the historic quote that is closest in time to the requested time will be returned. If no historic quotes are available in a given \"interval\" period up until the next interval period, it will be skipped.  **Interval Options** There are 2 types of time interval formats that may be used for \"interval\".  The first are calendar year and time constants in UTC time: **\"hourly\"** - Get the first quote available at the beginning of each calendar hour. **\"daily\"** - Get the first quote available at the beginning of each calendar day. **\"weekly\"** - Get the first quote available at the beginning of each calendar week. **\"monthly\"** - Get the first quote available at the beginning of each calendar month. **\"yearly\"** - Get the first quote available at the beginning of each calendar year.  The second are relative time intervals. **\"m\"**: Get the first quote available every \"m\" minutes (60 second intervals). Supported minutes are: \"5m\", \"10m\", \"15m\", \"30m\", \"45m\". **\"h\"**: Get the first quote available every \"h\" hours (3600 second intervals). Supported hour intervals are: \"1h\", \"2h\", \"3h\", \"6h\", \"12h\". **\"d\"**: Get the first quote available every \"d\" days (86400 second intervals). Supported day intervals are: \"1d\", \"2d\", \"3d\", \"7d\", \"14d\", \"15d\", \"30d\", \"60d\", \"90d\", \"365d\".  **This endpoint is available on the following API plans:** - ~~Starter~~ - ~~Hobbyist~~ - Standard (1 month) - Professional (12 months) - Enterprise (Up to 5 years)  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | By default market quotes are returned in USD |  |
**count** | Option<[**serde_json::Value**](.md)> | The number of interval periods to return results for |  |
**interval** | Option<[**serde_json::Value**](.md)> | Interval of time to return data points for |  |
**time_end** | Option<[**serde_json::Value**](.md)> | Timestamp (Unix or ISO 8601) to stop returning quotes for (inclusive) |  |
**time_start** | Option<[**serde_json::Value**](.md)> | Timestamp (Unix or ISO 8601) to start returning quotes for |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_globalmetrics_quotes_latest

> get_v1_globalmetrics_quotes_latest(convert)
Get aggregate market metrics (latest)

Get the latest quote of aggregate market metrics. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.  **This endpoint is available on the following API plans:** - Starter - Hobbyist - Standard - Professional - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

