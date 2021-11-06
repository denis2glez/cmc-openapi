# \MetadataApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_cryptocurrency_info**](MetadataApi.md#get_v1_cryptocurrency_info) | **GET** /v1/cryptocurrency/info | Get metadata
[**get_v1_exchange_info**](MetadataApi.md#get_v1_exchange_info) | **GET** /v1/exchange/info | Get metadata



## get_v1_cryptocurrency_info

> get_v1_cryptocurrency_info(id, symbol)
Get metadata

Returns all static metadata for one or more cryptocurrencies including name, symbol, logo, and its various registered URLs.  **This endpoint is available on the following API plans:** - Starter - Hobbyist - Standard - Professional - Enterprise  **Cache / Update frequency:** Static data is updated only as needed, every 30 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | One or more comma-separated CoinMarketCap cryptocurrency IDs |  |
**symbol** | Option<[**serde_json::Value**](.md)> | Alternatively pass one or more comma-separated cryptocurrency symbols |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_exchange_info

> get_v1_exchange_info(id, slug)
Get metadata

Returns all static metadata for one or more exchanges including logo and homepage URL.    **This endpoint is available on the following API plans:**   - ~~Starter~~   - Hobbyist   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Static data is updated only as needed, every 30 seconds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**serde_json::Value**](.md)> | One or more comma-separated CoinMarketCap cryptocurrency exchange ids |  |
**slug** | Option<[**serde_json::Value**](.md)> | Alternatively, one or more comma-separated exchange names in URL friendly shorthand slug format (all lowercase, spaces replaced with hyphens) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

