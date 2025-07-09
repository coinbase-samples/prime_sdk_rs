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
    asset::Asset, get_entity_assets_response::GetEntityAssetsResponse,
};

/// Wrapper for the list entity assets response
pub struct ListEntityAssetsResponse {
    pub assets: Vec<Asset>,
}

impl From<GetEntityAssetsResponse> for ListEntityAssetsResponse {
    fn from(response: GetEntityAssetsResponse) -> Self {
        Self {
            assets: response.assets.unwrap_or_default(),
        }
    }
}

impl ListEntityAssetsResponse {
    /// Get all assets from the response
    pub fn assets(&self) -> &[Asset] {
        &self.assets
    }

    /// Get the count of assets
    pub fn count(&self) -> usize {
        self.assets.len()
    }

    /// Get an asset by symbol
    pub fn get_by_symbol(&self, symbol: &str) -> Option<&Asset> {
        self.assets
            .iter()
            .find(|asset| asset.symbol.as_ref().map_or(false, |s| s == symbol))
    }

    /// Get all assets that support trading
    pub fn trading_assets(&self) -> Vec<&Asset> {
        self.assets
            .iter()
            .filter(|asset| asset.trading_supported.unwrap_or(false))
            .collect()
    }

    /// Get all assets that don't support trading
    pub fn non_trading_assets(&self) -> Vec<&Asset> {
        self.assets
            .iter()
            .filter(|asset| !asset.trading_supported.unwrap_or(false))
            .collect()
    }
}

/// Request for listing entity assets
pub struct ListEntityAssetsRequest {
    pub entity_id: String,
}

impl ListEntityAssetsRequest {
    /// Create a new request to list assets for an entity
    pub fn new(entity_id: &str) -> Self {
        Self {
            entity_id: entity_id.to_string(),
        }
    }
}
