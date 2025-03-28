// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
use crate::Result;

/// Implements a [ServiceHealth](super::stub::ServiceHealth) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ServiceHealth<T>
where
    T: super::stub::ServiceHealth + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ServiceHealth<T>
where
    T: super::stub::ServiceHealth + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::ServiceHealth for ServiceHealth<T>
where
    T: super::stub::ServiceHealth + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_events(
        &self,
        req: crate::model::ListEventsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEventsResponse> {
        self.inner.list_events(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_event(
        &self,
        req: crate::model::GetEventRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Event> {
        self.inner.get_event(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_organization_events(
        &self,
        req: crate::model::ListOrganizationEventsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListOrganizationEventsResponse> {
        self.inner.list_organization_events(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_organization_event(
        &self,
        req: crate::model::GetOrganizationEventRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::OrganizationEvent> {
        self.inner.get_organization_event(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_organization_impacts(
        &self,
        req: crate::model::ListOrganizationImpactsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListOrganizationImpactsResponse> {
        self.inner.list_organization_impacts(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_organization_impact(
        &self,
        req: crate::model::GetOrganizationImpactRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::OrganizationImpact> {
        self.inner.get_organization_impact(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }
}
