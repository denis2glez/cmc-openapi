# \ExchangesApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_exchange_listings_historical**](ExchangesApi.md#get_v1_exchange_listings_historical) | **GET** /v1/exchange/listings/historical | List all exchanges (historical)
[**get_v1_exchange_listings_latest**](ExchangesApi.md#get_v1_exchange_listings_latest) | **GET** /v1/exchange/listings/latest | List all exchanges (latest)



## get_v1_exchange_listings_historical

> get_v1_exchange_listings_historical(convert, limit, market_type, sort, sort_dir, start, timestamp)
List all exchanges (historical)

**This endpoint is not yet available. It is slated for release in Q3 2018.**   Get a paginated list of all cryptocurrency exchanges with historical market data for a given point in time. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**market_type** | Option<[**serde_json::Value**](.md)> | The type of exchange markets to include in rankings |  |
**sort** | Option<[**serde_json::Value**](.md)> | What field to sort the list of exchanges by |  |
**sort_dir** | Option<[**serde_json::Value**](.md)> | The direction in which to order exchanges against the specified sort |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |
**timestamp** | Option<[**serde_json::Value**](.md)> | Timestamp (Unix or ISO 8601) to return historical exchange listings for |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_exchange_listings_latest

> get_v1_exchange_listings_latest(convert, limit, market_type, sort, sort_dir, start)
List all exchanges (latest)

Get a paginated list of all cryptocurrency exchanges with 24 hour volume. Additional market data fields will be available in the future. You can configure this call to sort by 24 hour volume or another field. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.      **This endpoint is available on the following API plans:**   - ~~Starter~~   - ~~Hobbyist~~   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.        *NOTE: Use this endpoint if you need a sorted and paginated list of exchanges. If you want to query for market data on a few specific exchanges use /v1/exchange/quotes/latest which is optimized for that purpose. The response data between these endpoints is otherwise the same.*

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**market_type** | Option<[**serde_json::Value**](.md)> | The type of exchange markets to include in rankings |  |
**sort** | Option<[**serde_json::Value**](.md)> | What field to sort the list of exchanges by |  |
**sort_dir** | Option<[**serde_json::Value**](.md)> | The direction in which to order exchanges against the specified sort |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

