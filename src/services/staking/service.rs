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
    get_staking_status_response::GetStakingStatusResponse as GeneratedGetStakingStatusResponse,
    get_unstaking_status_response::GetUnstakingStatusResponse as GeneratedGetUnstakingStatusResponse,
    preview_unstake_response::PreviewUnstakeResponse as GeneratedPreviewUnstakeResponse,
    staking_claim_rewards_response::StakingClaimRewardsResponse as GeneratedStakingClaimRewardsResponse,
    staking_initiate_response::StakingInitiateResponse as GeneratedStakingInitiateResponse,
    staking_unstake_response::StakingUnstakeResponse as GeneratedStakingUnstakeResponse,
};
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;

use super::types::*;

pub struct StakingService {
    client: Box<dyn HttpClient>,
}

impl StakingService {
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    pub async fn portfolio_staking_initiate(
        &self,
        request: PortfolioStakingInitiateRequest,
    ) -> HttpResult<PortfolioStakingInitiateResponse> {
        let path = format!("portfolios/{}/staking/initiate", request.portfolio_id);
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn portfolio_staking_unstake(
        &self,
        request: PortfolioStakingUnstakeRequest,
    ) -> HttpResult<PortfolioStakingUnstakeResponse> {
        let path = format!("portfolios/{}/staking/unstake", request.portfolio_id);
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn list_transaction_validators(
        &self,
        request: ListTransactionValidatorsRequest,
    ) -> HttpResult<ListTransactionValidatorsResponse> {
        let path = format!(
            "portfolios/{}/staking/transaction-validators/query",
            request.portfolio_id
        );
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        Ok(resp.json().await?)
    }

    pub async fn wallet_staking_initiate(
        &self,
        request: WalletStakingInitiateRequest,
    ) -> HttpResult<WalletStakingInitiateResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/staking/initiate",
            request.portfolio_id, request.wallet_id
        );
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedStakingInitiateResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn staking_claim_rewards(
        &self,
        request: StakingClaimRewardsRequest,
    ) -> HttpResult<StakingClaimRewardsResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/staking/claim_rewards",
            request.portfolio_id, request.wallet_id
        );
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedStakingClaimRewardsResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn get_staking_status(
        &self,
        request: GetStakingStatusRequest,
    ) -> HttpResult<GetStakingStatusResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/staking/status",
            request.portfolio_id, request.wallet_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetStakingStatusResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn wallet_staking_unstake(
        &self,
        request: WalletStakingUnstakeRequest,
    ) -> HttpResult<WalletStakingUnstakeResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/staking/unstake",
            request.portfolio_id, request.wallet_id
        );
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedStakingUnstakeResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn preview_unstake(
        &self,
        request: PreviewUnstakeRequest,
    ) -> HttpResult<PreviewUnstakeResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/staking/unstake/preview",
            request.portfolio_id, request.wallet_id
        );
        let json_body = serde_json::to_value(&request.body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedPreviewUnstakeResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn get_unstaking_status(
        &self,
        request: GetUnstakingStatusRequest,
    ) -> HttpResult<GetUnstakingStatusResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/staking/unstake/status",
            request.portfolio_id, request.wallet_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetUnstakingStatusResponse = resp.json().await?;
        Ok(response)
    }
}
