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
    blockchain_address::BlockchainAddress,
    create_a_transfer_between_two_wallets::CreateATransferBetweenTwoWallets as GeneratedCreateWalletTransferRequest,
    create_conversion_request::CreateConversionRequest as GeneratedCreateConversionRequest,
    create_conversion_response::CreateConversionResponse as GeneratedCreateConversionResponse,
    create_onchain_transaction_request::CreateOnchainTransactionRequest as GeneratedCreateOnchainTransactionRequest,
    create_onchain_transaction_request_period_evm_params::CreateOnchainTransactionRequestPeriodEvmParams,
    create_onchain_transaction_response::CreateOnchainTransactionResponse as GeneratedCreateOnchainTransactionResponse,
    create_wallet_transfer_response::CreateWalletTransferResponse as GeneratedCreateWalletTransferResponse,
    create_wallet_withdrawal_request::CreateWalletWithdrawalRequest as GeneratedCreateWalletWithdrawalRequest,
    create_wallet_withdrawal_response::CreateWalletWithdrawalResponse as GeneratedCreateWalletWithdrawalResponse,
    destination_type::DestinationType,
    get_portfolio_transactions_response::GetPortfolioTransactionsResponse as GeneratedGetPortfolioTransactionsResponse,
    get_transaction_response::GetTransactionResponse as GeneratedGetTransactionResponse,
    get_wallet_transactions_response::GetWalletTransactionsResponse as GeneratedGetWalletTransactionsResponse,
    paginated_response::PaginatedResponse, payment_method_destination::PaymentMethodDestination,
    rpc_config::RpcConfig, sort_direction::SortDirection, transaction::Transaction,
    transaction_type::TransactionType,
};

#[derive(Debug, Clone)]
pub struct ListPortfolioTransactionsRequest {
    pub portfolio_id: String,
    pub symbols: Option<Vec<String>>,
    pub types: Option<Vec<TransactionType>>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
}

impl ListPortfolioTransactionsRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            symbols: None,
            types: None,
            start_time: None,
            end_time: None,
            cursor: None,
            limit: None,
            sort_direction: None,
        }
    }
    pub fn with_symbols(mut self, symbols: Vec<String>) -> Self {
        self.symbols = Some(symbols);
        self
    }
    pub fn with_types(mut self, types: Vec<TransactionType>) -> Self {
        self.types = Some(types);
        self
    }
    pub fn with_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }
    pub fn with_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let c = cursor.into();
        if !c.is_empty() {
            self.cursor = Some(c);
        }
        self
    }
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetTransactionRequest {
    pub portfolio_id: String,
    pub transaction_id: String,
}

impl GetTransactionRequest {
    pub fn new(portfolio_id: impl Into<String>, transaction_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            transaction_id: transaction_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListWalletTransactionsRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub types: Option<Vec<TransactionType>>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
}

impl ListWalletTransactionsRequest {
    pub fn new(portfolio_id: impl Into<String>, wallet_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            types: None,
            start_time: None,
            end_time: None,
            cursor: None,
            limit: None,
            sort_direction: None,
        }
    }
    pub fn with_types(mut self, types: Vec<TransactionType>) -> Self {
        self.types = Some(types);
        self
    }
    pub fn with_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }
    pub fn with_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let c = cursor.into();
        if !c.is_empty() {
            self.cursor = Some(c);
        }
        self
    }
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self
    }
}

pub struct ListPortfolioTransactionsResponse {
    pub transactions: Vec<Transaction>,
    pub pagination: Option<PaginatedResponse>,
}

impl From<GeneratedGetPortfolioTransactionsResponse> for ListPortfolioTransactionsResponse {
    fn from(resp: GeneratedGetPortfolioTransactionsResponse) -> Self {
        Self {
            transactions: resp.transactions.unwrap_or_default(),
            pagination: resp.pagination.map(|b| *b),
        }
    }
}

pub struct ListWalletTransactionsResponse {
    pub transactions: Vec<Transaction>,
    pub pagination: Option<PaginatedResponse>,
}

impl From<GeneratedGetWalletTransactionsResponse> for ListWalletTransactionsResponse {
    fn from(resp: GeneratedGetWalletTransactionsResponse) -> Self {
        Self {
            transactions: resp.transactions.unwrap_or_default(),
            pagination: resp.pagination.map(|b| *b),
        }
    }
}

pub type GetTransactionResponse = GeneratedGetTransactionResponse;

#[derive(Debug, Clone)]
pub struct CreateOnchainTransactionRequest {
    pub raw_unsigned_txn: String,
    pub rpc: Option<Box<RpcConfig>>,
    pub evm_params: Option<Box<CreateOnchainTransactionRequestPeriodEvmParams>>,
}

impl CreateOnchainTransactionRequest {
    pub fn new(raw_unsigned_txn: impl Into<String>) -> Self {
        Self {
            raw_unsigned_txn: raw_unsigned_txn.into(),
            rpc: None,
            evm_params: None,
        }
    }
    pub fn with_rpc(mut self, rpc: RpcConfig) -> Self {
        self.rpc = Some(Box::new(rpc));
        self
    }
    pub fn with_evm_params(
        mut self,
        evm_params: CreateOnchainTransactionRequestPeriodEvmParams,
    ) -> Self {
        self.evm_params = Some(Box::new(evm_params));
        self
    }
}

impl From<CreateOnchainTransactionRequest> for GeneratedCreateOnchainTransactionRequest {
    fn from(req: CreateOnchainTransactionRequest) -> Self {
        Self {
            raw_unsigned_txn: req.raw_unsigned_txn,
            rpc: req.rpc,
            evm_params: req.evm_params,
        }
    }
}
pub type CreateOnchainTransactionResponse = GeneratedCreateOnchainTransactionResponse;

#[derive(Debug, Clone)]
pub struct CreateWalletTransferRequest {
    pub amount: String,
    pub destination: String,
    pub idempotency_key: String,
    pub currency_symbol: String,
}

impl CreateWalletTransferRequest {
    pub fn new(
        amount: impl Into<String>,
        destination: impl Into<String>,
        idempotency_key: impl Into<String>,
        currency_symbol: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            destination: destination.into(),
            idempotency_key: idempotency_key.into(),
            currency_symbol: currency_symbol.into(),
        }
    }
}
impl From<CreateWalletTransferRequest> for GeneratedCreateWalletTransferRequest {
    fn from(req: CreateWalletTransferRequest) -> Self {
        Self {
            amount: req.amount,
            destination: req.destination,
            idempotency_key: req.idempotency_key,
            currency_symbol: req.currency_symbol,
        }
    }
}
pub type CreateWalletTransferResponse = GeneratedCreateWalletTransferResponse;

#[derive(Debug, Clone)]
pub struct CreateWalletWithdrawalRequest {
    pub amount: String,
    pub destination_type: DestinationType,
    pub idempotency_key: String,
    pub currency_symbol: String,
    pub payment_method: Option<Box<PaymentMethodDestination>>,
    pub blockchain_address: Option<Box<BlockchainAddress>>,
}

impl CreateWalletWithdrawalRequest {
    pub fn new(
        amount: impl Into<String>,
        destination_type: DestinationType,
        idempotency_key: impl Into<String>,
        currency_symbol: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            destination_type,
            idempotency_key: idempotency_key.into(),
            currency_symbol: currency_symbol.into(),
            payment_method: None,
            blockchain_address: None,
        }
    }
    pub fn with_payment_method(mut self, payment_method: PaymentMethodDestination) -> Self {
        self.payment_method = Some(Box::new(payment_method));
        self
    }
    pub fn with_blockchain_address(mut self, blockchain_address: BlockchainAddress) -> Self {
        self.blockchain_address = Some(Box::new(blockchain_address));
        self
    }
}
impl From<CreateWalletWithdrawalRequest> for GeneratedCreateWalletWithdrawalRequest {
    fn from(req: CreateWalletWithdrawalRequest) -> Self {
        Self {
            amount: req.amount,
            destination_type: req.destination_type,
            idempotency_key: req.idempotency_key,
            currency_symbol: req.currency_symbol,
            payment_method: req.payment_method,
            blockchain_address: req.blockchain_address,
        }
    }
}
pub type CreateWalletWithdrawalResponse = GeneratedCreateWalletWithdrawalResponse;

#[derive(Debug, Clone)]
pub struct CreateConversionRequest {
    pub amount: String,
    pub destination: String,
    pub idempotency_key: String,
    pub source_symbol: String,
    pub destination_symbol: String,
}

impl CreateConversionRequest {
    pub fn new(
        amount: impl Into<String>,
        destination: impl Into<String>,
        idempotency_key: impl Into<String>,
        source_symbol: impl Into<String>,
        destination_symbol: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            destination: destination.into(),
            idempotency_key: idempotency_key.into(),
            source_symbol: source_symbol.into(),
            destination_symbol: destination_symbol.into(),
        }
    }
}
impl From<CreateConversionRequest> for GeneratedCreateConversionRequest {
    fn from(req: CreateConversionRequest) -> Self {
        Self {
            amount: req.amount,
            destination: req.destination,
            idempotency_key: req.idempotency_key,
            source_symbol: req.source_symbol,
            destination_symbol: req.destination_symbol,
        }
    }
}
pub type CreateConversionResponse = GeneratedCreateConversionResponse;
