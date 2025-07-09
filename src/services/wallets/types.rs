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
use crate::types::generated::generated::paginated_response::PaginatedResponse;
use crate::types::generated::generated::{
    create_wallet_response::CreateWalletResponse as GeneratedCreateWalletResponse,
    get_wallet_deposit_instructions_response::GetWalletDepositInstructionsResponse as GeneratedGetWalletDepositInstructionsResponse,
    get_wallet_response::GetWalletResponse as GeneratedGetWalletResponse,
    get_wallets_response::GetWalletsResponse as GeneratedGetWalletsResponse, wallet::Wallet,
};
use uuid::Uuid;

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request to list wallets for a portfolio
#[derive(Debug, Clone)]
pub struct ListWalletsRequest {
    pub portfolio_id: String,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

impl ListWalletsRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            cursor: None,
            limit: None,
        }
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.cursor = Some(cursor_str);
        }
        self
    }
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request to get a wallet by ID
#[derive(Debug, Clone)]
pub struct GetWalletRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
}

impl GetWalletRequest {
    pub fn new(portfolio_id: impl Into<String>, wallet_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
        }
    }
}

/// Request to get deposit instructions for a wallet
#[derive(Debug, Clone)]
pub struct GetWalletDepositInstructionsRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub deposit_type: Option<crate::types::generated::generated::wallet_deposit_instruction_type::WalletDepositInstructionType>,
    pub network_id: Option<String>,
    pub network_type: Option<String>,
}

impl GetWalletDepositInstructionsRequest {
    pub fn new(portfolio_id: impl Into<String>, wallet_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            deposit_type: None,
            network_id: None,
            network_type: None,
        }
    }
    pub fn with_deposit_type(
        mut self,
        deposit_type: crate::types::generated::generated::wallet_deposit_instruction_type::WalletDepositInstructionType,
    ) -> Self {
        self.deposit_type = Some(deposit_type);
        self
    }
    pub fn with_network_id(mut self, network_id: impl Into<String>) -> Self {
        let id = network_id.into();
        if !id.is_empty() {
            self.network_id = Some(id);
        }
        self
    }
    pub fn with_network_type(mut self, network_type: impl Into<String>) -> Self {
        let t = network_type.into();
        if !t.is_empty() {
            self.network_type = Some(t);
        }
        self
    }
}

/// Request to create a wallet (builder pattern)
#[derive(Debug, Clone)]
pub struct CreateWalletRequest {
    pub name: String,
    pub symbol: String,
    pub wallet_type: Option<crate::types::generated::generated::wallet_type::WalletType>,
    pub idempotency_key: Option<String>,
    pub network_family: Option<crate::types::generated::generated::network_family::NetworkFamily>,
    pub network: Option<Box<crate::types::generated::generated::network::Network>>,
}

impl CreateWalletRequest {
    pub fn new(name: impl Into<String>, symbol: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            wallet_type: None,
            idempotency_key: Some(Uuid::new_v4().to_string()),
            network_family: None,
            network: None,
        }
    }
    pub fn with_wallet_type(
        mut self,
        wallet_type: crate::types::generated::generated::wallet_type::WalletType,
    ) -> Self {
        self.wallet_type = Some(wallet_type);
        self
    }
    pub fn with_idempotency_key(mut self, idempotency_key: impl Into<String>) -> Self {
        self.idempotency_key = Some(idempotency_key.into());
        self
    }
    pub fn with_network_family(
        mut self,
        network_family: crate::types::generated::generated::network_family::NetworkFamily,
    ) -> Self {
        self.network_family = Some(network_family);
        self
    }
    pub fn with_network(
        mut self,
        network: crate::types::generated::generated::network::Network,
    ) -> Self {
        self.network = Some(Box::new(network));
        self
    }
}

impl From<CreateWalletRequest>
    for crate::types::generated::generated::create_wallet_request::CreateWalletRequest
{
    fn from(req: CreateWalletRequest) -> Self {
        Self {
            name: req.name,
            symbol: req.symbol,
            wallet_type: req.wallet_type,
            idempotency_key: req.idempotency_key,
            network_family: req.network_family,
            network: req.network,
        }
    }
}

/// Request to list wallet addresses
#[derive(Debug, Clone)]
pub struct ListWalletAddressesRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub network_id: String,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

impl ListWalletAddressesRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        wallet_id: impl Into<String>,
        network_id: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            network_id: network_id.into(),
            cursor: None,
            limit: None,
        }
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.cursor = Some(cursor_str);
        }
        self
    }
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request to create a wallet deposit address
#[derive(Debug, Clone)]
pub struct CreateWalletDepositAddressRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub network_id: String,
}

impl CreateWalletDepositAddressRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        wallet_id: impl Into<String>,
        network_id: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            network_id: network_id.into(),
        }
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Response for listing wallets
#[derive(Debug, Clone)]
pub struct ListWalletsResponse {
    pub wallets: Vec<Wallet>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedGetWalletsResponse> for ListWalletsResponse {
    fn from(response: GeneratedGetWalletsResponse) -> Self {
        Self {
            wallets: response.wallets.unwrap_or_default(),
            pagination: response.pagination.map(|p| *p).unwrap_or_default(),
        }
    }
}

impl ListWalletsResponse {
    pub fn wallets(&self) -> &[Wallet] {
        &self.wallets
    }
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
    pub fn count(&self) -> usize {
        self.wallets.len()
    }
    pub fn has_more(&self) -> bool {
        self.pagination.has_next
    }
    pub fn next_cursor(&self) -> Option<&str> {
        if self.pagination.next_cursor.is_empty() {
            None
        } else {
            Some(self.pagination.next_cursor.as_str())
        }
    }
}

/// Response for getting a wallet
pub type GetWalletResponse = GeneratedGetWalletResponse;

/// Response for creating a wallet (wraps the generated type)
#[derive(Debug, Clone)]
pub struct CreateWalletResponse(pub GeneratedCreateWalletResponse);

impl From<GeneratedCreateWalletResponse> for CreateWalletResponse {
    fn from(inner: GeneratedCreateWalletResponse) -> Self {
        Self(inner)
    }
}

/// Response for getting wallet deposit instructions
pub type GetWalletDepositInstructionsResponse = GeneratedGetWalletDepositInstructionsResponse;

/// Response for listing wallet addresses
#[derive(Debug, Clone)]
pub struct ListWalletAddressesResponse {
    pub addresses: Vec<crate::types::generated::generated::blockchain_address::BlockchainAddress>,
    pub pagination: PaginatedResponse,
}

impl From<crate::types::generated::generated::list_wallet_addresses_response::ListWalletAddressesResponse> for ListWalletAddressesResponse {
    fn from(response: crate::types::generated::generated::list_wallet_addresses_response::ListWalletAddressesResponse) -> Self {
        Self {
            addresses: response.addresses,
            pagination: *response.pagination,
        }
    }
}

impl ListWalletAddressesResponse {
    pub fn addresses(
        &self,
    ) -> &[crate::types::generated::generated::blockchain_address::BlockchainAddress] {
        &self.addresses
    }
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
    pub fn count(&self) -> usize {
        self.addresses.len()
    }
    pub fn has_more(&self) -> bool {
        self.pagination.has_next
    }
    pub fn next_cursor(&self) -> Option<&str> {
        if self.pagination.next_cursor.is_empty() {
            None
        } else {
            Some(self.pagination.next_cursor.as_str())
        }
    }
}

// Response for creating a wallet deposit address
pub type CreateWalletDepositAddressResponse =
    crate::types::generated::generated::blockchain_address::BlockchainAddress;
