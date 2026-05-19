# Changelog

## [0.2.0] - 2026-MAY-19

### Added

#### 🆕 New API Endpoints

**Allocations Service**
- **`create_allocation()`**: Create a portfolio allocation from one or more orders (`POST /allocations`)
- **`create_net_allocation()`**: Create a net allocation from client netting IDs (`POST /allocations/net`)

**Orders Service**
- **`edit_order()`**: Edit an existing open order (limit price, size, quote value, etc.) (`POST /portfolios/{portfolioId}/orders/{orderId}/edit`)
- **`get_order_edit_history()`**: List edit history for an order (`GET /portfolios/{portfolioId}/orders/{orderId}/edit_history`)

**Products Service**
- **`get_candles()`**: Get historical candle data for a product with optional time range (`GET /portfolios/{portfolioId}/products/{productId}/candles`)

**Futures Service**
- **`get_fcm_equity()`**: Get FCM equity data for an entity (`GET /entities/{entityId}/futures/equity`)
- **`get_fcm_margin_call_details()`**: Get FCM margin call details for an entity (`GET /entities/{entityId}/futures/margin_call_details`)
- **`get_fcm_risk_limits()`**: Get FCM risk limits for an entity (`GET /entities/{entityId}/futures/risk_limits`)
- **`get_fcm_settings()`**: Get FCM settings for an entity (`GET /entities/{entityId}/futures/settings`)
- **`set_fcm_settings()`**: Update FCM settings for an entity (`POST /entities/{entityId}/futures/settings`)

**Financing Service**
- **`get_cross_margin_risk_parameters()`**: Get Cross Margin (XM) risk parameters for an entity (`GET /entities/{entityId}/cross_margin/risk_parameters`)
- **`get_cross_margin_prime_overview()`**: Get live cross-margin margin information for an entity (`GET /v2/entities/{entityId}/cross_margin/prime`)
- **`list_financing_eligible_assets()`**: List assets eligible for Trade Finance (`GET /financing/eligible-assets`)
- **`list_tf_obligations()`**: List Trade Finance obligations for an entity (`GET /entities/{entityId}/tf_obligations`)
- **`get_market_data()`**: Retrieve market data (volatility, ADV) for an entity with cursor pagination (`GET /entities/{entityId}/market_data`)
- **`set_funding_settings()`**: Set FCM funding configuration for an entity (`POST /entities/{entityId}/funding_settings`)

**Transactions Service**
- **`list_advanced_transfer_transactions()`**: List transactions for an advanced transfer (`GET /portfolios/{portfolioId}/advanced_transfers/{advancedTransferId}/transactions`)
- **`create_onchain_transaction()`**: Create an onchain transaction for a wallet (`POST /portfolios/{portfolioId}/wallets/{walletId}/onchain_transactions`)

**Travel Rule Service** *(new service)*
- **`get_transaction_travel_rule_data()`**: Retrieve travel rule data for a transaction (`GET /portfolios/{portfolioId}/transactions/{transactionId}/travel_rule`)
- **`submit_deposit_travel_rule_data()`**: Submit travel rule data for a deposit transaction (`POST /portfolios/{portfolioId}/transactions/{transactionId}/travel_rule`)

**Staking Service** *(new service)*
- **`portfolio_staking_initiate()`**: Initiate portfolio-level staking (`POST /portfolios/{portfolioId}/staking/initiate`)
- **`portfolio_staking_unstake()`**: Initiate portfolio-level unstaking (`POST /portfolios/{portfolioId}/staking/unstake`)
- **`list_transaction_validators()`**: Query validators associated with wallet-level stake transactions (`POST /portfolios/{portfolioId}/staking/transaction-validators/query`)
- **`wallet_staking_initiate()`**: Initiate wallet-level staking (`POST /portfolios/{portfolioId}/wallets/{walletId}/staking/initiate`)
- **`staking_claim_rewards()`**: Claim staking rewards for a wallet (`POST /portfolios/{portfolioId}/wallets/{walletId}/staking/claim_rewards`)
- **`get_staking_status()`**: Get staking status for a wallet (`GET /portfolios/{portfolioId}/wallets/{walletId}/staking/status`)
- **`wallet_staking_unstake()`**: Initiate wallet-level unstaking (`POST /portfolios/{portfolioId}/wallets/{walletId}/staking/unstake`)
- **`preview_unstake()`**: Preview an unstaking operation (`POST /portfolios/{portfolioId}/wallets/{walletId}/staking/preview_unstake`)
- **`get_unstaking_status()`**: Get unstaking status for a wallet (`GET /portfolios/{portfolioId}/wallets/{walletId}/staking/unstaking_status`)

**Advanced Transfers Service** *(new service)*
- **`list_advanced_transfers()`**: List advanced transfers for a portfolio with optional filtering and cursor pagination (`GET /portfolios/{portfolioId}/advanced_transfers`)
- **`create_advanced_transfer()`**: Create a new advanced transfer (`POST /portfolios/{portfolioId}/advanced_transfers`)
- **`cancel_advanced_transfer()`**: Cancel an advanced transfer by ID (`POST /portfolios/{portfolioId}/advanced_transfers/{advancedTransferId}/cancel`)
- **`get_portfolio_counterparty_id()`**: Get the counterparty ID for a portfolio (`GET /portfolios/{portfolioId}/counterparty_id`)

#### 📝 New & Updated Models

**New Models**
- **`CreateAllocationRequest`** / **`CreateAllocationResponse`**: Request/response for creating allocations
- **`CreateNetAllocationRequest`** / **`CreateNetAllocationResponse`**: Request/response for net allocations
- **`EditOrderRequest`** / **`EditOrderResponse`**: Request/response for editing orders
- **`GetOrderEditHistoryResponse`**: Order edit history entries
- **`GetCandlesRequest`** / **`GetCandlesResponse`**: Historical candle data for a product
- **`GetFcmEquityResponse`**, **`GetFcmMarginCallDetailsResponse`**, **`GetFcmRiskLimitsResponse`**, **`GetFcmSettingsResponse`**, **`SetFcmSettingsResponse`**: FCM entity endpoints
- **`GetCrossMarginRiskParametersResponse`**, **`GetCrossMarginPrimeOverviewResponse`**, **`ListFinancingEligibleAssetsResponse`**, **`ListTfObligationsResponse`**, **`GetMarketDataResponse`**, **`SetFundingSettingsResponse`**: Financing and cross-margin types
- **`ListAdvancedTransferTransactionsResponse`**, **`CreateOnchainTransactionRequest`** / **`CreateOnchainTransactionResponse`**: Transaction types for advanced transfers and onchain flows
- **`GetTransactionTravelRuleDataResponse`**, **`SubmitDepositTravelRuleDataRequest`**: Travel rule compliance types
- **`AdvancedTransfer`**, **`CreateAdvancedTransferRequest`** / **`CreateAdvancedTransferResponse`**, **`CancelAdvancedTransferResponse`**, **`ListAdvancedTransfersResponse`**, **`GetPortfolioCounterpartyIdResponse`**: Advanced transfer types
- Staking request/response types for portfolio and wallet initiate, unstake, preview, status, validators, and claim rewards

**Updated Models**
- Regenerated types from the latest OpenAPI spec, including cross-margin prime overview breakdowns, market data, FCM settings, and staking validator allocation fields

#### 🔢 New Enums

- Cross-margin and prime XM enums (e.g. **`PrimeXmControlStatus`**, **`PrimeXmMarginLevel`**, **`XmLiquidationStatus`**) from the latest spec
- **`AdvancedTransferState`**, **`AdvancedTransferType`**
- FCM and product enums (e.g. **`FcmMarginHealthState`**, **`FcmTradingSessionState`**, **`ProductType`**)
- Staking and travel rule related enums from the updated spec

#### 🔧 Tooling

- **`scripts/check_endpoint_coverage.py`**: Tag-aware script that compares the OpenAPI spec to implemented service methods (`make check-endpoints`)
- **`build.rs`**: Strips `Beta` / `PrimeBeta` prefixes from generated model and enum names for consistent naming with stable API types
- **`SDK_VERSION`** and **`USER_AGENT`** constants derived from `CARGO_PKG_VERSION` so the client `User-Agent` header stays in sync with the crate version

#### 📚 Examples

- Added examples for all new endpoints under `examples/<service>/`, runnable via `./run_example.sh`

### Changed

- Bumped crate version to **0.2.0**
- Full Prime API REST endpoint coverage (103/103 endpoints)

## [0.1.0] - 2024-DEC-12

### Added

- Support for Coinbase Prime API REST endpoints across activities, address book, allocations (list/get), assets, balances, commission, financing (core endpoints), futures (balance, positions, sweeps), invoices, onchain address groups, orders, payment methods, portfolios, positions, products (list), transactions (list/get/transfer/withdrawal/conversion), users, and wallets
- Async `PrimeClient` with credential-based authentication via `core_rs`
- OpenAPI-generated request and response types
- Example programs for common API operations
