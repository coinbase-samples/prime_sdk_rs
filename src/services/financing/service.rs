/**
 * Copyright 2025-present Coinbase Global, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::client::PrimeClient;
use crate::error::HttpResult;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::*;

use crate::types::generated::generated::{
    create_new_locates_response::CreateNewLocatesResponse as GeneratedCreateNewLocatesResponse,
    get_buying_power_response::GetBuyingPowerResponse as GeneratedGetBuyingPowerResponse,
    get_existing_locates_response::GetExistingLocatesResponse as GeneratedGetExistingLocatesResponse,
    get_interest_accruals_response::GetInterestAccrualsResponse as GeneratedGetInterestAccrualsResponse,
    get_locate_availabilities_response::GetLocateAvailabilitiesResponse as GeneratedGetLocateAvailabilitiesResponse,
    get_margin_conversions_response::GetMarginConversionsResponse as GeneratedGetMarginConversionsResponse,
    get_margin_information_response::GetMarginInformationResponse as GeneratedGetMarginInformationResponse,
    get_margin_summaries_response::GetMarginSummariesResponse as GeneratedGetMarginSummariesResponse,
    get_portfolio_interest_accruals_response::GetPortfolioInterestAccrualsResponse as GeneratedGetPortfolioInterestAccrualsResponse,
    get_post_trade_credit_response::GetPostTradeCreditResponse as GeneratedGetPostTradeCreditResponse,
    get_tf_tiered_pricing_fees_response::GetTfTieredPricingFeesResponse as GeneratedGetTfTieredPricingFeesResponse,
    get_withdrawal_power_response::GetWithdrawalPowerResponse as GeneratedGetWithdrawalPowerResponse,
    get_cross_margin_overview_response::GetCrossMarginOverviewResponse as GeneratedGetCrossMarginOverviewResponse,
    get_cross_margin_risk_parameters_response::GetCrossMarginRiskParametersResponse as GeneratedGetCrossMarginRiskParametersResponse,
    get_cross_margin_prime_overview_response::GetCrossMarginPrimeOverviewResponse as GeneratedGetCrossMarginPrimeOverviewResponse,
    list_financing_eligible_assets_response::ListFinancingEligibleAssetsResponse as GeneratedListFinancingEligibleAssetsResponse,
    list_tf_obligations_response::ListTfObligationsResponse as GeneratedListTfObligationsResponse,
    get_market_data_response::GetMarketDataResponse as GeneratedGetMarketDataResponse,
    set_funding_settings_response::SetFundingSettingsResponse as GeneratedSetFundingSettingsResponse,
};

/// Service for interacting with financing-related endpoints
pub struct FinancingService {
    client: Box<dyn HttpClient>,
}

impl FinancingService {
    /// Create a new FinancingService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List interest accruals for an entity
    pub async fn list_interest_accruals(
        &self,
        request: ListInterestAccrualsRequest,
    ) -> HttpResult<ListInterestAccrualsResponse> {
        let path = format!("entities/{}/accruals", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(portfolio_id) = &request.portfolio_id {
            query_params.insert("portfolio_id".to_string(), portfolio_id.clone());
        }
        if let Some(start_date) = &request.start_date {
            query_params.insert("start_date".to_string(), start_date.clone());
        }
        if let Some(end_date) = &request.end_date {
            query_params.insert("end_date".to_string(), end_date.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetInterestAccrualsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List interest accruals for a portfolio
    pub async fn list_portfolio_interest_accruals(
        &self,
        request: ListPortfolioInterestAccrualsRequest,
    ) -> HttpResult<ListPortfolioInterestAccrualsResponse> {
        let path = format!("portfolios/{}/accruals", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(start_date) = &request.start_date {
            query_params.insert("start_date".to_string(), start_date.clone());
        }
        if let Some(end_date) = &request.end_date {
            query_params.insert("end_date".to_string(), end_date.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPortfolioInterestAccrualsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List margin call summaries for an entity
    pub async fn list_margin_summaries(
        &self,
        request: ListMarginSummariesRequest,
    ) -> HttpResult<ListMarginSummariesResponse> {
        let path = format!("entities/{}/margin_summaries", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(start_date) = &request.start_date {
            query_params.insert("start_date".to_string(), start_date.clone());
        }
        if let Some(end_date) = &request.end_date {
            query_params.insert("end_date".to_string(), end_date.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetMarginSummariesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List locate availabilities for an entity
    pub async fn list_locate_availabilities(
        &self,
        request: ListLocateAvailabilitiesRequest,
    ) -> HttpResult<ListLocateAvailabilitiesResponse> {
        let path = format!("entities/{}/locates_availability", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(conversion_date) = &request.conversion_date {
            query_params.insert("conversion_date".to_string(), conversion_date.clone());
        }
        if let Some(locate_date) = &request.locate_date {
            query_params.insert("locate_date".to_string(), locate_date.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetLocateAvailabilitiesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List trade finance tiered pricing fees for an entity
    pub async fn list_tf_tiered_pricing_fees(
        &self,
        request: ListTFTieredPricingFeesRequest,
    ) -> HttpResult<ListTFTieredPricingFeesResponse> {
        let path = format!("entities/{}/tf_tiered_fees", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(effective_at) = &request.effective_at {
            query_params.insert("effective_at".to_string(), effective_at.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetTfTieredPricingFeesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get margin information for an entity
    pub async fn get_margin_information(
        &self,
        request: GetMarginInformationRequest,
    ) -> HttpResult<GetMarginInformationResponse> {
        let path = format!("entities/{}/margin", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetMarginInformationResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get buying power for a portfolio
    pub async fn get_buying_power(
        &self,
        request: GetBuyingPowerRequest,
    ) -> HttpResult<GetBuyingPowerResponse> {
        let path = format!("portfolios/{}/buying_power", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        query_params.insert("base_currency".to_string(), request.base_currency.clone());
        query_params.insert("quote_currency".to_string(), request.quote_currency.clone());
        req = req.with_query_params(query_params);
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetBuyingPowerResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get post-trade credit for a portfolio
    pub async fn get_post_trade_credit(
        &self,
        request: GetPostTradeCreditRequest,
    ) -> HttpResult<GetPostTradeCreditResponse> {
        let path = format!("portfolios/{}/credit", request.portfolio_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPostTradeCreditResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get withdrawal power for a portfolio
    pub async fn get_withdrawal_power(
        &self,
        request: GetWithdrawalPowerRequest,
    ) -> HttpResult<GetWithdrawalPowerResponse> {
        let path = format!("portfolios/{}/withdrawal_power", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        query_params.insert("symbol".to_string(), request.symbol.clone());
        req = req.with_query_params(query_params);
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetWithdrawalPowerResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List existing locates for a portfolio
    pub async fn list_existing_locates(
        &self,
        request: ListExistingLocatesRequest,
    ) -> HttpResult<ListExistingLocatesResponse> {
        let path = format!("portfolios/{}/locates", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(locate_date) = &request.locate_date {
            query_params.insert("locate_date".to_string(), locate_date.clone());
        }
        if let Some(locate_ids) = &request.locate_ids {
            for locate_id in locate_ids {
                query_params.insert("locate_ids".to_string(), locate_id.clone());
            }
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetExistingLocatesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List margin conversions for a portfolio
    pub async fn list_margin_conversions(
        &self,
        request: ListMarginConversionsRequest,
    ) -> HttpResult<ListMarginConversionsResponse> {
        let path = format!("portfolios/{}/margin_conversions", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(start_date) = &request.start_date {
            query_params.insert("start_date".to_string(), start_date.clone());
        }
        if let Some(end_date) = &request.end_date {
            query_params.insert("end_date".to_string(), end_date.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetMarginConversionsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Create new locates for a portfolio
    pub async fn create_new_locates(
        &self,
        request: CreateNewLocatesRequest,
    ) -> HttpResult<CreateNewLocatesResponse> {
        let path = format!("portfolios/{}/locates", request.portfolio_id);
        let body = crate::types::generated::generated::create_new_locates_request::CreateNewLocatesRequest::from(&request);
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateNewLocatesResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn get_cross_margin_overview(
        &self,
        request: GetCrossMarginOverviewRequest,
    ) -> HttpResult<GetCrossMarginOverviewResponse> {
        let path = format!("entities/{}/cross_margin", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn get_cross_margin_risk_parameters(
        &self,
        request: GetCrossMarginRiskParametersRequest,
    ) -> HttpResult<GetCrossMarginRiskParametersResponse> {
        let path = format!(
            "entities/{}/cross_margin/risk_parameters",
            request.entity_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn get_cross_margin_prime_overview(
        &self,
        request: GetCrossMarginPrimeOverviewRequest,
    ) -> HttpResult<GetCrossMarginPrimeOverviewResponse> {
        let path = format!("v2/entities/{}/cross_margin/prime", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn list_financing_eligible_assets(
        &self,
        _request: ListFinancingEligibleAssetsRequest,
    ) -> HttpResult<ListFinancingEligibleAssetsResponse> {
        let req = HttpRequest::new(HttpMethod::Get, "financing/eligible-assets")
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn list_tf_obligations(
        &self,
        request: ListTfObligationsRequest,
    ) -> HttpResult<ListTfObligationsResponse> {
        let path = format!("entities/{}/tf_obligations", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn get_market_data(
        &self,
        request: GetMarketDataRequest,
    ) -> HttpResult<GetMarketDataResponse> {
        let path = format!("entities/{}/market_data", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn set_funding_settings(
        &self,
        request: SetFundingSettingsRequest,
    ) -> HttpResult<SetFundingSettingsResponse> {
        let path = format!("entities/{}/funding_settings", request.entity_id);
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }
}
