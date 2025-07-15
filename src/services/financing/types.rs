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
    accrual::Accrual, buying_power::BuyingPower, conversion::Conversion,
    create_new_locates_request::CreateNewLocatesRequest as GeneratedCreateNewLocatesRequest,
    create_new_locates_response::CreateNewLocatesResponse as GeneratedCreateNewLocatesResponse,
    existing_locate::ExistingLocate, locate::Locate, margin_information::MarginInformation,
    margin_summary_historical::MarginSummaryHistorical,
    post_trade_credit_information::PostTradeCreditInformation,
    tiered_pricing_fee::TieredPricingFee, withdrawal_power::WithdrawalPower,
};

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct ListInterestAccrualsRequest {
    pub entity_id: String,
    pub portfolio_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
impl ListInterestAccrualsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            portfolio_id: None,
            start_date: None,
            end_date: None,
        }
    }
    pub fn with_portfolio_id(mut self, portfolio_id: impl Into<String>) -> Self {
        self.portfolio_id = Some(portfolio_id.into());
        self
    }
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ListPortfolioInterestAccrualsRequest {
    pub portfolio_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
impl ListPortfolioInterestAccrualsRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            start_date: None,
            end_date: None,
        }
    }
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ListMarginSummariesRequest {
    pub entity_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
impl ListMarginSummariesRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            start_date: None,
            end_date: None,
        }
    }
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ListLocateAvailabilitiesRequest {
    pub entity_id: String,
    pub conversion_date: Option<String>,
    pub locate_date: Option<String>,
}
impl ListLocateAvailabilitiesRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            conversion_date: None,
            locate_date: None,
        }
    }
    pub fn with_conversion_date(mut self, conversion_date: impl Into<String>) -> Self {
        self.conversion_date = Some(conversion_date.into());
        self
    }
    pub fn with_locate_date(mut self, locate_date: impl Into<String>) -> Self {
        self.locate_date = Some(locate_date.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ListTFTieredPricingFeesRequest {
    pub entity_id: String,
    pub effective_at: Option<String>,
}
impl ListTFTieredPricingFeesRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            effective_at: None,
        }
    }
    pub fn with_effective_at(mut self, effective_at: impl Into<String>) -> Self {
        self.effective_at = Some(effective_at.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetMarginInformationRequest {
    pub entity_id: String,
}
impl GetMarginInformationRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetBuyingPowerRequest {
    pub portfolio_id: String,
    pub base_currency: String,
    pub quote_currency: String,
}
impl GetBuyingPowerRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        base_currency: impl Into<String>,
        quote_currency: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            base_currency: base_currency.into(),
            quote_currency: quote_currency.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetPostTradeCreditRequest {
    pub portfolio_id: String,
}
impl GetPostTradeCreditRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetWithdrawalPowerRequest {
    pub portfolio_id: String,
    pub symbol: String,
}
impl GetWithdrawalPowerRequest {
    pub fn new(portfolio_id: impl Into<String>, symbol: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            symbol: symbol.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListExistingLocatesRequest {
    pub portfolio_id: String,
    pub locate_date: Option<String>,
    pub locate_ids: Option<Vec<String>>,
}
impl ListExistingLocatesRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            locate_date: None,
            locate_ids: None,
        }
    }
    pub fn with_locate_date(mut self, locate_date: impl Into<String>) -> Self {
        self.locate_date = Some(locate_date.into());
        self
    }
    pub fn with_locate_ids(mut self, locate_ids: Vec<String>) -> Self {
        self.locate_ids = Some(locate_ids);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ListMarginConversionsRequest {
    pub portfolio_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
impl ListMarginConversionsRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            start_date: None,
            end_date: None,
        }
    }
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct CreateNewLocatesRequest {
    pub portfolio_id: String,
    pub symbol: String,
    pub amount: String,
    pub conversion_date: Option<String>,
    pub locate_date: Option<String>,
}
impl CreateNewLocatesRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        symbol: impl Into<String>,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            symbol: symbol.into(),
            amount: amount.into(),
            conversion_date: None,
            locate_date: None,
        }
    }
    pub fn with_conversion_date(mut self, conversion_date: impl Into<String>) -> Self {
        self.conversion_date = Some(conversion_date.into());
        self
    }
    pub fn with_locate_date(mut self, locate_date: impl Into<String>) -> Self {
        self.locate_date = Some(locate_date.into());
        self
    }
}
impl From<&CreateNewLocatesRequest> for GeneratedCreateNewLocatesRequest {
    fn from(r: &CreateNewLocatesRequest) -> Self {
        Self {
            symbol: r.symbol.clone(),
            amount: r.amount.clone(),
            conversion_date: r.conversion_date.clone(),
            locate_date: r.locate_date.clone(),
        }
    }
}

// ============================================================================
// RESPONSE TYPE STRUCTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct ListInterestAccrualsResponse {
    pub total_notional_accrual: Option<String>,
    pub accruals: Vec<Accrual>,
}
impl From<crate::types::generated::generated::get_interest_accruals_response::GetInterestAccrualsResponse> for ListInterestAccrualsResponse {
    fn from(g: crate::types::generated::generated::get_interest_accruals_response::GetInterestAccrualsResponse) -> Self {
        Self {
            total_notional_accrual: g.total_notional_accrual,
            accruals: g.accruals.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListPortfolioInterestAccrualsResponse {
    pub total_notional_accrual: Option<String>,
    pub accruals: Vec<Accrual>,
}
impl From<crate::types::generated::generated::get_portfolio_interest_accruals_response::GetPortfolioInterestAccrualsResponse> for ListPortfolioInterestAccrualsResponse {
    fn from(g: crate::types::generated::generated::get_portfolio_interest_accruals_response::GetPortfolioInterestAccrualsResponse) -> Self {
        Self {
            total_notional_accrual: g.total_notional_accrual,
            accruals: g.accruals.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListMarginSummariesResponse {
    pub margin_summaries: Vec<MarginSummaryHistorical>,
}
impl From<crate::types::generated::generated::get_margin_summaries_response::GetMarginSummariesResponse> for ListMarginSummariesResponse {
    fn from(g: crate::types::generated::generated::get_margin_summaries_response::GetMarginSummariesResponse) -> Self {
        Self {
            margin_summaries: g.margin_summaries.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListLocateAvailabilitiesResponse {
    pub locates: Vec<Locate>,
}
impl From<crate::types::generated::generated::get_locate_availabilities_response::GetLocateAvailabilitiesResponse> for ListLocateAvailabilitiesResponse {
    fn from(g: crate::types::generated::generated::get_locate_availabilities_response::GetLocateAvailabilitiesResponse) -> Self {
        Self {
            locates: g.locates.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListTFTieredPricingFeesResponse {
    pub fees: Vec<TieredPricingFee>,
}
impl From<crate::types::generated::generated::get_tf_tiered_pricing_fees_response::GetTfTieredPricingFeesResponse> for ListTFTieredPricingFeesResponse {
    fn from(g: crate::types::generated::generated::get_tf_tiered_pricing_fees_response::GetTfTieredPricingFeesResponse) -> Self {
        Self {
            fees: g.fees.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetMarginInformationResponse {
    pub margin_information: Option<MarginInformation>,
}
impl From<crate::types::generated::generated::get_margin_information_response::GetMarginInformationResponse> for GetMarginInformationResponse {
    fn from(g: crate::types::generated::generated::get_margin_information_response::GetMarginInformationResponse) -> Self {
        Self {
            margin_information: g.margin_information.map(|b| *b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetBuyingPowerResponse {
    pub buying_power: Option<BuyingPower>,
}
impl From<crate::types::generated::generated::get_buying_power_response::GetBuyingPowerResponse>
    for GetBuyingPowerResponse
{
    fn from(
        g: crate::types::generated::generated::get_buying_power_response::GetBuyingPowerResponse,
    ) -> Self {
        Self {
            buying_power: g.buying_power.map(|b| *b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetPostTradeCreditResponse {
    pub post_trade_credit: Option<PostTradeCreditInformation>,
}
impl From<crate::types::generated::generated::get_post_trade_credit_response::GetPostTradeCreditResponse> for GetPostTradeCreditResponse {
    fn from(g: crate::types::generated::generated::get_post_trade_credit_response::GetPostTradeCreditResponse) -> Self {
        Self {
            post_trade_credit: g.post_trade_credit.map(|b| *b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetWithdrawalPowerResponse {
    pub withdrawal_power: Option<WithdrawalPower>,
}
impl From<crate::types::generated::generated::get_withdrawal_power_response::GetWithdrawalPowerResponse> for GetWithdrawalPowerResponse {
    fn from(g: crate::types::generated::generated::get_withdrawal_power_response::GetWithdrawalPowerResponse) -> Self {
        Self {
            withdrawal_power: g.withdrawal_power.map(|b| *b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListExistingLocatesResponse {
    pub locates: Vec<ExistingLocate>,
}
impl From<crate::types::generated::generated::get_existing_locates_response::GetExistingLocatesResponse> for ListExistingLocatesResponse {
    fn from(g: crate::types::generated::generated::get_existing_locates_response::GetExistingLocatesResponse) -> Self {
        Self {
            locates: g.locates.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListMarginConversionsResponse {
    pub conversions: Vec<Conversion>,
}
impl From<crate::types::generated::generated::get_margin_conversions_response::GetMarginConversionsResponse> for ListMarginConversionsResponse {
    fn from(g: crate::types::generated::generated::get_margin_conversions_response::GetMarginConversionsResponse) -> Self {
        Self {
            conversions: g.conversions.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateNewLocatesResponse {
    pub locate_id: Option<String>,
}
impl From<GeneratedCreateNewLocatesResponse> for CreateNewLocatesResponse {
    fn from(g: GeneratedCreateNewLocatesResponse) -> Self {
        Self {
            locate_id: g.locate_id,
        }
    }
}
