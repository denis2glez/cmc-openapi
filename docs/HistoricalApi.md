# \HistoricalApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_cryptocurrency_listings_historical**](HistoricalApi.md#get_v1_cryptocurrency_listings_historical) | **GET** /v1/cryptocurrency/listings/historical | List all cryptocurrencies (historical)
[**get_v1_cryptocurrency_ohlcv_historical**](HistoricalApi.md#get_v1_cryptocurrency_ohlcv_historical) | **GET** /v1/cryptocurrency/ohlcv/historical | Get OHLCV values (historical)
[**get_v1_cryptocurrency_quotes_historical**](HistoricalApi.md#get_v1_cryptocurrency_quotes_historical) | **GET** /v1/cryptocurrency/quotes/historical | Get market quotes (historical)
[**get_v1_exchange_listings_historical**](HistoricalApi.md#get_v1_exchange_listings_historical) | **GET** /v1/exchange/listings/historical | List all exchanges (historical)
[**get_v1_exchange_quotes_historical**](HistoricalApi.md#get_v1_exchange_quotes_historical) | **GET** /v1/exchange/quotes/historical | Get market quotes (historical)
[**get_v1_globalmetrics_quotes_historical**](HistoricalApi.md#get_v1_globalmetrics_quotes_historical) | **GET** /v1/global-metrics/quotes/historical | Get aggregate market metrics (historical)



## get_v1_cryptocurrency_listings_historical

> get_v1_cryptocurrency_listings_historical(convert, cryptocurrency_type, limit, sort, sort_dir, start, timestamp)
List all cryptocurrencies (historical)

**This endpoint is not yet available. It is slated for release in Q3 2018.**   Get a paginated list of all cryptocurrencies with market data for a given historical time. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | Optionally calculate market quotes in up to 32 currencies at once by passing a comma-separated list of cryptocurrency or fiat currency symbols |  |
**cryptocurrency_type** | Option<[**serde_json::Value**](.md)> | The type of cryptocurrency to include |  |
**limit** | Option<[**serde_json::Value**](.md)> | Optionally specify the number of results to return |  |
**sort** | Option<[**serde_json::Value**](.md)> | What field to sort the list of cryptocurrencies by |  |
**sort_dir** | Option<[**serde_json::Value**](.md)> | The direction in which to order cryptocurrencies against the specified sort |  |
**start** | Option<[**serde_json::Value**](.md)> | Optionally offset the start (1-based index) of the paginated list of items to return |  |
**timestamp** | Option<[**serde_json::Value**](.md)> | Timestamp (Unix or ISO 8601) to return historical cryptocurrency listings for |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_cryptocurrency_ohlcv_historical

> get_v1_cryptocurrency_ohlcv_historical(convert, count, id, interval, symbol, time_end, time_period, time_start)
Get OHLCV values (historical)

Return an interval of historic OHLCV (Open, High, Low, Close, Volume) market quotes for a cryptocurrency.  **Technical Details** One OHLCV quote will be returned for every \"time_period\" between your \"time_start\" and \"time_end\". If a \"time_start\" is not supplied, the \"time_period\" will be applied in reverse from \"time_end\". If \"time_end\" is not supplied, it defaults to the current time. If you don't need every \"time_period\" between your dates you may adjust the frequency that \"time_period\" is sampled using the \"interval\" parameter. For example with \"time_period\" set to \"daily\" you may set \"interval\" to \"2d\" to get the daily OHLCV for every other day. You could set \"interval\" to \"monthly\" to get the first daily OHLCV for each month, or set it to \"yearly\" to get the daily OHLCV value against the same date every year.  **Interval Options** There are 2 types of time interval formats that may be used for \"time_period\" and \"interval\" parameters. For \"time_period\" these return aggregate OHLCV data from the beginning to end of each interval period. Apply these time intervals to \"interval\" to adjust how frequently \"time_period\" is sampled.  The first are calendar year and time constants in UTC time: **\"daily\"** - Calendar day intervals for each UTC day. **\"weekly\"** - Calendar week intervals for each calendar week. **\"monthly\"** - Calendar month intervals for each calendar month. **\"yearly\"** - Calendar year intervals for each calendar year.  The second format is relative time: **\"d\"**: Time periods that repeat every \"d\" days (86400 second intervals). Supported day intervals are: \"1d\", \"2d\", \"3d\", \"7d\", \"14d\", \"15d\", \"30d\", \"60d\", \"90d\", \"365d\".  **Note:** \"time_period\" currently only supports the \"daily\" option. \"interval\" supports all interval options.  **This endpoint is available on the following API plans:** - ~~Starter~~ - ~~Hobbyist~~ - Standard (1 month) - Professional (12 months) - Enterprise (Up to 5 years)  **Cache / Update frequency:** Every 24 hours for 1 day OHLCV ranges. Additional OHLCV ranges will be available in the near-term.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | By default market quotes are returned in USD |  |
**count** | Option<[**serde_json::Value**](.md)> | Optionally limit the number of time periods to return results for |  |
**id** | Option<[**serde_json::Value**](.md)> | A CoinMarketCap cryptocurrency ID |  |
**interval** | Option<[**serde_json::Value**](.md)> | Optionally adjust the interval that time_period is sampled |  |
**symbol** | Option<[**serde_json::Value**](.md)> | Alternatively a cryptocurrency symbol |  |
**time_end** | Option<[**serde_json::Value**](.md)> | Timestamp (Unix or ISO 8601) to stop returning OHLCV time periods for (exclusive) |  |
**time_period** | Option<[**serde_json::Value**](.md)> | Time period to return OHLCV data for |  |
**time_start** | Option<[**serde_json::Value**](.md)> | Timestamp (Unix or ISO 8601) to start returning OHLCV time periods for |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_cryptocurrency_quotes_historical

> get_v1_cryptocurrency_quotes_historical(convert, count, id, interval, symbol, time_end, time_start)
Get market quotes (historical)

Returns an interval of historic market quotes for any cryptocurrency based on time and interval parameters.  **Technical Details** A historic quote for every \"interval\" period between your \"time_start\" and \"time_end\" will be returned. If a \"time_start\" is not supplied, the \"interval\" will be applied in reverse from \"time_end\". If \"time_end\" is not supplied, it defaults to the current time. At each \"interval\" period, the historic quote that is closest in time to the requested time will be returned. If no historic quotes are available in a given \"interval\" period up until the next interval period, it will be skipped.  **Interval Options** There are 2 types of time interval formats that may be used for \"interval\".  The first are calendar year and time constants in UTC time: **\"hourly\"** - Get the first quote available at the beginning of each calendar hour. **\"daily\"** - Get the first quote available at the beginning of each calendar day. **\"weekly\"** - Get the first quote available at the beginning of each calendar week. **\"monthly\"** - Get the first quote available at the beginning of each calendar month. **\"yearly\"** - Get the first quote available at the beginning of each calendar year.  The second are relative time intervals. **\"m\"**: Get the first quote available every \"m\" minutes (60 second intervals). Supported minutes are: \"5m\", \"10m\", \"15m\", \"30m\", \"45m\". **\"h\"**: Get the first quote available every \"h\" hours (3600 second intervals). Supported hour intervals are: \"1h\", \"2h\", \"3h\", \"6h\", \"12h\". **\"d\"**: Get the first quote available every \"d\" days (86400 second intervals). Supported day intervals are: \"1d\", \"2d\", \"3d\", \"7d\", \"14d\", \"15d\", \"30d\", \"60d\", \"90d\", \"365d\".  **This endpoint is available on the following API plans:** - ~~Starter~~ - ~~Hobbyist~~ - Standard (1 month) - Professional (12 months) - Enterprise (Up to 5 years)  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | By default market quotes are returned in USD |  |
**count** | Option<[**serde_json::Value**](.md)> | The number of interval periods to return results for |  |
**id** | Option<[**serde_json::Value**](.md)> | A CoinMarketCap cryptocurrency ID |  |
**interval** | Option<[**serde_json::Value**](.md)> | Interval of time to return data points for |  |
**symbol** | Option<[**serde_json::Value**](.md)> | Alternatively pass a cryptocurrency symbol |  |
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


## get_v1_exchange_quotes_historical

> get_v1_exchange_quotes_historical(convert, count, id, interval, slug, time_end, time_start)
Get market quotes (historical)

Returns an interval of historic quotes for any exchange based on time and interval parameters.  Historical exchange quotes currently include: volume_24: Combined 24 hour volume for all market pairs at each historical interval. num_market_pairs: Number of market pairs available at each historical interval. Quotes are returned in USD. Additional currency conversion options and additional fields will be available in the future.  **Technical Details** A historic quote for every \"interval\" period between your \"time_start\" and \"time_end\" will be returned. If a \"time_start\" is not supplied, the \"interval\" will be applied in reverse from \"time_end\". If \"time_end\" is not supplied, it defaults to the current time. At each \"interval\" period, the historic quote that is closest in time to the requested time will be returned. If no historic quotes are available in a given \"interval\" period up until the next interval period, it will be skipped.  **Interval Options** There are 2 types of time interval formats that may be used for \"interval\".  The first are calendar year and time constants in UTC time: **\"hourly\"** - Get the first quote available at the beginning of each calendar hour. **\"daily\"** - Get the first quote available at the beginning of each calendar day. **\"weekly\"** - Get the first quote available at the beginning of each calendar week. **\"monthly\"** - Get the first quote available at the beginning of each calendar month. **\"yearly\"** - Get the first quote available at the beginning of each calendar year.  The second are relative time intervals. **\"m\"**: Get the first quote available every \"m\" minutes (60 second intervals). Supported minutes are: \"5m\", \"10m\", \"15m\", \"30m\", \"45m\". **\"h\"**: Get the first quote available every \"h\" hours (3600 second intervals). Supported hour intervals are: \"1h\", \"2h\", \"3h\", \"6h\", \"12h\". **\"d\"**: Get the first quote available every \"d\" days (86400 second intervals). Supported day intervals are: \"1d\", \"2d\", \"3d\", \"7d\", \"14d\", \"15d\", \"30d\", \"60d\", \"90d\", \"365d\".  **This endpoint is available on the following API plans:**   - ~~Starter~~   - ~~Hobbyist~~   - Standard (1 month)   - Professional (12 months)   - Enterprise (Up to 5 years)  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**convert** | Option<[**serde_json::Value**](.md)> | By default market quotes are returned in USD |  |
**count** | Option<[**serde_json::Value**](.md)> | The number of interval periods to return results for |  |
**id** | Option<[**serde_json::Value**](.md)> | The CoinMarketCap exchange ID to return historical data for |  |
**interval** | Option<[**serde_json::Value**](.md)> | Interval of time to return data points for |  |
**slug** | Option<[**serde_json::Value**](.md)> | Alternatively the exchange slug to return historical data for |  |
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

