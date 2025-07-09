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
pub mod client;
pub mod constants;
pub mod credentials;
pub mod debug;
pub mod error;
pub mod services;
pub mod types;
pub mod utils;

pub use client::PrimeClient;
pub use services::*;

// Re-export commonly used core_rs types for convenience
pub use core_rs::http_method::HttpMethod;
pub use core_rs::http_request::HttpRequest;
pub use core_rs::http_response::HttpResponse;
pub use core_rs::http_status_code::HttpStatusCode;
pub use core_rs::http_url::HttpUrl;
