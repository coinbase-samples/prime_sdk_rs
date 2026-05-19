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
    advanced_transfer_state::AdvancedTransferState,
    advanced_transfer_type::AdvancedTransferType,
    cancel_advanced_transfer_response::CancelAdvancedTransferResponse as GeneratedCancelAdvancedTransferResponse,
    create_advanced_transfer_request::CreateAdvancedTransferRequest as GeneratedCreateAdvancedTransferRequest,
    create_advanced_transfer_response::CreateAdvancedTransferResponse as GeneratedCreateAdvancedTransferResponse,
    get_portfolio_counterparty_id_response::GetPortfolioCounterpartyIdResponse as GeneratedGetPortfolioCounterpartyIdResponse,
    list_advanced_transfers_response::ListAdvancedTransfersResponse as GeneratedListAdvancedTransfersResponse,
    sort_direction::SortDirection,
};

#[derive(Debug, Clone, Default)]
pub struct ListAdvancedTransfersRequest {
    pub portfolio_id: String,
    pub state: Option<AdvancedTransferState>,
    pub transfer_type: Option<AdvancedTransferType>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub sort_direction: Option<SortDirection>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub reference_id: Option<String>,
}

impl ListAdvancedTransfersRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            ..Default::default()
        }
    }

    pub fn with_state(mut self, state: AdvancedTransferState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_transfer_type(mut self, transfer_type: AdvancedTransferType) -> Self {
        self.transfer_type = Some(transfer_type);
        self
    }

    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self
    }

    pub fn with_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    pub fn with_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    pub fn with_reference_id(mut self, reference_id: impl Into<String>) -> Self {
        self.reference_id = Some(reference_id.into());
        self
    }
}

pub type ListAdvancedTransfersResponse = GeneratedListAdvancedTransfersResponse;

#[derive(Debug, Clone)]
pub struct CreateAdvancedTransferRequest {
    pub portfolio_id: String,
    pub body: GeneratedCreateAdvancedTransferRequest,
}

impl CreateAdvancedTransferRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        body: GeneratedCreateAdvancedTransferRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            body,
        }
    }
}

pub type CreateAdvancedTransferResponse = GeneratedCreateAdvancedTransferResponse;

#[derive(Debug, Clone)]
pub struct CancelAdvancedTransferRequest {
    pub portfolio_id: String,
    pub advanced_transfer_id: String,
}

impl CancelAdvancedTransferRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        advanced_transfer_id: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            advanced_transfer_id: advanced_transfer_id.into(),
        }
    }
}

pub type CancelAdvancedTransferResponse = GeneratedCancelAdvancedTransferResponse;

#[derive(Debug, Clone)]
pub struct GetPortfolioCounterpartyIdRequest {
    pub portfolio_id: String,
}

impl GetPortfolioCounterpartyIdRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
        }
    }
}

pub type GetPortfolioCounterpartyIdResponse = GeneratedGetPortfolioCounterpartyIdResponse;
