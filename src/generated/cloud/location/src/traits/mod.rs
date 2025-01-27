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
#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;

pub(crate) mod dyntraits;

/// An abstract interface that provides location-related information for
/// a service. Service-specific metadata is provided through the
/// [Location.metadata][google.cloud.location.Location.metadata] field.
///
/// [google.cloud.location.Location.metadata]: crate::model::Location::metadata
///
/// # Mocking
///
/// Application developers may use this trait to mock the cloud clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Locations: std::fmt::Debug + Send + Sync {
    /// Lists information about the supported locations for this service.
    fn list_locations(
        &self,
        _req: crate::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListLocationsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets information about a location.
    fn get_location(
        &self,
        _req: crate::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Location>> + Send {
        std::future::ready::<crate::Result<crate::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
