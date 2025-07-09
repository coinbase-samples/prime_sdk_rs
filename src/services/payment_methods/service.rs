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
use crate::client::PrimeClient;
use crate::types::generated::generated::{
    get_entity_payment_method_details_response::GetEntityPaymentMethodDetailsResponse as GeneratedGetEntityPaymentMethodDetailsResponse,
    get_entity_payment_methods_response::GetEntityPaymentMethodsResponse as GeneratedGetEntityPaymentMethodsResponse,
};
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;

use super::types::{
    GetEntityPaymentMethodDetailsRequest, GetEntityPaymentMethodDetailsResponse,
    ListEntityPaymentMethodsRequest, ListEntityPaymentMethodsResponse,
};

/// Service for interacting with payment method-related endpoints
pub struct PaymentMethodsService {
    client: Box<dyn HttpClient>,
}

impl PaymentMethodsService {
    /// Create a new PaymentMethodsService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List all payment methods for a given entity
    pub async fn list_entity_payment_methods(
        &self,
        request: ListEntityPaymentMethodsRequest,
    ) -> crate::error::HttpResult<ListEntityPaymentMethodsResponse> {
        let path = format!("entities/{}/payment-methods", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetEntityPaymentMethodsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get payment method details by ID for a given entity
    pub async fn get_entity_payment_method_details(
        &self,
        request: GetEntityPaymentMethodDetailsRequest,
    ) -> crate::error::HttpResult<GetEntityPaymentMethodDetailsResponse> {
        let path = format!(
            "entities/{}/payment-methods/{}",
            request.entity_id, request.payment_method_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetEntityPaymentMethodDetailsResponse = resp.json().await?;
        Ok(response.into())
    }
}
