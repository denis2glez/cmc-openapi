/*
 * CoinMarketCap Get aggregate market metrics (latest)
 *
 * Get the latest quote of aggregate market metrics. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.  **This endpoint is available on the following API plans:** - Starter - Hobbyist - Standard - Professional - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `get_v1_cryptocurrency_listings_historical`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetV1CryptocurrencyListingsHistoricalError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_v1_cryptocurrency_listings_latest`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetV1CryptocurrencyListingsLatestError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_v1_exchange_listings_historical`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetV1ExchangeListingsHistoricalError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_v1_exchange_listings_latest`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetV1ExchangeListingsLatestError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_v1_exchange_marketpairs_latest`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetV1ExchangeMarketpairsLatestError {
    UnknownValue(serde_json::Value),
}

/// **This endpoint is not yet available. It is slated for release in Q3 2018.**   Get a paginated list of all cryptocurrencies with market data for a given historical time. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.
pub async fn get_v1_cryptocurrency_listings_historical(
    configuration: &configuration::Configuration,
    convert: Option<serde_json::Value>,
    cryptocurrency_type: Option<serde_json::Value>,
    limit: Option<serde_json::Value>,
    sort: Option<serde_json::Value>,
    sort_dir: Option<serde_json::Value>,
    start: Option<serde_json::Value>,
    timestamp: Option<serde_json::Value>,
) -> Result<(), Error<GetV1CryptocurrencyListingsHistoricalError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/cryptocurrency/listings/historical",
        configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = convert {
        local_var_req_builder =
            local_var_req_builder.query(&[("convert", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cryptocurrency_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("cryptocurrency_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_dir {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort_dir", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timestamp {
        local_var_req_builder =
            local_var_req_builder.query(&[("timestamp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetV1CryptocurrencyListingsHistoricalError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a paginated list of all cryptocurrencies with latest market data. You can configure this call to sort by market cap or another market ranking field. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.      Cryptocurrencies are listed by CoinMarketCap Rank by default. You may optionally sort against any of the following: **name**: The cryptocurrency name. **symbol**: The cryptocurrency symbol. **date_added**: Date cryptocurrency was added to the system. **market_cap**: market cap (latest trade price x circulating supply). **price**: latest average trade price across markets. **circulating_supply**: approximate number of coins currently in circulation. **total_supply**: approximate total amount of coins in existence right now (minus any coins that have been verifiably burned). **max_supply**: our best approximation of the maximum amount of coins that will ever exist in the lifetime of the currency. **num_market_pairs**: number of market pairs across all exchanges trading each currency. **volume_24h**: 24 hour trading volume for each currency. **percent_change_1h**: 1 hour trading price percentage change for each currency. **percent_change_24h**: 24 hour trading price percentage change for each currency. **percent_change_7d**: 7 day trading price percentage change for each currency.    **This endpoint is available on the following API plans:**   - Starter   - Hobbyist   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Every ~1 minute.     *NOTE: Use this endpoint if you need a sorted and paginated list of cryptocurrencies. If you want to query for market data on a few specific cryptocurrencies use /v1/cryptocurrency/quotes/latest which is optimized for that purpose. The response data between these endpoints is otherwise the same.*
pub async fn get_v1_cryptocurrency_listings_latest(
    configuration: &configuration::Configuration,
    convert: Option<serde_json::Value>,
    cryptocurrency_type: Option<serde_json::Value>,
    limit: Option<serde_json::Value>,
    sort: Option<serde_json::Value>,
    sort_dir: Option<serde_json::Value>,
    start: Option<serde_json::Value>,
) -> Result<(), Error<GetV1CryptocurrencyListingsLatestError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/cryptocurrency/listings/latest",
        configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = convert {
        local_var_req_builder =
            local_var_req_builder.query(&[("convert", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cryptocurrency_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("cryptocurrency_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_dir {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort_dir", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetV1CryptocurrencyListingsLatestError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// **This endpoint is not yet available. It is slated for release in Q3 2018.**   Get a paginated list of all cryptocurrency exchanges with historical market data for a given point in time. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.
pub async fn get_v1_exchange_listings_historical(
    configuration: &configuration::Configuration,
    convert: Option<serde_json::Value>,
    limit: Option<serde_json::Value>,
    market_type: Option<serde_json::Value>,
    sort: Option<serde_json::Value>,
    sort_dir: Option<serde_json::Value>,
    start: Option<serde_json::Value>,
    timestamp: Option<serde_json::Value>,
) -> Result<(), Error<GetV1ExchangeListingsHistoricalError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/exchange/listings/historical",
        configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = convert {
        local_var_req_builder =
            local_var_req_builder.query(&[("convert", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = market_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("market_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_dir {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort_dir", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timestamp {
        local_var_req_builder =
            local_var_req_builder.query(&[("timestamp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetV1ExchangeListingsHistoricalError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a paginated list of all cryptocurrency exchanges with 24 hour volume. Additional market data fields will be available in the future. You can configure this call to sort by 24 hour volume or another field. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.      **This endpoint is available on the following API plans:**   - ~~Starter~~   - ~~Hobbyist~~   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.        *NOTE: Use this endpoint if you need a sorted and paginated list of exchanges. If you want to query for market data on a few specific exchanges use /v1/exchange/quotes/latest which is optimized for that purpose. The response data between these endpoints is otherwise the same.*
pub async fn get_v1_exchange_listings_latest(
    configuration: &configuration::Configuration,
    convert: Option<serde_json::Value>,
    limit: Option<serde_json::Value>,
    market_type: Option<serde_json::Value>,
    sort: Option<serde_json::Value>,
    sort_dir: Option<serde_json::Value>,
    start: Option<serde_json::Value>,
) -> Result<(), Error<GetV1ExchangeListingsLatestError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/exchange/listings/latest", configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = convert {
        local_var_req_builder =
            local_var_req_builder.query(&[("convert", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = market_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("market_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_dir {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort_dir", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetV1ExchangeListingsLatestError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of active market pairs for an exchange. Active means the market pair is open for trading. Use the \"convert\" option to return market values in multiple fiat and cryptocurrency conversions in the same call.'    **This endpoint is available on the following API plans:**   - ~~Starter~~   - ~~Hobbyist~~   - Standard   - Professional   - Enterprise  **Cache / Update frequency:** Every ~5 minutes. This endpoint will be migrated to ~1 minute updates shortly.
pub async fn get_v1_exchange_marketpairs_latest(
    configuration: &configuration::Configuration,
    convert: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
    limit: Option<serde_json::Value>,
    slug: Option<serde_json::Value>,
    start: Option<serde_json::Value>,
) -> Result<(), Error<GetV1ExchangeMarketpairsLatestError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/exchange/market-pairs/latest",
        configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = convert {
        local_var_req_builder =
            local_var_req_builder.query(&[("convert", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = slug {
        local_var_req_builder =
            local_var_req_builder.query(&[("slug", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetV1ExchangeMarketpairsLatestError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
