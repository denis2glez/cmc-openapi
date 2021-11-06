# \OHLCVApi

All URIs are relative to *http://pro-api.coinmarketcap.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_cryptocurrency_ohlcv_historical**](OHLCVApi.md#get_v1_cryptocurrency_ohlcv_historical) | **GET** /v1/cryptocurrency/ohlcv/historical | Get OHLCV values (historical)



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

