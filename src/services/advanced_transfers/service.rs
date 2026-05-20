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
use crate::constants::DEFAULT_LIMIT;
use crate::error::HttpResult;
use crate::types::generated::generated::{
    cancel_advanced_transfer_response::CancelAdvancedTransferResponse as GeneratedCancelAdvancedTransferResponse,
    get_portfolio_counterparty_id_response::GetPortfolioCounterpartyIdResponse as GeneratedGetPortfolioCounterpartyIdResponse,
};
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::*;

pub struct AdvancedTransfersService {
    client: Box<dyn HttpClient>,
}

impl AdvancedTransfersService {
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    pub async fn list_advanced_transfers(
        &self,
        request: ListAdvancedTransfersRequest,
    ) -> HttpResult<ListAdvancedTransfersResponse> {
        let path = format!("portfolios/{}/advanced_transfers", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let mut query_params = HashMap::new();
        if let Some(state) = &request.state {
            query_params.insert("state".to_string(), state.to_string());
        }
        if let Some(transfer_type) = &request.transfer_type {
            query_params.insert("type".to_string(), transfer_type.to_string());
        }
        if let Some(cursor) = &request.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        } else {
            query_params.insert("limit".to_string(), DEFAULT_LIMIT.to_string());
        }
        if let Some(sort_direction) = &request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }
        if let Some(start_time) = &request.start_time {
            query_params.insert("start_time".to_string(), start_time.clone());
        }
        if let Some(end_time) = &request.end_time {
            query_params.insert("end_time".to_string(), end_time.clone());
        }
        if let Some(reference_id) = &request.reference_id {
            query_params.insert("reference_id".to_string(), reference_id.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn create_advanced_transfer(
        &self,
        request: CreateAdvancedTransferRequest,
    ) -> HttpResult<CreateAdvancedTransferResponse> {
        let path = format!("portfolios/{}/advanced_transfers", request.portfolio_id);
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn cancel_advanced_transfer(
        &self,
        request: CancelAdvancedTransferRequest,
    ) -> HttpResult<CancelAdvancedTransferResponse> {
        let path = format!(
            "portfolios/{}/advanced_transfers/{}/cancel",
            request.portfolio_id, request.advanced_transfer_id
        );
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(serde_json::json!({}));
        let resp = self.client.execute(req).await?;
        let response: GeneratedCancelAdvancedTransferResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn get_portfolio_counterparty_id(
        &self,
        request: GetPortfolioCounterpartyIdRequest,
    ) -> HttpResult<GetPortfolioCounterpartyIdResponse> {
        let path = format!("portfolios/{}/counterparty", request.portfolio_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPortfolioCounterpartyIdResponse = resp.json().await?;
        Ok(response)
    }
}
