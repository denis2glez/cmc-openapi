# Rust `cmc`-openapi client

## Overview

This API client was generated using the [OpenAPI Generator](https://openapi-generator.tech) project.

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://pro-api.coinmarketcap.com*.

**Disclaimer:** I don't have any type of relationship with CoinMarketCap.

| Class                 | Method                                                                                                                 | HTTP request                                   | Description                               |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- | ----------------------------------------- |
| *DefaultApi*          | [**get_v1_cryptocurrency_listings_historical**](docs/DefaultApi.md#get_v1_cryptocurrency_listings_historical)          | **GET** /v1/cryptocurrency/listings/historical | List all cryptocurrencies (historical)    |
| *DefaultApi*          | [**get_v1_cryptocurrency_listings_latest**](docs/DefaultApi.md#get_v1_cryptocurrency_listings_latest)                  | **GET** /v1/cryptocurrency/listings/latest     | List all cryptocurrencies (latest)        |
| *DefaultApi*          | [**get_v1_exchange_listings_historical**](docs/DefaultApi.md#get_v1_exchange_listings_historical)                      | **GET** /v1/exchange/listings/historical       | List all exchanges (historical)           |
| *DefaultApi*          | [**get_v1_exchange_listings_latest**](docs/DefaultApi.md#get_v1_exchange_listings_latest)                              | **GET** /v1/exchange/listings/latest           | List all exchanges (latest)               |
| *DefaultApi*          | [**get_v1_exchange_marketpairs_latest**](docs/DefaultApi.md#get_v1_exchange_marketpairs_latest)                        | **GET** /v1/exchange/market-pairs/latest       | List all market pairs (latest)            |
| *AggregateApi*        | [**get_v1_globalmetrics_quotes_historical**](docs/AggregateApi.md#get_v1_globalmetrics_quotes_historical)              | **GET** /v1/global-metrics/quotes/historical   | Get aggregate market metrics (historical) |
| *AggregateApi*        | [**get_v1_globalmetrics_quotes_latest**](docs/AggregateApi.md#get_v1_globalmetrics_quotes_latest)                      | **GET** /v1/global-metrics/quotes/latest       | Get aggregate market metrics (latest)     |
| *BlockchainApi*       | [**get_v1_cryptocurrency_info**](docs/BlockchainApi.md#get_v1_cryptocurrency_info)                                     | **GET** /v1/cryptocurrency/info                | Get metadata                              |
| *BlockchainApi*       | [**get_v1_cryptocurrency_listings_historical**](docs/BlockchainApi.md#get_v1_cryptocurrency_listings_historical)       | **GET** /v1/cryptocurrency/listings/historical | List all cryptocurrencies (historical)    |
| *BlockchainApi*       | [**get_v1_cryptocurrency_listings_latest**](docs/BlockchainApi.md#get_v1_cryptocurrency_listings_latest)               | **GET** /v1/cryptocurrency/listings/latest     | List all cryptocurrencies (latest)        |
| *BlockchainApi*       | [**get_v1_cryptocurrency_map**](docs/BlockchainApi.md#get_v1_cryptocurrency_map)                                       | **GET** /v1/cryptocurrency/map                 | Get CoinMarketCap ID map                  |
| *BlockchainApi*       | [**get_v1_cryptocurrency_marketpairs_latest**](docs/BlockchainApi.md#get_v1_cryptocurrency_marketpairs_latest)         | **GET** /v1/cryptocurrency/market-pairs/latest | Get market pairs (latest)                 |
| *BlockchainApi*       | [**get_v1_cryptocurrency_ohlcv_historical**](docs/BlockchainApi.md#get_v1_cryptocurrency_ohlcv_historical)             | **GET** /v1/cryptocurrency/ohlcv/historical    | Get OHLCV values (historical)             |
| *BlockchainApi*       | [**get_v1_cryptocurrency_quotes_historical**](docs/BlockchainApi.md#get_v1_cryptocurrency_quotes_historical)           | **GET** /v1/cryptocurrency/quotes/historical   | Get market quotes (historical)            |
| *BlockchainApi*       | [**get_v1_cryptocurrency_quotes_latest**](docs/BlockchainApi.md#get_v1_cryptocurrency_quotes_latest)                   | **GET** /v1/cryptocurrency/quotes/latest       | Get market quotes (latest)                |
| *BlockchainApi*       | [**get_v1_exchange_info**](docs/BlockchainApi.md#get_v1_exchange_info)                                                 | **GET** /v1/exchange/info                      | Get metadata                              |
| *BlockchainApi*       | [**get_v1_exchange_listings_historical**](docs/BlockchainApi.md#get_v1_exchange_listings_historical)                   | **GET** /v1/exchange/listings/historical       | List all exchanges (historical)           |
| *BlockchainApi*       | [**get_v1_exchange_listings_latest**](docs/BlockchainApi.md#get_v1_exchange_listings_latest)                           | **GET** /v1/exchange/listings/latest           | List all exchanges (latest)               |
| *BlockchainApi*       | [**get_v1_exchange_map**](docs/BlockchainApi.md#get_v1_exchange_map)                                                   | **GET** /v1/exchange/map                       | Get CoinMarketCap ID map                  |
| *BlockchainApi*       | [**get_v1_exchange_marketpairs_latest**](docs/BlockchainApi.md#get_v1_exchange_marketpairs_latest)                     | **GET** /v1/exchange/market-pairs/latest       | List all market pairs (latest)            |
| *BlockchainApi*       | [**get_v1_exchange_quotes_historical**](docs/BlockchainApi.md#get_v1_exchange_quotes_historical)                       | **GET** /v1/exchange/quotes/historical         | Get market quotes (historical)            |
| *BlockchainApi*       | [**get_v1_exchange_quotes_latest**](docs/BlockchainApi.md#get_v1_exchange_quotes_latest)                               | **GET** /v1/exchange/quotes/latest             | Get market quotes (latest)                |
| *BlockchainApi*       | [**get_v1_globalmetrics_quotes_historical**](docs/BlockchainApi.md#get_v1_globalmetrics_quotes_historical)             | **GET** /v1/global-metrics/quotes/historical   | Get aggregate market metrics (historical) |
| *BlockchainApi*       | [**get_v1_globalmetrics_quotes_latest**](docs/BlockchainApi.md#get_v1_globalmetrics_quotes_latest)                     | **GET** /v1/global-metrics/quotes/latest       | Get aggregate market metrics (latest)     |
| *BlockchainApi*       | [**get_v1_tools_priceconversion**](docs/BlockchainApi.md#get_v1_tools_priceconversion)                                 | **GET** /v1/tools/price-conversion             | Price conversion tool                     |
| *CoinMarketCapApi*    | [**get_v1_cryptocurrency_map**](docs/CoinMarketCapApi.md#get_v1_cryptocurrency_map)                                    | **GET** /v1/cryptocurrency/map                 | Get CoinMarketCap ID map                  |
| *CoinMarketCapApi*    | [**get_v1_exchange_map**](docs/CoinMarketCapApi.md#get_v1_exchange_map)                                                | **GET** /v1/exchange/map                       | Get CoinMarketCap ID map                  |
| *ConversionApi*       | [**get_v1_tools_priceconversion**](docs/ConversionApi.md#get_v1_tools_priceconversion)                                 | **GET** /v1/tools/price-conversion             | Price conversion tool                     |
| *CryptocurrenciesApi* | [**get_v1_cryptocurrency_listings_historical**](docs/CryptocurrenciesApi.md#get_v1_cryptocurrency_listings_historical) | **GET** /v1/cryptocurrency/listings/historical | List all cryptocurrencies (historical)    |
| *CryptocurrenciesApi* | [**get_v1_cryptocurrency_listings_latest**](docs/CryptocurrenciesApi.md#get_v1_cryptocurrency_listings_latest)         | **GET** /v1/cryptocurrency/listings/latest     | List all cryptocurrencies (latest)        |
| *ExchangesApi*        | [**get_v1_exchange_listings_historical**](docs/ExchangesApi.md#get_v1_exchange_listings_historical)                    | **GET** /v1/exchange/listings/historical       | List all exchanges (historical)           |
| *ExchangesApi*        | [**get_v1_exchange_listings_latest**](docs/ExchangesApi.md#get_v1_exchange_listings_latest)                            | **GET** /v1/exchange/listings/latest           | List all exchanges (latest)               |
| *HistoricalApi*       | [**get_v1_cryptocurrency_listings_historical**](docs/HistoricalApi.md#get_v1_cryptocurrency_listings_historical)       | **GET** /v1/cryptocurrency/listings/historical | List all cryptocurrencies (historical)    |
| *HistoricalApi*       | [**get_v1_cryptocurrency_ohlcv_historical**](docs/HistoricalApi.md#get_v1_cryptocurrency_ohlcv_historical)             | **GET** /v1/cryptocurrency/ohlcv/historical    | Get OHLCV values (historical)             |
| *HistoricalApi*       | [**get_v1_cryptocurrency_quotes_historical**](docs/HistoricalApi.md#get_v1_cryptocurrency_quotes_historical)           | **GET** /v1/cryptocurrency/quotes/historical   | Get market quotes (historical)            |
| *HistoricalApi*       | [**get_v1_exchange_listings_historical**](docs/HistoricalApi.md#get_v1_exchange_listings_historical)                   | **GET** /v1/exchange/listings/historical       | List all exchanges (historical)           |
| *HistoricalApi*       | [**get_v1_exchange_quotes_historical**](docs/HistoricalApi.md#get_v1_exchange_quotes_historical)                       | **GET** /v1/exchange/quotes/historical         | Get market quotes (historical)            |
| *HistoricalApi*       | [**get_v1_globalmetrics_quotes_historical**](docs/HistoricalApi.md#get_v1_globalmetrics_quotes_historical)             | **GET** /v1/global-metrics/quotes/historical   | Get aggregate market metrics (historical) |
| *IDApi*               | [**get_v1_cryptocurrency_map**](docs/IDApi.md#get_v1_cryptocurrency_map)                                               | **GET** /v1/cryptocurrency/map                 | Get CoinMarketCap ID map                  |
| *IDApi*               | [**get_v1_exchange_map**](docs/IDApi.md#get_v1_exchange_map)                                                           | **GET** /v1/exchange/map                       | Get CoinMarketCap ID map                  |
| *LatestApi*           | [**get_v1_cryptocurrency_listings_latest**](docs/LatestApi.md#get_v1_cryptocurrency_listings_latest)                   | **GET** /v1/cryptocurrency/listings/latest     | List all cryptocurrencies (latest)        |
| *LatestApi*           | [**get_v1_cryptocurrency_marketpairs_latest**](docs/LatestApi.md#get_v1_cryptocurrency_marketpairs_latest)             | **GET** /v1/cryptocurrency/market-pairs/latest | Get market pairs (latest)                 |
| *LatestApi*           | [**get_v1_cryptocurrency_quotes_latest**](docs/LatestApi.md#get_v1_cryptocurrency_quotes_latest)                       | **GET** /v1/cryptocurrency/quotes/latest       | Get market quotes (latest)                |
| *LatestApi*           | [**get_v1_exchange_listings_latest**](docs/LatestApi.md#get_v1_exchange_listings_latest)                               | **GET** /v1/exchange/listings/latest           | List all exchanges (latest)               |
| *LatestApi*           | [**get_v1_exchange_marketpairs_latest**](docs/LatestApi.md#get_v1_exchange_marketpairs_latest)                         | **GET** /v1/exchange/market-pairs/latest       | List all market pairs (latest)            |
| *LatestApi*           | [**get_v1_exchange_quotes_latest**](docs/LatestApi.md#get_v1_exchange_quotes_latest)                                   | **GET** /v1/exchange/quotes/latest             | Get market quotes (latest)                |
| *LatestApi*           | [**get_v1_globalmetrics_quotes_latest**](docs/LatestApi.md#get_v1_globalmetrics_quotes_latest)                         | **GET** /v1/global-metrics/quotes/latest       | Get aggregate market metrics (latest)     |
| *ListApi*             | [**get_v1_cryptocurrency_listings_historical**](docs/ListApi.md#get_v1_cryptocurrency_listings_historical)             | **GET** /v1/cryptocurrency/listings/historical | List all cryptocurrencies (historical)    |
| *ListApi*             | [**get_v1_cryptocurrency_listings_latest**](docs/ListApi.md#get_v1_cryptocurrency_listings_latest)                     | **GET** /v1/cryptocurrency/listings/latest     | List all cryptocurrencies (latest)        |
| *ListApi*             | [**get_v1_exchange_listings_historical**](docs/ListApi.md#get_v1_exchange_listings_historical)                         | **GET** /v1/exchange/listings/historical       | List all exchanges (historical)           |
| *ListApi*             | [**get_v1_exchange_listings_latest**](docs/ListApi.md#get_v1_exchange_listings_latest)                                 | **GET** /v1/exchange/listings/latest           | List all exchanges (latest)               |
| *ListApi*             | [**get_v1_exchange_marketpairs_latest**](docs/ListApi.md#get_v1_exchange_marketpairs_latest)                           | **GET** /v1/exchange/market-pairs/latest       | List all market pairs (latest)            |
| *MapApi*              | [**get_v1_cryptocurrency_map**](docs/MapApi.md#get_v1_cryptocurrency_map)                                              | **GET** /v1/cryptocurrency/map                 | Get CoinMarketCap ID map                  |
| *MapApi*              | [**get_v1_exchange_map**](docs/MapApi.md#get_v1_exchange_map)                                                          | **GET** /v1/exchange/map                       | Get CoinMarketCap ID map                  |
| *MarketApi*           | [**get_v1_cryptocurrency_marketpairs_latest**](docs/MarketApi.md#get_v1_cryptocurrency_marketpairs_latest)             | **GET** /v1/cryptocurrency/market-pairs/latest | Get market pairs (latest)                 |
| *MarketApi*           | [**get_v1_cryptocurrency_quotes_historical**](docs/MarketApi.md#get_v1_cryptocurrency_quotes_historical)               | **GET** /v1/cryptocurrency/quotes/historical   | Get market quotes (historical)            |
| *MarketApi*           | [**get_v1_cryptocurrency_quotes_latest**](docs/MarketApi.md#get_v1_cryptocurrency_quotes_latest)                       | **GET** /v1/cryptocurrency/quotes/latest       | Get market quotes (latest)                |
| *MarketApi*           | [**get_v1_exchange_marketpairs_latest**](docs/MarketApi.md#get_v1_exchange_marketpairs_latest)                         | **GET** /v1/exchange/market-pairs/latest       | List all market pairs (latest)            |
| *MarketApi*           | [**get_v1_exchange_quotes_historical**](docs/MarketApi.md#get_v1_exchange_quotes_historical)                           | **GET** /v1/exchange/quotes/historical         | Get market quotes (historical)            |
| *MarketApi*           | [**get_v1_exchange_quotes_latest**](docs/MarketApi.md#get_v1_exchange_quotes_latest)                                   | **GET** /v1/exchange/quotes/latest             | Get market quotes (latest)                |
| *MarketApi*           | [**get_v1_globalmetrics_quotes_historical**](docs/MarketApi.md#get_v1_globalmetrics_quotes_historical)                 | **GET** /v1/global-metrics/quotes/historical   | Get aggregate market metrics (historical) |
| *MarketApi*           | [**get_v1_globalmetrics_quotes_latest**](docs/MarketApi.md#get_v1_globalmetrics_quotes_latest)                         | **GET** /v1/global-metrics/quotes/latest       | Get aggregate market metrics (latest)     |
| *MetadataApi*         | [**get_v1_cryptocurrency_info**](docs/MetadataApi.md#get_v1_cryptocurrency_info)                                       | **GET** /v1/cryptocurrency/info                | Get metadata                              |
| *MetadataApi*         | [**get_v1_exchange_info**](docs/MetadataApi.md#get_v1_exchange_info)                                                   | **GET** /v1/exchange/info                      | Get metadata                              |
| *MetricsApi*          | [**get_v1_globalmetrics_quotes_historical**](docs/MetricsApi.md#get_v1_globalmetrics_quotes_historical)                | **GET** /v1/global-metrics/quotes/historical   | Get aggregate market metrics (historical) |
| *MetricsApi*          | [**get_v1_globalmetrics_quotes_latest**](docs/MetricsApi.md#get_v1_globalmetrics_quotes_latest)                        | **GET** /v1/global-metrics/quotes/latest       | Get aggregate market metrics (latest)     |
| *OHLCVApi*            | [**get_v1_cryptocurrency_ohlcv_historical**](docs/OHLCVApi.md#get_v1_cryptocurrency_ohlcv_historical)                  | **GET** /v1/cryptocurrency/ohlcv/historical    | Get OHLCV values (historical)             |
| *PairsApi*            | [**get_v1_cryptocurrency_marketpairs_latest**](docs/PairsApi.md#get_v1_cryptocurrency_marketpairs_latest)              | **GET** /v1/cryptocurrency/market-pairs/latest | Get market pairs (latest)                 |
| *PairsApi*            | [**get_v1_exchange_marketpairs_latest**](docs/PairsApi.md#get_v1_exchange_marketpairs_latest)                          | **GET** /v1/exchange/market-pairs/latest       | List all market pairs (latest)            |
| *PriceApi*            | [**get_v1_tools_priceconversion**](docs/PriceApi.md#get_v1_tools_priceconversion)                                      | **GET** /v1/tools/price-conversion             | Price conversion tool                     |
| *QuotesApi*           | [**get_v1_cryptocurrency_quotes_historical**](docs/QuotesApi.md#get_v1_cryptocurrency_quotes_historical)               | **GET** /v1/cryptocurrency/quotes/historical   | Get market quotes (historical)            |
| *QuotesApi*           | [**get_v1_cryptocurrency_quotes_latest**](docs/QuotesApi.md#get_v1_cryptocurrency_quotes_latest)                       | **GET** /v1/cryptocurrency/quotes/latest       | Get market quotes (latest)                |
| *QuotesApi*           | [**get_v1_exchange_quotes_historical**](docs/QuotesApi.md#get_v1_exchange_quotes_historical)                           | **GET** /v1/exchange/quotes/historical         | Get market quotes (historical)            |
| *QuotesApi*           | [**get_v1_exchange_quotes_latest**](docs/QuotesApi.md#get_v1_exchange_quotes_latest)                                   | **GET** /v1/exchange/quotes/latest             | Get market quotes (latest)                |
| *ToolApi*             | [**get_v1_tools_priceconversion**](docs/ToolApi.md#get_v1_tools_priceconversion)                                       | **GET** /v1/tools/price-conversion             | Price conversion tool                     |
| *ValuesApi*           | [**get_v1_cryptocurrency_ohlcv_historical**](docs/ValuesApi.md#get_v1_cryptocurrency_ohlcv_historical)                 | **GET** /v1/cryptocurrency/ohlcv/historical    | Get OHLCV values (historical)             |


## Documentation for models

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## License

This project is licensed under the [Apache License 2.0](./LICENSE).


