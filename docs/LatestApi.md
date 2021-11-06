# \LatestApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_cryptocurrency_listings_latest**](LatestApi.md#get_v1_cryptocurrency_listings_latest) | **GET** /v1/cryptocurrency/listings/latest | List all cryptocurrencies (latest)
[**get_v1_cryptocurrency_marketpairs_latest**](LatestApi.md#get_v1_cryptocurrency_marketpairs_latest) | **GET** /v1/cryptocurrency/market-pairs/latest | Get market pairs (latest)
[**get_v1_cryptocurrency_quotes_latest**](LatestApi.md#get_v1_cryptocurrency_quotes_latest) | **GET** /v1/cryptocurrency/quotes/latest | Get market quotes (latest)
[**get_v1_exchange_listings_latest**](LatestApi.md#get_v1_exchange_listings_latest) | **GET** /v1/exchange/listings/latest | List all exchanges (latest)
[**get_v1_exchange_marketpairs_latest**](LatestApi.md#get_v1_exchange_marketpairs_latest) | **GET** /v1/exchange/market-pairs/latest | List all market pairs (latest)
[**get_v1_exchange_quotes_latest**](LatestApi.md#get_v1_exchange_quotes_latest) | **GET** /v1/exchange/quotes/latest | Get market quotes (latest)
[**get_v1_globalmetrics_quotes_latest**](LatestApi.md#get_v1_globalmetrics_quotes_latest) | **GET** /v1/global-metrics/quotes/latest | Get aggregate market metrics (latest)



## get_v1_cryptocurrency_listings_latest

> get_v1_cryptocurrency_listings_latest(convert, cryptocurrency_type, limit, sort, sort_dir, start)
List all cryptocurrencies (latest)

Get a paginated list of all cryptocurrencies with latest market data. You can configure this call to sort by market cap or another market ranking field. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.      Cryptocurrencies are listed by CoinMarketCap Rank by default. You may optionally sort against any of the following: **name**: The cryptocurrency name. **symbol**: The cryptocurrency symbol. **date_added**: Date cryptocurrency was added to the system. **market_cap**: market cap (latest trade price x circulating supply). **price**: latest average trade price across markets. **circulating_supply**: approximate number of coins currently in circulation. **total_supply**: approximate total amount of coins in existence right now (minus any coins that have been verifiably burned). **max_supply**: our best approximation of the maximum amount of coins that will ever exist in the lifetime of the currency. **num_market_pairs**: number of market pairs across all exchanges trading each currency. **volume_24h**: 24 hour trading volume for each currency. **percent_change_1h**: 1 hour trading price percentage change for each currency. **percent_change_24h**: 24 hour trading price percentage change for each currency. **percent_change_7d**: 7 day trading price percentage change for each currency.    **This endpoint is available on the following API plans:**   - Starter   - Hobbyist   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Every ~1 minute.     *NOTE: Use this endpoint if you need a sorted and paginated list of cryptocurrencies. If you want to query for market data on a few specific cryptocurrencies use /v1/cryptocurrency/quotes/latest which is optimized for that purpose. The response data between these endpoints is otherwise the same.*

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**cryptocurrency_type** | Option<[**serde_json::Value**](.md)> | The type of cryptocurrency to include |  |
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**sort** | Option<[**serde_json::Value**](.md)> | What field to sort the list of cryptocurrencies by |  |
**sort_dir** | Option<[**serde_json::Value**](.md)> | The direction in which to order cryptocurrencies against the specified sort |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_v1_cryptocurrency_quotes_latest

> get_v1_cryptocurrency_quotes_latest(convert, id, symbol)
Get market quotes (latest)

Get the latest market quote for 1 or more cryptocurrencies. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.  **This endpoint is available on the following API plans:** - Starter - Hobbyist - Standard - Professional - Enterprise  **Cache / Update frequency:** Every ~1 minute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**id** | Option<[**serde_json::Value**](.md)> | One or more comma-separated cryptocurrency CoinMarketCap IDs |  |
**symbol** | Option<[**serde_json::Value**](.md)> | Alternatively pass one or more comma-separated cryptocurrency symbols |  |

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


## get_v1_exchange_quotes_latest

> get_v1_exchange_quotes_latest(convert, id, slug)
Get market quotes (latest)

Get the latest 24 hour volume quote for 1 or more exchanges. Additional market data fields will be available in the future. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.  **This endpoint is available on the following API plans:** - ~~Starter~~ - ~~Hobbyist~~ - Standard - Professional - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**id** | Option<[**serde_json::Value**](.md)> | One or more comma-separated CoinMarketCap exchange IDs |  |
**slug** | Option<[**serde_json::Value**](.md)> | Alternatively, pass a comma-separated list of exchange slugs (URL friendly all lowercase shorthand version of name with spaces replaced with hyphens) |  |

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

