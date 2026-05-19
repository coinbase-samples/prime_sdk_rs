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
use crate::types::generated::generated::{
    get_staking_status_response::GetStakingStatusResponse as GeneratedGetStakingStatusResponse,
    get_unstaking_status_response::GetUnstakingStatusResponse as GeneratedGetUnstakingStatusResponse,
    list_transaction_validators_request::ListTransactionValidatorsRequest as GeneratedListTransactionValidatorsRequest,
    list_transaction_validators_response::ListTransactionValidatorsResponse as GeneratedListTransactionValidatorsResponse,
    portfolio_staking_initiate_request::PortfolioStakingInitiateRequest as GeneratedPortfolioStakingInitiateRequest,
    portfolio_staking_initiate_response::PortfolioStakingInitiateResponse as GeneratedPortfolioStakingInitiateResponse,
    portfolio_staking_unstake_request::PortfolioStakingUnstakeRequest as GeneratedPortfolioStakingUnstakeRequest,
    portfolio_staking_unstake_response::PortfolioStakingUnstakeResponse as GeneratedPortfolioStakingUnstakeResponse,
    preview_unstake_request::PreviewUnstakeRequest as GeneratedPreviewUnstakeRequest,
    preview_unstake_response::PreviewUnstakeResponse as GeneratedPreviewUnstakeResponse,
    staking_claim_rewards_request::StakingClaimRewardsRequest as GeneratedStakingClaimRewardsRequest,
    staking_claim_rewards_response::StakingClaimRewardsResponse as GeneratedStakingClaimRewardsResponse,
    staking_initiate_request::StakingInitiateRequest as GeneratedStakingInitiateRequest,
    staking_initiate_response::StakingInitiateResponse as GeneratedStakingInitiateResponse,
    staking_unstake_request::StakingUnstakeRequest as GeneratedStakingUnstakeRequest,
    staking_unstake_response::StakingUnstakeResponse as GeneratedStakingUnstakeResponse,
};

#[derive(Debug, Clone)]
pub struct PortfolioStakingInitiateRequest {
    pub portfolio_id: String,
    pub body: GeneratedPortfolioStakingInitiateRequest,
}

impl PortfolioStakingInitiateRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        body: GeneratedPortfolioStakingInitiateRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            body,
        }
    }
}

pub type PortfolioStakingInitiateResponse = GeneratedPortfolioStakingInitiateResponse;

#[derive(Debug, Clone)]
pub struct PortfolioStakingUnstakeRequest {
    pub portfolio_id: String,
    pub body: GeneratedPortfolioStakingUnstakeRequest,
}

impl PortfolioStakingUnstakeRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        body: GeneratedPortfolioStakingUnstakeRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            body,
        }
    }
}

pub type PortfolioStakingUnstakeResponse = GeneratedPortfolioStakingUnstakeResponse;

#[derive(Debug, Clone)]
pub struct ListTransactionValidatorsRequest {
    pub portfolio_id: String,
    pub body: GeneratedListTransactionValidatorsRequest,
}

impl ListTransactionValidatorsRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        body: GeneratedListTransactionValidatorsRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            body,
        }
    }
}

pub type ListTransactionValidatorsResponse = GeneratedListTransactionValidatorsResponse;

#[derive(Debug, Clone)]
pub struct WalletStakingInitiateRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub body: GeneratedStakingInitiateRequest,
}

impl WalletStakingInitiateRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        wallet_id: impl Into<String>,
        body: GeneratedStakingInitiateRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            body,
        }
    }
}

pub type WalletStakingInitiateResponse = GeneratedStakingInitiateResponse;

#[derive(Debug, Clone)]
pub struct StakingClaimRewardsRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub body: GeneratedStakingClaimRewardsRequest,
}

impl StakingClaimRewardsRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        wallet_id: impl Into<String>,
        body: GeneratedStakingClaimRewardsRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            body,
        }
    }
}

pub type StakingClaimRewardsResponse = GeneratedStakingClaimRewardsResponse;

#[derive(Debug, Clone)]
pub struct GetStakingStatusRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
}

impl GetStakingStatusRequest {
    pub fn new(portfolio_id: impl Into<String>, wallet_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
        }
    }
}

pub type GetStakingStatusResponse = GeneratedGetStakingStatusResponse;

#[derive(Debug, Clone)]
pub struct WalletStakingUnstakeRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub body: GeneratedStakingUnstakeRequest,
}

impl WalletStakingUnstakeRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        wallet_id: impl Into<String>,
        body: GeneratedStakingUnstakeRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            body,
        }
    }
}

pub type WalletStakingUnstakeResponse = GeneratedStakingUnstakeResponse;

#[derive(Debug, Clone)]
pub struct PreviewUnstakeRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub body: GeneratedPreviewUnstakeRequest,
}

impl PreviewUnstakeRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        wallet_id: impl Into<String>,
        body: GeneratedPreviewUnstakeRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            body,
        }
    }
}

pub type PreviewUnstakeResponse = GeneratedPreviewUnstakeResponse;

#[derive(Debug, Clone)]
pub struct GetUnstakingStatusRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
}

impl GetUnstakingStatusRequest {
    pub fn new(portfolio_id: impl Into<String>, wallet_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
        }
    }
}

pub type GetUnstakingStatusResponse = GeneratedGetUnstakingStatusResponse;
