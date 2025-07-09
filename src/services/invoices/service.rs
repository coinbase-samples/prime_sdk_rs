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
use crate::services::invoices::types::{ListInvoicesRequest, ListInvoicesResponse};
use crate::types::generated::generated::get_invoices_response::GetInvoicesResponse as GeneratedGetInvoicesResponse;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

/// Service for managing invoices
pub struct InvoiceService {
    client: Box<dyn HttpClient>,
}

impl InvoiceService {
    /// Create a new invoice service
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List invoices for an entity
    pub async fn list_invoices(
        &self,
        request: ListInvoicesRequest,
    ) -> crate::error::HttpResult<ListInvoicesResponse> {
        let path = format!("entities/{}/invoices", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Add query parameters
        let mut query_params = HashMap::new();

        if let Some(state) = request.state {
            query_params.insert("state".to_string(), state.to_string());
        }

        if let Some(year) = request.year {
            query_params.insert("year".to_string(), year.to_string());
        }

        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }

        if let Some(cursor) = request.cursor {
            query_params.insert("cursor".to_string(), cursor);
        }

        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        // Make the request
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetInvoicesResponse = resp.json().await?;

        Ok(response.into())
    }
}
