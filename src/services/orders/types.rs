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
    accept_quote_request::AcceptQuoteRequest as GeneratedAcceptQuoteRequest,
    accept_quote_response::AcceptQuoteResponse as GeneratedAcceptQuoteResponse,
    cancel_order_response::CancelOrderResponse as GeneratedCancelOrderResponse,
    create_order_request::CreateOrderRequest as GeneratedCreateOrderRequest,
    create_order_response::CreateOrderResponse as GeneratedCreateOrderResponse, fill::Fill,
    get_open_orders_response::GetOpenOrdersResponse as GeneratedGetOpenOrdersResponse,
    get_order_fills_response::GetOrderFillsResponse as GeneratedGetOrderFillsResponse,
    get_order_response::GetOrderResponse as GeneratedGetOrderResponse,
    get_orders_response::GetOrdersResponse as GeneratedGetOrdersResponse,
    get_portfolio_fills_response::GetPortfolioFillsResponse as GeneratedGetPortfolioFillsResponse,
    order::Order, order_preview_request::OrderPreviewRequest as GeneratedOrderPreviewRequest,
    order_side::OrderSide, order_status::OrderStatus, order_type::OrderType,
    paginated_response::PaginatedResponse,
    post_order_preview_response::PostOrderPreviewResponse as GeneratedPostOrderPreviewResponse,
    quote_response::QuoteResponse as GeneratedQuoteResponse, rfq::Rfq as GeneratedRfq,
    sort_direction::SortDirection, time_in_force_type::TimeInForceType,
};
use crate::utils::paginated_list::PaginatedList;

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct ListOrdersRequest {
    pub portfolio_id: String,
    pub order_statuses: Option<Vec<OrderStatus>>,
    pub product_ids: Option<Vec<String>>,
    pub order_type: Option<OrderType>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
    pub order_side: Option<OrderSide>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
impl ListOrdersRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            ..Default::default()
        }
    }
    pub fn with_order_statuses(mut self, statuses: Vec<OrderStatus>) -> Self {
        self.order_statuses = Some(statuses);
        self
    }
    pub fn with_product_ids(mut self, ids: Vec<String>) -> Self {
        self.product_ids = Some(ids);
        self
    }
    pub fn with_order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
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
    pub fn with_order_side(mut self, order_side: OrderSide) -> Self {
        self.order_side = Some(order_side);
        self
    }
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetOrderRequest {
    pub portfolio_id: String,
    pub order_id: String,
}
impl GetOrderRequest {
    pub fn new(portfolio_id: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            order_id: order_id.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListOpenOrdersRequest {
    pub portfolio_id: String,
    pub product_ids: Option<Vec<String>>,
    pub order_type: Option<OrderType>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
    pub start_date: Option<String>,
    pub order_side: Option<OrderSide>,
    pub end_date: Option<String>,
}
impl ListOpenOrdersRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            ..Default::default()
        }
    }
    pub fn with_product_ids(mut self, ids: Vec<String>) -> Self {
        self.product_ids = Some(ids);
        self
    }
    pub fn with_order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
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
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    pub fn with_order_side(mut self, order_side: OrderSide) -> Self {
        self.order_side = Some(order_side);
        self
    }
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListPortfolioFillsRequest {
    pub portfolio_id: String,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub sort_direction: Option<SortDirection>,
}
impl ListPortfolioFillsRequest {
    pub fn new(portfolio_id: impl Into<String>, start_date: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            start_date: start_date.into(),
            ..Default::default()
        }
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListOrderFillsRequest {
    pub portfolio_id: String,
    pub order_id: String,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
}
impl ListOrderFillsRequest {
    pub fn new(portfolio_id: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            order_id: order_id.into(),
            ..Default::default()
        }
    }
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
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
pub struct CancelOrderRequest {
    pub portfolio_id: String,
    pub order_id: String,
}
impl CancelOrderRequest {
    pub fn new(portfolio_id: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            order_id: order_id.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CreateOrderRequest {
    pub portfolio_id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub client_order_id: String,
    pub order_type: OrderType,
    pub base_quantity: Option<String>,
    pub quote_value: Option<String>,
    pub limit_price: Option<String>,
    pub start_time: Option<String>,
    pub expiry_time: Option<String>,
    pub time_in_force: Option<TimeInForceType>,
    pub stp_id: Option<String>,
    pub display_quote_size: Option<String>,
    pub display_base_size: Option<String>,
    pub is_raise_exact: Option<bool>,
    pub historical_pov: Option<String>,
    pub stop_price: Option<String>,
    pub settl_currency: Option<String>,
}

impl CreateOrderRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        product_id: impl Into<String>,
        side: OrderSide,
        client_order_id: impl Into<String>,
        order_type: OrderType,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            product_id: product_id.into(),
            side,
            client_order_id: client_order_id.into(),
            order_type,
            ..Default::default()
        }
    }
    pub fn with_base_quantity(mut self, base_quantity: impl Into<String>) -> Self {
        self.base_quantity = Some(base_quantity.into());
        self
    }
    pub fn with_quote_value(mut self, quote_value: impl Into<String>) -> Self {
        self.quote_value = Some(quote_value.into());
        self
    }
    pub fn with_limit_price(mut self, limit_price: impl Into<String>) -> Self {
        self.limit_price = Some(limit_price.into());
        self
    }
    pub fn with_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }
    pub fn with_expiry_time(mut self, expiry_time: impl Into<String>) -> Self {
        self.expiry_time = Some(expiry_time.into());
        self
    }
    pub fn with_time_in_force(mut self, tif: TimeInForceType) -> Self {
        self.time_in_force = Some(tif);
        self
    }
    pub fn with_stp_id(mut self, stp_id: impl Into<String>) -> Self {
        self.stp_id = Some(stp_id.into());
        self
    }
    pub fn with_display_quote_size(mut self, display_quote_size: impl Into<String>) -> Self {
        self.display_quote_size = Some(display_quote_size.into());
        self
    }
    pub fn with_display_base_size(mut self, display_base_size: impl Into<String>) -> Self {
        self.display_base_size = Some(display_base_size.into());
        self
    }
    pub fn with_is_raise_exact(mut self, is_raise_exact: bool) -> Self {
        self.is_raise_exact = Some(is_raise_exact);
        self
    }
    pub fn with_historical_pov(mut self, historical_pov: impl Into<String>) -> Self {
        self.historical_pov = Some(historical_pov.into());
        self
    }
    pub fn with_stop_price(mut self, stop_price: impl Into<String>) -> Self {
        self.stop_price = Some(stop_price.into());
        self
    }
    pub fn with_settl_currency(mut self, settl_currency: impl Into<String>) -> Self {
        self.settl_currency = Some(settl_currency.into());
        self
    }
}

impl From<&CreateOrderRequest> for GeneratedCreateOrderRequest {
    fn from(req: &CreateOrderRequest) -> Self {
        GeneratedCreateOrderRequest {
            product_id: req.product_id.clone(),
            side: req.side.clone(),
            client_order_id: req.client_order_id.clone(),
            r#type: req.order_type.clone(),
            base_quantity: req.base_quantity.clone(),
            quote_value: req.quote_value.clone(),
            limit_price: req.limit_price.clone(),
            start_time: req.start_time.clone(),
            expiry_time: req.expiry_time.clone(),
            time_in_force: req.time_in_force.clone(),
            stp_id: req.stp_id.clone(),
            display_quote_size: req.display_quote_size.clone(),
            display_base_size: req.display_base_size.clone(),
            is_raise_exact: req.is_raise_exact,
            historical_pov: req.historical_pov.clone(),
            stop_price: req.stop_price.clone(),
            settl_currency: req.settl_currency.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OrderPreviewRequest {
    pub portfolio_id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub base_quantity: Option<String>,
    pub quote_value: Option<String>,
    pub limit_price: Option<String>,
    pub start_time: Option<String>,
    pub expiry_time: Option<String>,
    pub time_in_force: Option<TimeInForceType>,
    pub is_raise_exact: Option<bool>,
    pub historical_pov: Option<String>,
    pub stop_price: Option<String>,
    pub settl_currency: Option<String>,
}
impl OrderPreviewRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        product_id: impl Into<String>,
        side: OrderSide,
        order_type: OrderType,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            product_id: product_id.into(),
            side,
            order_type,
            ..Default::default()
        }
    }
    pub fn with_base_quantity(mut self, base_quantity: impl Into<String>) -> Self {
        self.base_quantity = Some(base_quantity.into());
        self
    }
    pub fn with_quote_value(mut self, quote_value: impl Into<String>) -> Self {
        self.quote_value = Some(quote_value.into());
        self
    }
    pub fn with_limit_price(mut self, limit_price: impl Into<String>) -> Self {
        self.limit_price = Some(limit_price.into());
        self
    }
    pub fn with_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }
    pub fn with_expiry_time(mut self, expiry_time: impl Into<String>) -> Self {
        self.expiry_time = Some(expiry_time.into());
        self
    }
    pub fn with_time_in_force(mut self, tif: TimeInForceType) -> Self {
        self.time_in_force = Some(tif);
        self
    }
    pub fn with_is_raise_exact(mut self, is_raise_exact: bool) -> Self {
        self.is_raise_exact = Some(is_raise_exact);
        self
    }
    pub fn with_historical_pov(mut self, historical_pov: impl Into<String>) -> Self {
        self.historical_pov = Some(historical_pov.into());
        self
    }
    pub fn with_stop_price(mut self, stop_price: impl Into<String>) -> Self {
        self.stop_price = Some(stop_price.into());
        self
    }
    pub fn with_settl_currency(mut self, settl_currency: impl Into<String>) -> Self {
        self.settl_currency = Some(settl_currency.into());
        self
    }
}
impl From<&OrderPreviewRequest> for GeneratedOrderPreviewRequest {
    fn from(req: &OrderPreviewRequest) -> Self {
        GeneratedOrderPreviewRequest {
            product_id: req.product_id.clone(),
            side: req.side.clone(),
            r#type: req.order_type.clone(),
            base_quantity: req.base_quantity.clone(),
            quote_value: req.quote_value.clone(),
            limit_price: req.limit_price.clone(),
            start_time: req.start_time.clone(),
            expiry_time: req.expiry_time.clone(),
            time_in_force: req.time_in_force.clone(),
            is_raise_exact: req.is_raise_exact,
            historical_pov: req.historical_pov.clone(),
            stop_price: req.stop_price.clone(),
            settl_currency: req.settl_currency.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RfqRequest {
    pub portfolio_id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub client_quote_id: String,
    pub base_quantity: Option<String>,
    pub quote_value: Option<String>,
    pub limit_price: String,
    pub settl_currency: Option<String>,
}
impl RfqRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        product_id: impl Into<String>,
        side: OrderSide,
        client_quote_id: impl Into<String>,
        limit_price: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            product_id: product_id.into(),
            side,
            client_quote_id: client_quote_id.into(),
            limit_price: limit_price.into(),
            ..Default::default()
        }
    }
    pub fn with_base_quantity(mut self, base_quantity: impl Into<String>) -> Self {
        self.base_quantity = Some(base_quantity.into());
        self
    }
    pub fn with_quote_value(mut self, quote_value: impl Into<String>) -> Self {
        self.quote_value = Some(quote_value.into());
        self
    }
    pub fn with_settl_currency(mut self, settl_currency: impl Into<String>) -> Self {
        self.settl_currency = Some(settl_currency.into());
        self
    }
}
impl From<&RfqRequest> for GeneratedRfq {
    fn from(req: &RfqRequest) -> Self {
        GeneratedRfq {
            product_id: req.product_id.clone(),
            side: req.side.clone(),
            client_quote_id: req.client_quote_id.clone(),
            base_quantity: req.base_quantity.clone(),
            quote_value: req.quote_value.clone(),
            limit_price: req.limit_price.clone(),
            settl_currency: req.settl_currency.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AcceptQuoteRequest {
    pub portfolio_id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub client_order_id: String,
    pub quote_id: String,
    pub settl_currency: Option<String>,
}
impl AcceptQuoteRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        product_id: impl Into<String>,
        side: OrderSide,
        client_order_id: impl Into<String>,
        quote_id: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            product_id: product_id.into(),
            side,
            client_order_id: client_order_id.into(),
            quote_id: quote_id.into(),
            ..Default::default()
        }
    }
    pub fn with_settl_currency(mut self, settl_currency: impl Into<String>) -> Self {
        self.settl_currency = Some(settl_currency.into());
        self
    }
}
impl From<&AcceptQuoteRequest> for GeneratedAcceptQuoteRequest {
    fn from(req: &AcceptQuoteRequest) -> Self {
        GeneratedAcceptQuoteRequest {
            product_id: req.product_id.clone(),
            side: req.side.clone(),
            client_order_id: req.client_order_id.clone(),
            quote_id: req.quote_id.clone(),
            settl_currency: req.settl_currency.clone(),
        }
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct ListOrdersResponse {
    pub orders: Vec<Order>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedGetOrdersResponse> for ListOrdersResponse {
    fn from(resp: GeneratedGetOrdersResponse) -> Self {
        Self {
            orders: resp.orders.unwrap_or_default(),
            pagination: *resp.pagination.unwrap_or_default(),
        }
    }
}

impl PaginatedList<Order> for ListOrdersResponse {
    fn items(&self) -> &[Order] {
        &self.orders
    }
    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

#[derive(Debug, Clone)]
pub struct GetOrderResponse {
    pub order: Option<Order>,
}

impl From<GeneratedGetOrderResponse> for GetOrderResponse {
    fn from(resp: GeneratedGetOrderResponse) -> Self {
        Self {
            order: resp.order.map(|b| *b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListOpenOrdersResponse {
    pub orders: Vec<Order>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedGetOpenOrdersResponse> for ListOpenOrdersResponse {
    fn from(resp: GeneratedGetOpenOrdersResponse) -> Self {
        Self {
            orders: resp.orders.unwrap_or_default(),
            pagination: *resp.pagination.unwrap_or_default(),
        }
    }
}

impl PaginatedList<Order> for ListOpenOrdersResponse {
    fn items(&self) -> &[Order] {
        &self.orders
    }
    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

#[derive(Debug, Clone)]
pub struct ListPortfolioFillsResponse {
    pub fills: Vec<Fill>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedGetPortfolioFillsResponse> for ListPortfolioFillsResponse {
    fn from(resp: GeneratedGetPortfolioFillsResponse) -> Self {
        Self {
            fills: resp.fills.unwrap_or_default(),
            pagination: *resp.pagination.unwrap_or_default(),
        }
    }
}

impl PaginatedList<Fill> for ListPortfolioFillsResponse {
    fn items(&self) -> &[Fill] {
        &self.fills
    }
    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

#[derive(Debug, Clone)]
pub struct ListOrderFillsResponse {
    pub fills: Vec<Fill>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedGetOrderFillsResponse> for ListOrderFillsResponse {
    fn from(resp: GeneratedGetOrderFillsResponse) -> Self {
        Self {
            fills: resp.fills.unwrap_or_default(),
            pagination: *resp.pagination.unwrap_or_default(),
        }
    }
}

impl PaginatedList<Fill> for ListOrderFillsResponse {
    fn items(&self) -> &[Fill] {
        &self.fills
    }
    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

#[derive(Debug, Clone)]
pub struct CancelOrderResponse {
    pub id: Option<String>,
}

impl From<GeneratedCancelOrderResponse> for CancelOrderResponse {
    fn from(resp: GeneratedCancelOrderResponse) -> Self {
        Self { id: resp.id }
    }
}

#[derive(Debug, Clone)]
pub struct CreateOrderResponse {
    pub order_id: Option<String>,
}
impl From<GeneratedCreateOrderResponse> for CreateOrderResponse {
    fn from(resp: GeneratedCreateOrderResponse) -> Self {
        Self {
            order_id: resp.order_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderPreviewResponse {
    pub inner: GeneratedPostOrderPreviewResponse,
}
impl From<GeneratedPostOrderPreviewResponse> for OrderPreviewResponse {
    fn from(resp: GeneratedPostOrderPreviewResponse) -> Self {
        Self { inner: resp }
    }
}

#[derive(Debug, Clone)]
pub struct RfqResponse {
    pub inner: GeneratedQuoteResponse,
}
impl From<GeneratedQuoteResponse> for RfqResponse {
    fn from(resp: GeneratedQuoteResponse) -> Self {
        Self { inner: resp }
    }
}

#[derive(Debug, Clone)]
pub struct AcceptQuoteResponse {
    pub order_id: Option<String>,
}
impl From<GeneratedAcceptQuoteResponse> for AcceptQuoteResponse {
    fn from(resp: GeneratedAcceptQuoteResponse) -> Self {
        Self {
            order_id: resp.order_id,
        }
    }
}
