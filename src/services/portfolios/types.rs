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
pub use crate::types::generated::generated::get_portfolio_response::GetPortfolioResponse;
pub use crate::types::generated::generated::get_portfolios_response::GetPortfoliosResponse as ListPortfoliosResponse;
pub use crate::types::generated::generated::portfolio::Portfolio;

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request parameters for getting a specific portfolio
#[derive(Debug, Clone)]
pub struct GetPortfolioRequest {
    pub portfolio_id: String,
}

impl GetPortfolioRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
        }
    }
}
