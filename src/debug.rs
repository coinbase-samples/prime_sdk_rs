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
use async_trait::async_trait;
use core_rs::http_request::HttpRequest;
use core_rs::interceptor::PreRequestInterceptor;
use log::info;

// Debug interceptor to log request details
pub struct DebugInterceptor;

#[async_trait]
impl PreRequestInterceptor for DebugInterceptor {
    async fn intercept(&self, request: &mut HttpRequest) {
        let method = request.get_method();
        let url_path = request.get_url_path();
        let full_url = request.as_reqwest().url().to_string();

        info!("🌐 HTTP Request: {} {}", method, full_url);
        info!("📁 Path: {}", url_path);

        if let Some(ref json_body) = request.json_body {
            info!("📦 Body: {}", json_body);
        }

        if let Some(ref query_params) = request.query_params {
            info!("🔍 Query Params: {:?}", query_params);
        }
    }
}
