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
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use serde_json;
use std::collections::HashMap;

use super::types::{
    CreateWalletDepositAddressRequest, CreateWalletDepositAddressResponse, CreateWalletRequest,
    CreateWalletResponse, GetWalletDepositInstructionsRequest,
    GetWalletDepositInstructionsResponse, GetWalletRequest, GetWalletResponse,
    ListWalletAddressesRequest, ListWalletAddressesResponse, ListWalletsRequest,
    ListWalletsResponse,
};

use crate::types::generated::generated::{
    blockchain_address::BlockchainAddress as GeneratedBlockchainAddress,
    create_wallet_deposit_address_request::CreateWalletDepositAddressRequest as GeneratedCreateWalletDepositAddressRequest,
    create_wallet_request::CreateWalletRequest as GeneratedCreateWalletRequest,
    create_wallet_response::CreateWalletResponse as GeneratedCreateWalletResponse,
    get_wallet_deposit_instructions_response::GetWalletDepositInstructionsResponse as GeneratedGetWalletDepositInstructionsResponse,
    get_wallet_response::GetWalletResponse as GeneratedGetWalletResponse,
    get_wallets_response::GetWalletsResponse as GeneratedGetWalletsResponse,
    list_wallet_addresses_response::ListWalletAddressesResponse as GeneratedListWalletAddressesResponse,
};

/// Service for managing wallets
pub struct WalletsService {
    client: Box<dyn HttpClient>,
}

impl WalletsService {
    /// Create a new wallets service
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List all wallets for a given portfolio
    pub async fn list_wallets(
        &self,
        request: ListWalletsRequest,
    ) -> crate::error::HttpResult<ListWalletsResponse> {
        let path = format!("portfolios/{}/wallets", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let mut query_params = HashMap::new();
        if let Some(cursor) = &request.cursor {
            if !cursor.is_empty() {
                query_params.insert("cursor".to_string(), cursor.clone());
            }
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetWalletsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get a wallet by ID
    pub async fn get_wallet(
        &self,
        request: GetWalletRequest,
    ) -> crate::error::HttpResult<GetWalletResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}",
            request.portfolio_id, request.wallet_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetWalletResponse = resp.json().await?;
        Ok(response)
    }

    /// Create a new wallet
    pub async fn create_wallet(
        &self,
        portfolio_id: String,
        request: CreateWalletRequest,
    ) -> crate::error::HttpResult<CreateWalletResponse> {
        let path = format!("portfolios/{}/wallets", portfolio_id);
        let gen_req: GeneratedCreateWalletRequest = request.into();
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(gen_req)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateWalletResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get deposit instructions for a wallet
    pub async fn get_wallet_deposit_instructions(
        &self,
        request: GetWalletDepositInstructionsRequest,
    ) -> crate::error::HttpResult<GetWalletDepositInstructionsResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/deposit_instructions",
            request.portfolio_id, request.wallet_id
        );
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let mut query_params = HashMap::new();
        if let Some(deposit_type) = &request.deposit_type {
            query_params.insert("deposit_type".to_string(), deposit_type.to_string());
        }
        if let Some(network_id) = &request.network_id {
            if !network_id.is_empty() {
                query_params.insert("network.id".to_string(), network_id.clone());
            }
        }
        if let Some(network_type) = &request.network_type {
            if !network_type.is_empty() {
                query_params.insert("network.type".to_string(), network_type.clone());
            }
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetWalletDepositInstructionsResponse = resp.json().await?;
        Ok(response)
    }

    /// List all deposit addresses for a given wallet
    pub async fn list_wallet_addresses(
        &self,
        request: ListWalletAddressesRequest,
    ) -> crate::error::HttpResult<ListWalletAddressesResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/addresses",
            request.portfolio_id, request.wallet_id
        );
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("network_id".to_string(), request.network_id.clone());
        if let Some(cursor) = &request.cursor {
            if !cursor.is_empty() {
                query_params.insert("cursor".to_string(), cursor.clone());
            }
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        req = req.with_query_params(query_params);
        let resp = self.client.execute(req).await?;
        let response: GeneratedListWalletAddressesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Create a new deposit address for a wallet
    pub async fn create_wallet_deposit_address(
        &self,
        request: CreateWalletDepositAddressRequest,
    ) -> crate::error::HttpResult<CreateWalletDepositAddressResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/addresses",
            request.portfolio_id, request.wallet_id
        );
        let gen_req = GeneratedCreateWalletDepositAddressRequest {
            network_id: request.network_id.clone(),
        };
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(gen_req)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        let response: GeneratedBlockchainAddress = resp.json().await?;
        Ok(response)
    }
}
