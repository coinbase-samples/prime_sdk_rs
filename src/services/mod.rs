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
pub mod activities;
pub mod address_book;
pub mod allocations;
pub mod assets;
pub mod balances;
pub mod commission;
pub mod financing;
pub mod futures;
pub mod invoices;
pub mod onchain_address_groups;
pub mod orders;
pub mod payment_methods;
pub mod portfolios;
pub mod positions;
pub mod products;
pub mod transactions;
pub mod users;
pub mod wallets;

pub use activities::{
    ActivitiesService, ListEntityActivitiesRequest, ListEntityActivitiesResponse,
    ListPortfolioActivitiesRequest, ListPortfolioActivitiesResponse,
};

pub use address_book::{
    AddressBookService, CreateAddressBookEntryRequest, CreateAddressBookEntryResponse,
    GetAddressBookResponse, ListAddressBookRequest, ListAddressBookResponse,
};

pub use allocations::{
    AllocationService, CreateAllocationRequest, CreateAllocationResponse,
    CreateNetAllocationRequest, CreateNetAllocationResponse, GetAllocationRequest,
    GetAllocationResponse, ListAllocationsByClientNettingIdRequest,
    ListAllocationsByClientNettingIdResponse, ListPortfolioAllocationsRequest,
    ListPortfolioAllocationsResponse,
};

pub use assets::{AssetsService, ListEntityAssetsRequest, ListEntityAssetsResponse};

pub use balances::{
    BalancesService, GetWalletBalanceRequest, GetWalletBalanceResponse, ListEntityBalancesRequest,
    ListEntityBalancesResponse, ListPortfolioBalancesRequest, ListPortfolioBalancesResponse,
    ListWeb3WalletBalancesRequest, ListWeb3WalletBalancesResponse,
};

pub use commission::{
    CommissionService, GetPortfolioCommissionRequest, GetPortfolioCommissionResponse,
};

pub use financing::{
    CreateNewLocatesRequest, CreateNewLocatesResponse, FinancingService, GetBuyingPowerRequest,
    GetBuyingPowerResponse, GetMarginInformationRequest, GetMarginInformationResponse,
    GetPostTradeCreditRequest, GetPostTradeCreditResponse, GetWithdrawalPowerRequest,
    GetWithdrawalPowerResponse, ListExistingLocatesRequest, ListExistingLocatesResponse,
    ListInterestAccrualsRequest, ListInterestAccrualsResponse, ListLocateAvailabilitiesRequest,
    ListLocateAvailabilitiesResponse, ListMarginConversionsRequest, ListMarginConversionsResponse,
    ListMarginSummariesRequest, ListMarginSummariesResponse, ListPortfolioInterestAccrualsRequest,
    ListPortfolioInterestAccrualsResponse, ListTFTieredPricingFeesRequest,
    ListTFTieredPricingFeesResponse,
};

pub use futures::{
    FuturesService, GetFcmBalanceRequest, GetFcmBalanceResponse, GetFcmEquityRequest,
    GetFcmEquityResponse, GetFcmMarginCallDetailsRequest, GetFcmMarginCallDetailsResponse,
    GetFcmRiskLimitsRequest, GetFcmRiskLimitsResponse, GetFcmSettingsRequest,
    GetFcmSettingsResponse, GetFuturesSweepsRequest, GetFuturesSweepsResponse, GetPositionsRequest,
    GetPositionsResponse, SetFcmSettingsRequest, SetFcmSettingsResponse,
};

pub use invoices::{InvoiceService, ListInvoicesRequest, ListInvoicesResponse};

pub use onchain_address_groups::*;

pub use orders::{
    CreateOrderRequest, EditOrderRequest, EditOrderResponse, GetOrderEditHistoryRequest,
    GetOrderEditHistoryResponse, OrderSide, OrderType, OrdersService, TimeInForceType,
};

pub use payment_methods::{
    GetEntityPaymentMethodDetailsRequest, GetEntityPaymentMethodDetailsResponse,
    ListEntityPaymentMethodsRequest, ListEntityPaymentMethodsResponse, PaymentMethodsService,
};

pub use portfolios::{GetPortfolioResponse, ListPortfoliosResponse, Portfolio, PortfoliosService};

pub use positions::{
    ListAggregateEntityPositionsResponse, ListEntityPositionsResponse, PositionsService,
};

pub use products::{
    GetCandlesRequest, GetCandlesResponse, ListPortfolioProductsRequest,
    ListPortfolioProductsResponse, ProductsService,
};

pub use transactions::{
    GetTransactionRequest, GetTransactionResponse, ListPortfolioTransactionsRequest,
    ListPortfolioTransactionsResponse, ListWalletTransactionsRequest,
    ListWalletTransactionsResponse, TransactionsService,
};

pub use users::{
    ListEntityUsersRequest, ListEntityUsersResponse, ListPortfolioUsersRequest,
    ListPortfolioUsersResponse, UsersService,
};

pub use wallets::{
    CreateWalletRequest, CreateWalletResponse, GetWalletDepositInstructionsRequest,
    GetWalletDepositInstructionsResponse, GetWalletRequest, GetWalletResponse, ListWalletsRequest,
    ListWalletsResponse, WalletsService,
};

// Re-export SortDirection from generated types for convenience
pub use crate::types::generated::generated::sort_direction::SortDirection;
