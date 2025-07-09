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
    get_entity_payment_method_details_response::GetEntityPaymentMethodDetailsResponse as GeneratedGetEntityPaymentMethodDetailsResponse,
    get_entity_payment_methods_response::GetEntityPaymentMethodsResponse as GeneratedGetEntityPaymentMethodsResponse,
    payment_method_details::PaymentMethodDetails, payment_method_summary::PaymentMethodSummary,
};

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request parameters for listing entity payment methods
#[derive(Debug, Clone)]
pub struct ListEntityPaymentMethodsRequest {
    pub entity_id: String,
}

impl ListEntityPaymentMethodsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
}

/// Request parameters for getting entity payment method details
#[derive(Debug, Clone)]
pub struct GetEntityPaymentMethodDetailsRequest {
    pub entity_id: String,
    pub payment_method_id: String,
}

impl GetEntityPaymentMethodDetailsRequest {
    pub fn new(entity_id: impl Into<String>, payment_method_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            payment_method_id: payment_method_id.into(),
        }
    }
}

// ============================================================================
// BASE STRUCTS
// ============================================================================

/// Base struct for entity payment methods list responses
#[derive(Debug, Clone)]
pub struct EntityPaymentMethodsListResponse {
    pub payment_methods: Vec<PaymentMethodSummary>,
}

impl EntityPaymentMethodsListResponse {
    pub fn new(payment_methods: Vec<PaymentMethodSummary>) -> Self {
        Self { payment_methods }
    }
}

/// Base struct for entity payment method details responses
#[derive(Debug, Clone)]
pub struct EntityPaymentMethodDetailsResponse {
    pub details: Option<PaymentMethodDetails>,
}

impl EntityPaymentMethodDetailsResponse {
    pub fn new(details: Option<PaymentMethodDetails>) -> Self {
        Self { details }
    }
}

// ============================================================================
// RESPONSE TYPE ALIASES
// ============================================================================

/// Wrapper for entity payment methods list response
pub type ListEntityPaymentMethodsResponse = EntityPaymentMethodsListResponse;

impl From<GeneratedGetEntityPaymentMethodsResponse> for EntityPaymentMethodsListResponse {
    fn from(response: GeneratedGetEntityPaymentMethodsResponse) -> Self {
        Self {
            payment_methods: response.payment_methods.unwrap_or_default(),
        }
    }
}

/// Wrapper for entity payment method details response
pub type GetEntityPaymentMethodDetailsResponse = EntityPaymentMethodDetailsResponse;

impl From<GeneratedGetEntityPaymentMethodDetailsResponse> for EntityPaymentMethodDetailsResponse {
    fn from(response: GeneratedGetEntityPaymentMethodDetailsResponse) -> Self {
        Self {
            details: response.details.map(|d| *d),
        }
    }
}

// ============================================================================
// RESPONSE IMPLEMENTATIONS
// ============================================================================

impl ListEntityPaymentMethodsResponse {
    /// Get all payment methods
    pub fn payment_methods(&self) -> &[PaymentMethodSummary] {
        &self.payment_methods
    }

    /// Get payment methods by type
    pub fn get_by_type(&self, payment_method_type: &str) -> Vec<&PaymentMethodSummary> {
        self.payment_methods
            .iter()
            .filter(|pm| {
                pm.payment_method_type
                    .as_ref()
                    .map(|t| t.to_string() == payment_method_type)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get payment methods by symbol
    pub fn get_by_symbol(&self, symbol: &str) -> Vec<&PaymentMethodSummary> {
        self.payment_methods
            .iter()
            .filter(|pm| pm.symbol.as_ref().map(|s| s == symbol).unwrap_or(false))
            .collect()
    }
}

impl GetEntityPaymentMethodDetailsResponse {
    /// Get the payment method details
    pub fn details(&self) -> Option<&PaymentMethodDetails> {
        self.details.as_ref()
    }
}
