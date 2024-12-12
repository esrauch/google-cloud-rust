// Copyright 2024 Google LLC
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
use std::sync::Arc;

/// An implementation of [crate::traits::Locations] to make requests with.
///
/// `Locations` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `Locations` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Locations` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// An abstract interface that provides location-related information for
/// a service. Service-specific metadata is provided through the
/// [Location.metadata][google.cloud.location.Location.metadata] field.
///
/// [google.cloud.location.Location.metadata]: crate::model::Location::metadata
#[derive(Clone, Debug)]
pub struct Locations {
    inner: Arc<dyn crate::traits::dyntraits::Locations>,
}

impl Locations {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: crate::ConfigBuilder,
    ) -> Result<Arc<dyn crate::traits::dyntraits::Locations>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: crate::ConfigBuilder) -> Result<impl crate::traits::Locations> {
        crate::transport::Locations::new(conf).await
    }

    async fn build_with_tracing(
        conf: crate::ConfigBuilder,
    ) -> Result<impl crate::traits::Locations> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Locations::new)
    }
}

impl crate::traits::Locations for Locations {
    /// Lists information about the supported locations for this service.
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest,
    ) -> Result<crate::model::ListLocationsResponse> {
        self.inner.list_locations(req).await
    }

    /// Gets information about a location.
    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest,
    ) -> Result<crate::model::Location> {
        self.inner.get_location(req).await
    }
}
