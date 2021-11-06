# \PairsApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_cryptocurrency_marketpairs_latest**](PairsApi.md#get_v1_cryptocurrency_marketpairs_latest) | **GET** /v1/cryptocurrency/market-pairs/latest | Get market pairs (latest)
[**get_v1_exchange_marketpairs_latest**](PairsApi.md#get_v1_exchange_marketpairs_latest) | **GET** /v1/exchange/market-pairs/latest | List all market pairs (latest)



## get_v1_cryptocurrency_marketpairs_latest

> get_v1_cryptocurrency_marketpairs_latest(convert, id, limit, start, symbol)
Get market pairs (latest)

Lists all market pairs for the specified cryptocurrency with associated stats. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.     **This endpoint is available on the following API plans:**   - ~~Starter~~   - ~~Hobbyist~~   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Every ~1 minute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**id** | Option<[**serde_json::Value**](.md)> | A cryptocurrency by CoinMarketCap ID |  |
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |
**symbol** | Option<[**serde_json::Value**](.md)> | Alternatively pass a cryptocurrency by symbol |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_exchange_marketpairs_latest

> get_v1_exchange_marketpairs_latest(convert, id, limit, slug, start)
List all market pairs (latest)

Get a list of active market pairs for an exchange. Active means the market pair is open for trading. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.'    **This endpoint is available on the following API plans:**   - ~~Starter~~   - ~~Hobbyist~~   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**id** | Option<[**serde_json::Value**](.md)> | A CoinMarketCap exchange ID |  |
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**slug** | Option<[**serde_json::Value**](.md)> | Alternatively pass an exchange slug (URL friendly all lowercase shorthand version of name with spaces replaced with hyphens) |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

