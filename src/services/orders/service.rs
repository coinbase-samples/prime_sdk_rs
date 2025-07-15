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
use crate::error::HttpResult;
use crate::types::generated::generated::{
    accept_quote_request::AcceptQuoteRequest as GeneratedAcceptQuoteRequest,
    accept_quote_response::AcceptQuoteResponse as GeneratedAcceptQuoteResponse,
    cancel_order_response::CancelOrderResponse as GeneratedCancelOrderResponse,
    create_order_request::CreateOrderRequest as GeneratedCreateOrderRequest,
    create_order_response::CreateOrderResponse as GeneratedCreateOrderResponse,
    get_open_orders_response::GetOpenOrdersResponse as GeneratedGetOpenOrdersResponse,
    get_order_fills_response::GetOrderFillsResponse as GeneratedGetOrderFillsResponse,
    get_order_response::GetOrderResponse as GeneratedGetOrderResponse,
    get_orders_response::GetOrdersResponse as GeneratedGetOrdersResponse,
    get_portfolio_fills_response::GetPortfolioFillsResponse as GeneratedGetPortfolioFillsResponse,
    order_preview_request::OrderPreviewRequest as GeneratedOrderPreviewRequest,
    post_order_preview_response::PostOrderPreviewResponse as GeneratedPostOrderPreviewResponse,
    quote_response::QuoteResponse as GeneratedQuoteResponse, rfq::Rfq as GeneratedRfq,
};
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::*;

pub struct OrdersService {
    client: Box<dyn HttpClient>,
}

impl OrdersService {
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    pub async fn list_orders(&self, request: ListOrdersRequest) -> HttpResult<ListOrdersResponse> {
        let path = format!("portfolios/{}/orders", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(statuses) = &request.order_statuses {
            for status in statuses {
                query_params.insert("order_statuses".to_string(), status.to_string());
            }
        }
        if let Some(product_ids) = &request.product_ids {
            for id in product_ids {
                query_params.insert("product_ids".to_string(), id.clone());
            }
        }
        if let Some(order_type) = &request.order_type {
            query_params.insert("order_type".to_string(), order_type.to_string());
        }
        if let Some(cursor) = &request.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(sort_direction) = &request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }
        if let Some(order_side) = &request.order_side {
            query_params.insert("order_side".to_string(), order_side.to_string());
        }
        if let Some(start_date) = &request.start_date {
            query_params.insert("start_date".to_string(), start_date.clone());
        }
        if let Some(end_date) = &request.end_date {
            query_params.insert("end_date".to_string(), end_date.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetOrdersResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn get_order(&self, request: GetOrderRequest) -> HttpResult<GetOrderResponse> {
        let path = format!(
            "portfolios/{}/orders/{}",
            request.portfolio_id, request.order_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetOrderResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn list_open_orders(
        &self,
        request: ListOpenOrdersRequest,
    ) -> HttpResult<ListOpenOrdersResponse> {
        let path = format!("portfolios/{}/open_orders", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(product_ids) = &request.product_ids {
            for id in product_ids {
                query_params.insert("product_ids".to_string(), id.clone());
            }
        }
        if let Some(order_type) = &request.order_type {
            query_params.insert("order_type".to_string(), order_type.to_string());
        }
        if let Some(cursor) = &request.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(sort_direction) = &request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }
        if let Some(start_date) = &request.start_date {
            query_params.insert("start_date".to_string(), start_date.clone());
        }
        if let Some(order_side) = &request.order_side {
            query_params.insert("order_side".to_string(), order_side.to_string());
        }
        if let Some(end_date) = &request.end_date {
            query_params.insert("end_date".to_string(), end_date.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetOpenOrdersResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn list_portfolio_fills(
        &self,
        request: ListPortfolioFillsRequest,
    ) -> HttpResult<ListPortfolioFillsResponse> {
        let path = format!("portfolios/{}/fills", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(cursor) = &request.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        query_params.insert("start_date".to_string(), request.start_date.clone());
        if let Some(end_date) = &request.end_date {
            query_params.insert("end_date".to_string(), end_date.clone());
        }
        if let Some(sort_direction) = &request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPortfolioFillsResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn list_order_fills(
        &self,
        request: ListOrderFillsRequest,
    ) -> HttpResult<ListOrderFillsResponse> {
        let path = format!(
            "portfolios/{}/orders/{}/fills",
            request.portfolio_id, request.order_id
        );
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(cursor) = &request.cursor {
            query_params.insert("cursor".to_string(), cursor.clone());
        }
        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(sort_direction) = &request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetOrderFillsResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> HttpResult<CancelOrderResponse> {
        let path = format!(
            "portfolios/{}/orders/{}/cancel",
            request.portfolio_id, request.order_id
        );
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GeneratedCancelOrderResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn create_order(
        &self,
        request: CreateOrderRequest,
    ) -> HttpResult<CreateOrderResponse> {
        let path = format!("portfolios/{}/order", request.portfolio_id);
        let body = GeneratedCreateOrderRequest::from(&request);
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);

        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateOrderResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn order_preview(
        &self,
        request: OrderPreviewRequest,
    ) -> HttpResult<OrderPreviewResponse> {
        let path = format!("portfolios/{}/order_preview", request.portfolio_id);
        let body = GeneratedOrderPreviewRequest::from(&request);
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);

        let resp = self.client.execute(req).await?;
        let response: GeneratedPostOrderPreviewResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn create_quote_request(&self, request: RfqRequest) -> HttpResult<RfqResponse> {
        let path = format!("portfolios/{}/rfq", request.portfolio_id);
        let body = GeneratedRfq::from(&request);
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);

        let resp = self.client.execute(req).await?;
        let response: GeneratedQuoteResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn accept_quote(
        &self,
        request: AcceptQuoteRequest,
    ) -> HttpResult<AcceptQuoteResponse> {
        let path = format!("portfolios/{}/accept_quote", request.portfolio_id);
        let body = GeneratedAcceptQuoteRequest::from(&request);
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedAcceptQuoteResponse = resp.json().await?;
        Ok(response.into())
    }
}
