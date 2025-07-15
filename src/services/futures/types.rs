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
    get_futures_sweeps_response::GetFuturesSweepsResponse as GeneratedGetFuturesSweepsResponse,
    get_positions_response::GetPositionsResponse as GeneratedGetPositionsResponse,
    schedule_futures_sweep_request::ScheduleFuturesSweepRequest as GeneratedScheduleFuturesSweepRequest,
    schedule_futures_sweep_response::ScheduleFuturesSweepResponse as GeneratedScheduleFuturesSweepResponse,
    set_auto_sweep_request::SetAutoSweepRequest as GeneratedSetAutoSweepRequest,
    set_auto_sweep_response::SetAutoSweepResponse as GeneratedSetAutoSweepResponse,
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
    pub inner: GeneratedGetFcmBalanceResponse,
}
impl From<GeneratedGetFcmBalanceResponse> for GetFcmBalanceResponse {
    fn from(inner: GeneratedGetFcmBalanceResponse) -> Self {
        Self { inner }
    }
}

#[derive(Debug, Clone)]
pub struct GetPositionsResponse {
    pub inner: GeneratedGetPositionsResponse,
}
impl From<GeneratedGetPositionsResponse> for GetPositionsResponse {
    fn from(inner: GeneratedGetPositionsResponse) -> Self {
        Self { inner }
    }
}
impl GetPositionsResponse {
    pub fn positions(&self) -> &[FcmPosition] {
        self.inner.positions.as_deref().unwrap_or(&[])
    }
}

#[derive(Debug, Clone)]
pub struct GetFuturesSweepsResponse {
    pub inner: GeneratedGetFuturesSweepsResponse,
}
impl From<GeneratedGetFuturesSweepsResponse> for GetFuturesSweepsResponse {
    fn from(inner: GeneratedGetFuturesSweepsResponse) -> Self {
        Self { inner }
    }
}
impl GetFuturesSweepsResponse {
    pub fn sweeps(&self) -> &[FcmFuturesSweep] {
        self.inner.sweeps.as_deref().unwrap_or(&[])
    }
    pub fn auto_sweep(&self) -> Option<bool> {
        self.inner.auto_sweep
    }
}

#[derive(Debug, Clone)]
pub struct SetAutoSweepResponse {
    pub inner: GeneratedSetAutoSweepResponse,
}
impl From<GeneratedSetAutoSweepResponse> for SetAutoSweepResponse {
    fn from(inner: GeneratedSetAutoSweepResponse) -> Self {
        Self { inner }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleFuturesSweepResponse {
    pub inner: GeneratedScheduleFuturesSweepResponse,
}
impl From<GeneratedScheduleFuturesSweepResponse> for ScheduleFuturesSweepResponse {
    fn from(inner: GeneratedScheduleFuturesSweepResponse) -> Self {
        Self { inner }
    }
}

#[derive(Debug, Clone)]
pub struct CancelFuturesSweepResponse {
    pub inner: crate::types::generated::generated::cancel_futures_sweep_response::CancelFuturesSweepResponse,
}
impl From<crate::types::generated::generated::cancel_futures_sweep_response::CancelFuturesSweepResponse> for CancelFuturesSweepResponse {
    fn from(inner: crate::types::generated::generated::cancel_futures_sweep_response::CancelFuturesSweepResponse) -> Self {
        Self { inner }
    }
}
