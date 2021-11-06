# \ToolApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_tools_priceconversion**](ToolApi.md#get_v1_tools_priceconversion) | **GET** /v1/tools/price-conversion | Price conversion tool



## get_v1_tools_priceconversion

> get_v1_tools_priceconversion(amount, convert, id, symbol, time)
Price conversion tool

Convert an amount of one currency into up to 32 other cryptocurrency or fiat currencies at the same time using latest exchange rates. Optionally pass a historical timestamp to convert values based on historic averages.  **Note:** Historical fiat conversions aren't yet available and the latest fiat rates will be used as noted by the `last_updated` timestamp included in the market quote. Historical fiat rates will be coming soon.  **This endpoint is available on the following API plans:** - ~~Starter~~ - Hobbyist - Standard - Professional - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount** | Option<[**serde_json::Value**](.md)> | An amount of currency to convert |  |
**convert** | Option<[**serde_json::Value**](.md)> | Pass up to 32 comma-separated fiat or cryptocurrency symbols to convert the source amount to |  |
**id** | Option<[**serde_json::Value**](.md)> | The CoinMarketCap cryptocurrency ID of the base currency to convert from |  |
**symbol** | Option<[**serde_json::Value**](.md)> | Alternatively the cryptocurrency symbol of the base currency to convert from |  |
**time** | Option<[**serde_json::Value**](.md)> | Optional timestamp (Unix or ISO 8601) to reference historical pricing during conversion |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

