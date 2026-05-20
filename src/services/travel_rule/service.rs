/**
 * Copyright 2026-present Coinbase Global, Inc.
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
use crate::types::generated::generated::{
    get_transaction_travel_rule_data_response::GetTransactionTravelRuleDataResponse as GeneratedGetTransactionTravelRuleDataResponse,
    submit_deposit_travel_rule_data_response::SubmitDepositTravelRuleDataResponse as GeneratedSubmitDepositTravelRuleDataResponse,
};
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;

use super::types::{
    GetTransactionTravelRuleDataRequest, GetTransactionTravelRuleDataResponse,
    SubmitDepositTravelRuleDataRequest, SubmitDepositTravelRuleDataResponse,
};

pub struct TravelRuleService {
    client: Box<dyn HttpClient>,
}

impl TravelRuleService {
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    pub async fn get_transaction_travel_rule_data(
        &self,
        request: GetTransactionTravelRuleDataRequest,
    ) -> HttpResult<GetTransactionTravelRuleDataResponse> {
        let path = format!(
            "portfolios/{}/transactions/{}/travel_rule",
            request.portfolio_id, request.transaction_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetTransactionTravelRuleDataResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn submit_deposit_travel_rule_data(
        &self,
        request: SubmitDepositTravelRuleDataRequest,
    ) -> HttpResult<SubmitDepositTravelRuleDataResponse> {
        let path = format!(
            "portfolios/{}/transactions/{}/travel_rule/deposit",
            request.portfolio_id, request.transaction_id
        );
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedSubmitDepositTravelRuleDataResponse = resp.json().await?;
        Ok(response)
    }
}
