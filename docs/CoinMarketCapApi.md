# \CoinMarketCapApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_cryptocurrency_map**](CoinMarketCapApi.md#get_v1_cryptocurrency_map) | **GET** /v1/cryptocurrency/map | Get CoinMarketCap ID map
[**get_v1_exchange_map**](CoinMarketCapApi.md#get_v1_exchange_map) | **GET** /v1/exchange/map | Get CoinMarketCap ID map



## get_v1_cryptocurrency_map

> get_v1_cryptocurrency_map(limit, listing_status, start, symbol)
Get CoinMarketCap ID map

Returns a paginated list of all cryptocurrencies by CoinMarketCap ID. We recommend using this convenience endpoint to lookup and utilize our unique cryptocurrency `id` across all endpoints as typical identifiers like ticker symbols can match multiple cryptocurrencies and change over time. As a convenience you may pass a comma-separated list of cryptocurrency symbols as `symbol` to filter this list to only those you require.     **This endpoint is available on the following API plans:**   - Starter   - Hobbyist   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Mapping data is updated only as needed, every 30 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**listing_status** | Option<[**serde_json::Value**](.md)> | Only active coins are returned by default |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |
**symbol** | Option<[**serde_json::Value**](.md)> | Optionally pass a comma-separated list of cryptocurrency symbols to return CoinMarketCap IDs for |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_exchange_map

> get_v1_exchange_map(limit, listing_status, slug, start)
Get CoinMarketCap ID map

Returns a paginated list of all cryptocurrency exchanges by CoinMarketCap ID. We recommend using this convenience endpoint to lookup and utilize our unique exchange `id` across all endpoints as typical exchange identifiers may change over time. As a convenience you may pass a comma-separated list of exchanges by `slug` to filter this list to only those you require.  **This endpoint is available on the following API plans:**   - ~~Starter~~   - Hobbyist   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Mapping data is updated only as needed, every 30 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**listing_status** | Option<[**serde_json::Value**](.md)> | Only active cryptocurrency exchanges are returned by default |  |
**slug** | Option<[**serde_json::Value**](.md)> | Optionally pass a comma-separated list of exchange slugs (lowercase URL friendly shorthand name with spaces replaced with dashes) to return CoinMarketCap IDs for |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

