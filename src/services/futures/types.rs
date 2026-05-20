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
    fcm_futures_sweep::FcmFuturesSweep, fcm_position::FcmPosition,
    get_fcm_balance_response::GetFcmBalanceResponse as GeneratedGetFcmBalanceResponse,
    get_fcm_equity_response::GetFcmEquityResponse as GeneratedGetFcmEquityResponse,
    get_fcm_margin_call_details_response::GetFcmMarginCallDetailsResponse as GeneratedGetFcmMarginCallDetailsResponse,
    get_fcm_risk_limits_response::GetFcmRiskLimitsResponse as GeneratedGetFcmRiskLimitsResponse,
    get_fcm_settings_response::GetFcmSettingsResponse as GeneratedGetFcmSettingsResponse,
    get_futures_sweeps_response::GetFuturesSweepsResponse as GeneratedGetFuturesSweepsResponse,
    get_positions_response::GetPositionsResponse as GeneratedGetPositionsResponse,
    schedule_futures_sweep_request::ScheduleFuturesSweepRequest as GeneratedScheduleFuturesSweepRequest,
    schedule_futures_sweep_response::ScheduleFuturesSweepResponse as GeneratedScheduleFuturesSweepResponse,
    set_auto_sweep_request::SetAutoSweepRequest as GeneratedSetAutoSweepRequest,
    set_auto_sweep_response::SetAutoSweepResponse as GeneratedSetAutoSweepResponse,
    set_fcm_settings_request::SetFcmSettingsRequest as GeneratedSetFcmSettingsRequest,
    set_fcm_settings_response::SetFcmSettingsResponse as GeneratedSetFcmSettingsResponse,
};

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct GetFcmBalanceRequest {
    pub entity_id: String,
}
impl GetFcmBalanceRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetPositionsRequest {
    pub entity_id: String,
    pub product_id: Option<String>,
}
impl GetPositionsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            product_id: None,
        }
    }
    pub fn with_product_id(mut self, product_id: impl Into<String>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetFuturesSweepsRequest {
    pub entity_id: String,
}
impl GetFuturesSweepsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetAutoSweepRequest {
    pub entity_id: String,
    pub auto_sweep: bool,
}
impl SetAutoSweepRequest {
    pub fn new(entity_id: impl Into<String>, auto_sweep: bool) -> Self {
        Self {
            entity_id: entity_id.into(),
            auto_sweep,
        }
    }
}
impl From<&SetAutoSweepRequest> for GeneratedSetAutoSweepRequest {
    fn from(req: &SetAutoSweepRequest) -> Self {
        GeneratedSetAutoSweepRequest {
            auto_sweep: req.auto_sweep,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleFuturesSweepRequest {
    pub entity_id: String,
    pub currency: String,
    pub amount: Option<String>,
}
impl ScheduleFuturesSweepRequest {
    pub fn new(entity_id: impl Into<String>, currency: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            currency: currency.into(),
            amount: None,
        }
    }
    pub fn with_amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }
}
impl From<&ScheduleFuturesSweepRequest> for GeneratedScheduleFuturesSweepRequest {
    fn from(req: &ScheduleFuturesSweepRequest) -> Self {
        GeneratedScheduleFuturesSweepRequest {
            currency: req.currency.clone(),
            amount: req.amount.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CancelFuturesSweepRequest {
    pub entity_id: String,
}
impl CancelFuturesSweepRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct GetFcmBalanceResponse {
    pub portfolio_id: Option<String>,
    pub cfm_usd_balance: Option<String>,
    pub unrealized_pnl: Option<String>,
    pub daily_realized_pnl: Option<String>,
    pub excess_liquidity: Option<String>,
    pub futures_buying_power: Option<String>,
    pub initial_margin: Option<String>,
    pub maintenance_margin: Option<String>,
    pub clearing_account_id: Option<String>,
}
impl From<GeneratedGetFcmBalanceResponse> for GetFcmBalanceResponse {
    fn from(g: GeneratedGetFcmBalanceResponse) -> Self {
        Self {
            portfolio_id: g.portfolio_id,
            cfm_usd_balance: g.cfm_usd_balance,
            unrealized_pnl: g.unrealized_pnl,
            daily_realized_pnl: g.daily_realized_pnl,
            excess_liquidity: g.excess_liquidity,
            futures_buying_power: g.futures_buying_power,
            initial_margin: g.initial_margin,
            maintenance_margin: g.maintenance_margin,
            clearing_account_id: g.clearing_account_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetPositionsResponse {
    pub positions: Vec<FcmPosition>,
    pub clearing_account_id: Option<String>,
}
impl From<GeneratedGetPositionsResponse> for GetPositionsResponse {
    fn from(g: GeneratedGetPositionsResponse) -> Self {
        Self {
            positions: g.positions.unwrap_or_default(),
            clearing_account_id: g.clearing_account_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetFuturesSweepsResponse {
    pub sweeps: Vec<FcmFuturesSweep>,
    pub auto_sweep: Option<bool>,
}
impl From<GeneratedGetFuturesSweepsResponse> for GetFuturesSweepsResponse {
    fn from(g: GeneratedGetFuturesSweepsResponse) -> Self {
        Self {
            sweeps: g.sweeps.unwrap_or_default(),
            auto_sweep: g.auto_sweep,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetAutoSweepResponse {
    pub success: Option<bool>,
}
impl From<GeneratedSetAutoSweepResponse> for SetAutoSweepResponse {
    fn from(g: GeneratedSetAutoSweepResponse) -> Self {
        Self { success: g.success }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleFuturesSweepResponse {
    pub success: Option<bool>,
    pub request_id: Option<String>,
}
impl From<GeneratedScheduleFuturesSweepResponse> for ScheduleFuturesSweepResponse {
    fn from(g: GeneratedScheduleFuturesSweepResponse) -> Self {
        Self {
            success: g.success,
            request_id: g.request_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CancelFuturesSweepResponse {
    pub success: Option<bool>,
    pub request_id: Option<String>,
}
impl From<crate::types::generated::generated::cancel_futures_sweep_response::CancelFuturesSweepResponse> for CancelFuturesSweepResponse {
    fn from(g: crate::types::generated::generated::cancel_futures_sweep_response::CancelFuturesSweepResponse) -> Self {
        Self {
            success: g.success,
            request_id: g.request_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetFcmEquityRequest {
    pub entity_id: String,
}
impl GetFcmEquityRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}
pub type GetFcmEquityResponse = GeneratedGetFcmEquityResponse;

#[derive(Debug, Clone)]
pub struct GetFcmMarginCallDetailsRequest {
    pub entity_id: String,
}
impl GetFcmMarginCallDetailsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}
pub type GetFcmMarginCallDetailsResponse = GeneratedGetFcmMarginCallDetailsResponse;

#[derive(Debug, Clone)]
pub struct GetFcmRiskLimitsRequest {
    pub entity_id: String,
}
impl GetFcmRiskLimitsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}
pub type GetFcmRiskLimitsResponse = GeneratedGetFcmRiskLimitsResponse;

#[derive(Debug, Clone)]
pub struct GetFcmSettingsRequest {
    pub entity_id: String,
}
impl GetFcmSettingsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}
pub type GetFcmSettingsResponse = GeneratedGetFcmSettingsResponse;

#[derive(Debug, Clone)]
pub struct SetFcmSettingsRequest {
    pub entity_id: String,
    pub body: GeneratedSetFcmSettingsRequest,
}
impl SetFcmSettingsRequest {
    pub fn new(entity_id: impl Into<String>, body: GeneratedSetFcmSettingsRequest) -> Self {
        Self {
            entity_id: entity_id.into(),
            body,
        }
    }
}
pub type SetFcmSettingsResponse = GeneratedSetFcmSettingsResponse;
