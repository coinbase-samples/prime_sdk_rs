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
use super::types::{
    GetTransactionRequest, GetTransactionResponse, ListPortfolioTransactionsRequest,
    ListPortfolioTransactionsResponse, ListWalletTransactionsRequest,
    ListWalletTransactionsResponse,
};
use crate::client::PrimeClient;
use crate::error::HttpResult;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use serde_json;
use std::collections::HashMap;

// Aliased imports for generated types
use crate::types::generated::generated::{
    create_a_transfer_between_two_wallets::CreateATransferBetweenTwoWallets as GeneratedCreateWalletTransferRequest,
    create_conversion_request::CreateConversionRequest as GeneratedCreateConversionRequest,
    create_conversion_response::CreateConversionResponse as GeneratedCreateConversionResponse,
    create_onchain_transaction_request::CreateOnchainTransactionRequest as GeneratedCreateOnchainTransactionRequest,
    create_onchain_transaction_response::CreateOnchainTransactionResponse as GeneratedCreateOnchainTransactionResponse,
    create_wallet_transfer_response::CreateWalletTransferResponse as GeneratedCreateWalletTransferResponse,
    create_wallet_withdrawal_request::CreateWalletWithdrawalRequest as GeneratedCreateWalletWithdrawalRequest,
    create_wallet_withdrawal_response::CreateWalletWithdrawalResponse as GeneratedCreateWalletWithdrawalResponse,
    get_portfolio_transactions_response::GetPortfolioTransactionsResponse as GeneratedGetPortfolioTransactionsResponse,
    get_transaction_response::GetTransactionResponse as GeneratedGetTransactionResponse,
    get_wallet_transactions_response::GetWalletTransactionsResponse as GeneratedGetWalletTransactionsResponse,
};

pub struct TransactionsService {
    client: Box<dyn HttpClient>,
}

impl TransactionsService {
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List transactions for a portfolio
    pub async fn list_portfolio_transactions(
        &self,
        request: ListPortfolioTransactionsRequest,
    ) -> HttpResult<ListPortfolioTransactionsResponse> {
        let path = format!("portfolios/{}/transactions", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(symbols) = &request.symbols {
            query_params.insert("symbols".to_string(), symbols.join(","));
        }
        if let Some(types) = &request.types {
            let type_strs: Vec<_> = types.iter().map(|t| t.to_string()).collect();
            query_params.insert("types".to_string(), type_strs.join(","));
        }
        if let Some(start_time) = &request.start_time {
            query_params.insert("start_time".to_string(), start_time.clone());
        }
        if let Some(end_time) = &request.end_time {
            query_params.insert("end_time".to_string(), end_time.clone());
        }
        if let Some(cursor) = &request.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(sort_direction) = &request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPortfolioTransactionsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get a transaction by ID
    pub async fn get_transaction(
        &self,
        request: GetTransactionRequest,
    ) -> HttpResult<GetTransactionResponse> {
        let path = format!(
            "portfolios/{}/transactions/{}",
            request.portfolio_id, request.transaction_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetTransactionResponse = resp.json().await?;
        Ok(response)
    }

    /// List transactions for a wallet
    pub async fn list_wallet_transactions(
        &self,
        request: ListWalletTransactionsRequest,
    ) -> HttpResult<ListWalletTransactionsResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/transactions",
            request.portfolio_id, request.wallet_id
        );
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(types) = &request.types {
            let type_strs: Vec<_> = types.iter().map(|t| t.to_string()).collect();
            query_params.insert("types".to_string(), type_strs.join(","));
        }
        if let Some(start_time) = &request.start_time {
            query_params.insert("start_time".to_string(), start_time.clone());
        }
        if let Some(end_time) = &request.end_time {
            query_params.insert("end_time".to_string(), end_time.clone());
        }
        if let Some(cursor) = &request.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(sort_direction) = &request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetWalletTransactionsResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn create_onchain_transaction(
        &self,
        portfolio_id: &str,
        wallet_id: &str,
        request: super::types::CreateOnchainTransactionRequest,
    ) -> crate::error::HttpResult<super::types::CreateOnchainTransactionResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/onchain_transaction",
            portfolio_id, wallet_id
        );
        let gen_req: GeneratedCreateOnchainTransactionRequest = request.into();
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(gen_req)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateOnchainTransactionResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn create_wallet_transfer(
        &self,
        portfolio_id: &str,
        wallet_id: &str,
        request: super::types::CreateWalletTransferRequest,
    ) -> crate::error::HttpResult<super::types::CreateWalletTransferResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/transfers",
            portfolio_id, wallet_id
        );
        let gen_req: GeneratedCreateWalletTransferRequest = request.into();
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(gen_req)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateWalletTransferResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn create_wallet_withdrawal(
        &self,
        portfolio_id: &str,
        wallet_id: &str,
        request: super::types::CreateWalletWithdrawalRequest,
    ) -> crate::error::HttpResult<super::types::CreateWalletWithdrawalResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/withdrawals",
            portfolio_id, wallet_id
        );
        let gen_req: GeneratedCreateWalletWithdrawalRequest = request.into();
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(gen_req)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        println!("🔍 Response: {:#?}", resp);
        let response: GeneratedCreateWalletWithdrawalResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn create_conversion(
        &self,
        portfolio_id: &str,
        wallet_id: &str,
        request: super::types::CreateConversionRequest,
    ) -> crate::error::HttpResult<super::types::CreateConversionResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/conversion",
            portfolio_id, wallet_id
        );
        let gen_req: GeneratedCreateConversionRequest = request.into();
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(gen_req)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateConversionResponse = resp.json().await?;
        Ok(response)
    }
}
