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
use crate::types::generated::generated::get_entity_assets_response::GetEntityAssetsResponse;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;

use super::types::{ListEntityAssetsRequest, ListEntityAssetsResponse};

/// Service for interacting with asset-related endpoints
pub struct AssetsService {
    client: Box<dyn HttpClient>,
}

impl AssetsService {
    /// Create a new AssetsService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List all assets available for a given entity
    pub async fn list_entity_assets(
        &self,
        request: ListEntityAssetsRequest,
    ) -> crate::error::HttpResult<ListEntityAssetsResponse> {
        let path = format!("entities/{}/assets", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GetEntityAssetsResponse = resp.json().await?;
        Ok(response.into())
    }
}
