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
    activity::Activity, activity_category::ActivityCategory, activity_level::ActivityLevel,
    activity_status::ActivityStatus,
    get_activity_response::GetActivityResponse as GeneratedGetActivityResponse,
    get_entity_activities_response::GetEntityActivitiesResponse,
    get_portfolio_activities_response::GetPortfolioActivitiesResponse,
    paginated_response::PaginatedResponse, prime_activity_type::PrimeActivityType,
    sort_direction::SortDirection,
};
use crate::utils::PaginatedList;

// ============================================================================
// SHARED TRAITS AND BASE STRUCTS
// ============================================================================

/// Base struct for paginated activities list responses
#[derive(Debug, Clone)]
pub struct ActivitiesListResponse {
    pub activities: Vec<Activity>,
    pub pagination: PaginatedResponse,
}

impl ActivitiesListResponse {
    pub fn new(activities: Vec<Activity>, pagination: PaginatedResponse) -> Self {
        Self {
            activities,
            pagination,
        }
    }
}

impl PaginatedList<Activity> for ActivitiesListResponse {
    fn items(&self) -> &[Activity] {
        &self.activities
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

/// Base struct for activities list requests with common pagination fields
#[derive(Debug, Clone)]
pub struct ActivitiesListRequest {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl ActivitiesListRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            limit: None,
            sort_direction: None,
            start_date: None,
            end_date: None,
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

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
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

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request parameters for getting a specific activity
#[derive(Debug, Clone)]
pub struct GetActivityRequest {
    pub activity_id: String,
}

impl GetActivityRequest {
    pub fn new(activity_id: impl Into<String>) -> Self {
        Self {
            activity_id: activity_id.into(),
        }
    }
}

/// Request parameters for getting a specific portfolio activity
#[derive(Debug, Clone)]
pub struct GetPortfolioActivityRequest {
    pub portfolio_id: String,
    pub activity_id: String,
}

impl GetPortfolioActivityRequest {
    pub fn new(portfolio_id: impl Into<String>, activity_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            activity_id: activity_id.into(),
        }
    }
}

/// Request parameters for getting entity activities
#[derive(Debug, Clone)]
pub struct ListEntityActivitiesRequest {
    pub entity_id: String,
    pub activity_level: Option<ActivityLevel>,
    pub pagination: ActivitiesListRequest,
}

impl ListEntityActivitiesRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            activity_level: None,
            pagination: ActivitiesListRequest::new(),
        }
    }

    pub fn with_activity_level(mut self, activity_level: ActivityLevel) -> Self {
        self.activity_level = Some(activity_level);
        self
    }

    // Delegate pagination methods to the inner struct
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.pagination = self.pagination.with_cursor(cursor_str);
        }
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.pagination = self.pagination.with_sort_direction(sort_direction);
        self
    }

    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.pagination = self.pagination.with_start_date(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.pagination = self.pagination.with_end_date(end_date);
        self
    }
}

/// Request parameters for getting portfolio activities
#[derive(Debug, Clone)]
pub struct ListPortfolioActivitiesRequest {
    pub portfolio_id: String,
    pub pagination: ActivitiesListRequest,
}

impl ListPortfolioActivitiesRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            pagination: ActivitiesListRequest::new(),
        }
    }

    // Delegate pagination methods to the inner struct
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.pagination = self.pagination.with_cursor(cursor_str);
        }
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.pagination = self.pagination.with_sort_direction(sort_direction);
        self
    }

    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.pagination = self.pagination.with_start_date(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.pagination = self.pagination.with_end_date(end_date);
        self
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Wrapper for entity activities list response
pub type ListEntityActivitiesResponse = ActivitiesListResponse;

impl From<GetEntityActivitiesResponse> for ListEntityActivitiesResponse {
    fn from(response: GetEntityActivitiesResponse) -> Self {
        Self::new(response.activities, *response.pagination)
    }
}

/// Wrapper for portfolio activities list response
pub type ListPortfolioActivitiesResponse = ActivitiesListResponse;

impl From<GetPortfolioActivitiesResponse> for ListPortfolioActivitiesResponse {
    fn from(response: GetPortfolioActivitiesResponse) -> Self {
        Self::new(response.activities, *response.pagination)
    }
}

/// Wrapper for single activity response
#[derive(Debug, Clone)]
pub struct GetActivityResponse {
    pub activity: Activity,
}

impl From<GeneratedGetActivityResponse> for GetActivityResponse {
    fn from(response: GeneratedGetActivityResponse) -> Self {
        Self {
            activity: *response.activity,
        }
    }
}

impl GetActivityResponse {
    /// Get the activity from the response
    pub fn activity(&self) -> &Activity {
        &self.activity
    }

    /// Get the activity ID
    pub fn activity_id(&self) -> Option<&str> {
        self.activity.id.as_deref()
    }

    /// Get the activity title
    pub fn title(&self) -> Option<&str> {
        self.activity.title.as_deref()
    }

    /// Get the activity description
    pub fn description(&self) -> Option<&str> {
        self.activity.description.as_deref()
    }

    /// Get the activity status
    pub fn status(&self) -> Option<&ActivityStatus> {
        self.activity.status.as_ref()
    }

    /// Get the activity type
    pub fn activity_type(&self) -> Option<&PrimeActivityType> {
        self.activity.r#type.as_ref()
    }

    /// Get the activity category
    pub fn category(&self) -> Option<&ActivityCategory> {
        self.activity.category.as_ref()
    }

    /// Get the activity symbols
    pub fn symbols(&self) -> Option<&[String]> {
        self.activity.symbols.as_deref()
    }

    /// Get the activity creation timestamp
    pub fn created_at(&self) -> Option<&str> {
        self.activity.created_at.as_deref()
    }

    /// Get the activity update timestamp
    pub fn updated_at(&self) -> Option<&str> {
        self.activity.updated_at.as_deref()
    }
}
