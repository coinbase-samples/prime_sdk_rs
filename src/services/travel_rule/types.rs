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
    get_transaction_travel_rule_data_response::GetTransactionTravelRuleDataResponse as GeneratedGetTransactionTravelRuleDataResponse,
    request_to_submit_travel_rule_data_for_an_existing_deposit_transaction::RequestToSubmitTravelRuleDataForAnExistingDepositTransaction as GeneratedSubmitDepositTravelRuleDataRequest,
    submit_deposit_travel_rule_data_response::SubmitDepositTravelRuleDataResponse as GeneratedSubmitDepositTravelRuleDataResponse,
};

#[derive(Debug, Clone)]
pub struct GetTransactionTravelRuleDataRequest {
    pub portfolio_id: String,
    pub transaction_id: String,
}

impl GetTransactionTravelRuleDataRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        transaction_id: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            transaction_id: transaction_id.into(),
        }
    }
}

pub type GetTransactionTravelRuleDataResponse = GeneratedGetTransactionTravelRuleDataResponse;

#[derive(Debug, Clone)]
pub struct SubmitDepositTravelRuleDataRequest {
    pub portfolio_id: String,
    pub transaction_id: String,
    pub body: GeneratedSubmitDepositTravelRuleDataRequest,
}

impl SubmitDepositTravelRuleDataRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        transaction_id: impl Into<String>,
        body: GeneratedSubmitDepositTravelRuleDataRequest,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            transaction_id: transaction_id.into(),
            body,
        }
    }
}

pub type SubmitDepositTravelRuleDataResponse = GeneratedSubmitDepositTravelRuleDataResponse;
