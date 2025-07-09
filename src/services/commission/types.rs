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
use crate::types::generated::generated::{
    commission::Commission,
    get_portfolio_commission_response::GetPortfolioCommissionResponse as GeneratedGetPortfolioCommissionResponse,
};

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request to get portfolio commission
#[derive(Debug, Clone)]
pub struct GetPortfolioCommissionRequest {
    pub portfolio_id: String,
    pub product_id: Option<String>,
}

impl GetPortfolioCommissionRequest {
    /// Create a new request to get portfolio commission
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            product_id: None,
        }
    }

    /// Set the product ID for specific trading pair commission
    pub fn with_product_id(mut self, product_id: impl Into<String>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Response for getting portfolio commission
#[derive(Debug, Clone)]
pub struct GetPortfolioCommissionResponse {
    pub commission: Commission,
}

impl GetPortfolioCommissionResponse {
    /// Get the commission configuration
    pub fn commission(&self) -> &Commission {
        &self.commission
    }

    /// Get the commission type (all_in or cost_plus)
    pub fn commission_type(&self) -> Option<&str> {
        self.commission.r#type.as_deref()
    }

    /// Get the commission rate
    pub fn rate(&self) -> Option<&str> {
        self.commission.rate.as_deref()
    }

    /// Get the trading volume
    pub fn trading_volume(&self) -> Option<&str> {
        self.commission.trading_volume.as_deref()
    }
}

impl From<GeneratedGetPortfolioCommissionResponse> for GetPortfolioCommissionResponse {
    fn from(response: GeneratedGetPortfolioCommissionResponse) -> Self {
        Self {
            commission: *response.commission.unwrap(),
        }
    }
}
