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

#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate api;
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate gaxi;
extern crate lazy_static;
extern crate longrunning;
extern crate lro;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// A service that is available for use by the consumer.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Service {
    /// The resource name of the consumer and service.
    ///
    /// A valid name would be:
    ///
    /// - projects/123/services/serviceusage.googleapis.com
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The resource name of the consumer.
    ///
    /// A valid name would be:
    ///
    /// - projects/123
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// The service configuration of the available service.
    /// Some fields may be filtered out of the configuration in responses to
    /// the `ListServices` method. These fields are present only in responses to
    /// the `GetService` method.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub config: std::option::Option<crate::model::ServiceConfig>,

    /// Whether or not the service has been enabled for use by the consumer.
    pub state: crate::model::State,
}

impl Service {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Service::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [parent][crate::model::Service::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [config][crate::model::Service::config].
    pub fn set_config<T: std::convert::Into<std::option::Option<crate::model::ServiceConfig>>>(
        mut self,
        v: T,
    ) -> Self {
        self.config = v.into();
        self
    }

    /// Sets the value of [state][crate::model::Service::state].
    pub fn set_state<T: std::convert::Into<crate::model::State>>(mut self, v: T) -> Self {
        self.state = v.into();
        self
    }
}

impl wkt::message::Message for Service {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.Service"
    }
}

/// The configuration of the service.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ServiceConfig {
    /// The DNS address at which this service is available.
    ///
    /// An example DNS address would be:
    /// `calendar.googleapis.com`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The product title for this service.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub title: std::string::String,

    /// A list of API interfaces exported by this service. Contains only the names,
    /// versions, and method names of the interfaces.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub apis: std::vec::Vec<wkt::Api>,

    /// Additional API documentation. Contains only the summary and the
    /// documentation URL.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub documentation: std::option::Option<api::model::Documentation>,

    /// Quota configuration.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub quota: std::option::Option<api::model::Quota>,

    /// Auth configuration. Contains only the OAuth rules.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub authentication: std::option::Option<api::model::Authentication>,

    /// Configuration controlling usage of this service.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub usage: std::option::Option<api::model::Usage>,

    /// Configuration for network endpoints. Contains only the names and aliases
    /// of the endpoints.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub endpoints: std::vec::Vec<api::model::Endpoint>,

    /// Defines the monitored resources used by this service. This is required
    /// by the [Service.monitoring][google.api.Service.monitoring] and
    /// [Service.logging][google.api.Service.logging] configurations.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub monitored_resources: std::vec::Vec<api::model::MonitoredResourceDescriptor>,

    /// Monitoring configuration.
    /// This should not include the 'producer_destinations' field.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub monitoring: std::option::Option<api::model::Monitoring>,
}

impl ServiceConfig {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::ServiceConfig::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [title][crate::model::ServiceConfig::title].
    pub fn set_title<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.title = v.into();
        self
    }

    /// Sets the value of [documentation][crate::model::ServiceConfig::documentation].
    pub fn set_documentation<
        T: std::convert::Into<std::option::Option<api::model::Documentation>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.documentation = v.into();
        self
    }

    /// Sets the value of [quota][crate::model::ServiceConfig::quota].
    pub fn set_quota<T: std::convert::Into<std::option::Option<api::model::Quota>>>(
        mut self,
        v: T,
    ) -> Self {
        self.quota = v.into();
        self
    }

    /// Sets the value of [authentication][crate::model::ServiceConfig::authentication].
    pub fn set_authentication<
        T: std::convert::Into<std::option::Option<api::model::Authentication>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.authentication = v.into();
        self
    }

    /// Sets the value of [usage][crate::model::ServiceConfig::usage].
    pub fn set_usage<T: std::convert::Into<std::option::Option<api::model::Usage>>>(
        mut self,
        v: T,
    ) -> Self {
        self.usage = v.into();
        self
    }

    /// Sets the value of [monitoring][crate::model::ServiceConfig::monitoring].
    pub fn set_monitoring<T: std::convert::Into<std::option::Option<api::model::Monitoring>>>(
        mut self,
        v: T,
    ) -> Self {
        self.monitoring = v.into();
        self
    }

    /// Sets the value of [apis][crate::model::ServiceConfig::apis].
    pub fn set_apis<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<wkt::Api>,
    {
        use std::iter::Iterator;
        self.apis = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [endpoints][crate::model::ServiceConfig::endpoints].
    pub fn set_endpoints<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<api::model::Endpoint>,
    {
        use std::iter::Iterator;
        self.endpoints = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [monitored_resources][crate::model::ServiceConfig::monitored_resources].
    pub fn set_monitored_resources<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<api::model::MonitoredResourceDescriptor>,
    {
        use std::iter::Iterator;
        self.monitored_resources = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ServiceConfig {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.ServiceConfig"
    }
}

/// The operation metadata returned for the batchend services operation.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationMetadata {
    /// The full name of the resources that this operation is directly
    /// associated with.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub resource_names: std::vec::Vec<std::string::String>,
}

impl OperationMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [resource_names][crate::model::OperationMetadata::resource_names].
    pub fn set_resource_names<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.resource_names = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for OperationMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.OperationMetadata"
    }
}

/// Request message for the `EnableService` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct EnableServiceRequest {
    /// Name of the consumer and service to enable the service on.
    ///
    /// The `EnableService` and `DisableService` methods currently only support
    /// projects.
    ///
    /// Enabling a service requires that the service is public or is shared with
    /// the user enabling the service.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl EnableServiceRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::EnableServiceRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for EnableServiceRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.EnableServiceRequest"
    }
}

/// Response message for the `EnableService` method.
/// This response message is assigned to the `response` field of the returned
/// Operation when that operation is done.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct EnableServiceResponse {
    /// The new state of the service after enabling.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub service: std::option::Option<crate::model::Service>,
}

impl EnableServiceResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [service][crate::model::EnableServiceResponse::service].
    pub fn set_service<T: std::convert::Into<std::option::Option<crate::model::Service>>>(
        mut self,
        v: T,
    ) -> Self {
        self.service = v.into();
        self
    }
}

impl wkt::message::Message for EnableServiceResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.EnableServiceResponse"
    }
}

/// Request message for the `DisableService` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DisableServiceRequest {
    /// Name of the consumer and service to disable the service on.
    ///
    /// The enable and disable methods currently only support projects.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Indicates if services that are enabled and which depend on this service
    /// should also be disabled. If not set, an error will be generated if any
    /// enabled services depend on the service to be disabled. When set, the
    /// service, and any enabled services that depend on it, will be disabled
    /// together.
    pub disable_dependent_services: bool,

    /// Defines the behavior for checking service usage when disabling a service.
    pub check_if_service_has_usage: crate::model::disable_service_request::CheckIfServiceHasUsage,
}

impl DisableServiceRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DisableServiceRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [disable_dependent_services][crate::model::DisableServiceRequest::disable_dependent_services].
    pub fn set_disable_dependent_services<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.disable_dependent_services = v.into();
        self
    }

    /// Sets the value of [check_if_service_has_usage][crate::model::DisableServiceRequest::check_if_service_has_usage].
    pub fn set_check_if_service_has_usage<
        T: std::convert::Into<crate::model::disable_service_request::CheckIfServiceHasUsage>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.check_if_service_has_usage = v.into();
        self
    }
}

impl wkt::message::Message for DisableServiceRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.DisableServiceRequest"
    }
}

/// Defines additional types related to [DisableServiceRequest].
pub mod disable_service_request {
    #[allow(unused_imports)]
    use super::*;

    /// Enum to determine if service usage should be checked when disabling a
    /// service.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct CheckIfServiceHasUsage(i32);

    impl CheckIfServiceHasUsage {
        /// When unset, the default behavior is used, which is SKIP.
        pub const CHECK_IF_SERVICE_HAS_USAGE_UNSPECIFIED: CheckIfServiceHasUsage =
            CheckIfServiceHasUsage::new(0);

        /// If set, skip checking service usage when disabling a service.
        pub const SKIP: CheckIfServiceHasUsage = CheckIfServiceHasUsage::new(1);

        /// If set, service usage is checked when disabling the service. If a
        /// service, or its dependents, has usage in the last 30 days, the request
        /// returns a FAILED_PRECONDITION error.
        pub const CHECK: CheckIfServiceHasUsage = CheckIfServiceHasUsage::new(2);

        /// Creates a new CheckIfServiceHasUsage instance.
        pub(crate) const fn new(value: i32) -> Self {
            Self(value)
        }

        /// Gets the enum value.
        pub fn value(&self) -> i32 {
            self.0
        }

        /// Gets the enum value as a string.
        pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
            match self.0 {
                0 => std::borrow::Cow::Borrowed("CHECK_IF_SERVICE_HAS_USAGE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("SKIP"),
                2 => std::borrow::Cow::Borrowed("CHECK"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "CHECK_IF_SERVICE_HAS_USAGE_UNSPECIFIED" => {
                    std::option::Option::Some(Self::CHECK_IF_SERVICE_HAS_USAGE_UNSPECIFIED)
                }
                "SKIP" => std::option::Option::Some(Self::SKIP),
                "CHECK" => std::option::Option::Some(Self::CHECK),
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for CheckIfServiceHasUsage {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for CheckIfServiceHasUsage {
        fn default() -> Self {
            Self::new(0)
        }
    }
}

/// Response message for the `DisableService` method.
/// This response message is assigned to the `response` field of the returned
/// Operation when that operation is done.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DisableServiceResponse {
    /// The new state of the service after disabling.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub service: std::option::Option<crate::model::Service>,
}

impl DisableServiceResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [service][crate::model::DisableServiceResponse::service].
    pub fn set_service<T: std::convert::Into<std::option::Option<crate::model::Service>>>(
        mut self,
        v: T,
    ) -> Self {
        self.service = v.into();
        self
    }
}

impl wkt::message::Message for DisableServiceResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.DisableServiceResponse"
    }
}

/// Request message for the `GetService` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetServiceRequest {
    /// Name of the consumer and service to get the `ConsumerState` for.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetServiceRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetServiceRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetServiceRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.GetServiceRequest"
    }
}

/// Request message for the `ListServices` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListServicesRequest {
    /// Parent to search for services on.
    ///
    /// An example name would be:
    /// `projects/123` where `123` is the project number.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Requested size of the next page of data.
    /// Requested page size cannot exceed 200.
    /// If not set, the default page size is 50.
    pub page_size: i32,

    /// Token identifying which result to start with, which is returned by a
    /// previous list call.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Only list services that conform to the given filter.
    /// The allowed filter strings are `state:ENABLED` and `state:DISABLED`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,
}

impl ListServicesRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListServicesRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListServicesRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListServicesRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::ListServicesRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

impl wkt::message::Message for ListServicesRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.ListServicesRequest"
    }
}

/// Response message for the `ListServices` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListServicesResponse {
    /// The available services for the requested project.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub services: std::vec::Vec<crate::model::Service>,

    /// Token that can be passed to `ListServices` to resume a paginated
    /// query.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,
}

impl ListServicesResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListServicesResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [services][crate::model::ListServicesResponse::services].
    pub fn set_services<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Service>,
    {
        use std::iter::Iterator;
        self.services = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListServicesResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.ListServicesResponse"
    }
}

#[doc(hidden)]
impl gax::paginator::internal::PageableResponse for ListServicesResponse {
    type PageItem = crate::model::Service;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.services
    }

    fn next_page_token(&self) -> std::string::String {
        use std::clone::Clone;
        self.next_page_token.clone()
    }
}

/// Request message for the `BatchEnableServices` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct BatchEnableServicesRequest {
    /// Parent to enable services on.
    ///
    /// An example name would be:
    /// `projects/123` where `123` is the project number.
    ///
    /// The `BatchEnableServices` method currently only supports projects.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// The identifiers of the services to enable on the project.
    ///
    /// A valid identifier would be:
    /// serviceusage.googleapis.com
    ///
    /// Enabling services requires that each service is public or is shared with
    /// the user enabling the service.
    ///
    /// A single request can enable a maximum of 20 services at a time. If more
    /// than 20 services are specified, the request will fail, and no state changes
    /// will occur.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub service_ids: std::vec::Vec<std::string::String>,
}

impl BatchEnableServicesRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::BatchEnableServicesRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [service_ids][crate::model::BatchEnableServicesRequest::service_ids].
    pub fn set_service_ids<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.service_ids = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for BatchEnableServicesRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.BatchEnableServicesRequest"
    }
}

/// Response message for the `BatchEnableServices` method.
/// This response message is assigned to the `response` field of the returned
/// Operation when that operation is done.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct BatchEnableServicesResponse {
    /// The new state of the services after enabling.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub services: std::vec::Vec<crate::model::Service>,

    /// If allow_partial_success is true, and one or more services could not be
    /// enabled, this field contains the details about each failure.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub failures: std::vec::Vec<crate::model::batch_enable_services_response::EnableFailure>,
}

impl BatchEnableServicesResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [services][crate::model::BatchEnableServicesResponse::services].
    pub fn set_services<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Service>,
    {
        use std::iter::Iterator;
        self.services = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [failures][crate::model::BatchEnableServicesResponse::failures].
    pub fn set_failures<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::batch_enable_services_response::EnableFailure>,
    {
        use std::iter::Iterator;
        self.failures = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for BatchEnableServicesResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.BatchEnableServicesResponse"
    }
}

/// Defines additional types related to [BatchEnableServicesResponse].
pub mod batch_enable_services_response {
    #[allow(unused_imports)]
    use super::*;

    /// Provides error messages for the failing services.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct EnableFailure {
        /// The service id of a service that could not be enabled.
        #[serde(skip_serializing_if = "std::string::String::is_empty")]
        pub service_id: std::string::String,

        /// An error message describing why the service could not be enabled.
        #[serde(skip_serializing_if = "std::string::String::is_empty")]
        pub error_message: std::string::String,
    }

    impl EnableFailure {
        pub fn new() -> Self {
            std::default::Default::default()
        }

        /// Sets the value of [service_id][crate::model::batch_enable_services_response::EnableFailure::service_id].
        pub fn set_service_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
            self.service_id = v.into();
            self
        }

        /// Sets the value of [error_message][crate::model::batch_enable_services_response::EnableFailure::error_message].
        pub fn set_error_message<T: std::convert::Into<std::string::String>>(
            mut self,
            v: T,
        ) -> Self {
            self.error_message = v.into();
            self
        }
    }

    impl wkt::message::Message for EnableFailure {
        fn typename() -> &'static str {
            "type.googleapis.com/google.api.serviceusage.v1.BatchEnableServicesResponse.EnableFailure"
        }
    }
}

/// Request message for the `BatchGetServices` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct BatchGetServicesRequest {
    /// Parent to retrieve services from.
    /// If this is set, the parent of all of the services specified in `names` must
    /// match this field. An example name would be: `projects/123` where `123` is
    /// the project number. The `BatchGetServices` method currently only supports
    /// projects.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Names of the services to retrieve.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    /// A single request can get a maximum of 30 services at a time.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub names: std::vec::Vec<std::string::String>,
}

impl BatchGetServicesRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::BatchGetServicesRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [names][crate::model::BatchGetServicesRequest::names].
    pub fn set_names<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.names = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for BatchGetServicesRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.BatchGetServicesRequest"
    }
}

/// Response message for the `BatchGetServices` method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct BatchGetServicesResponse {
    /// The requested Service states.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub services: std::vec::Vec<crate::model::Service>,
}

impl BatchGetServicesResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [services][crate::model::BatchGetServicesResponse::services].
    pub fn set_services<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Service>,
    {
        use std::iter::Iterator;
        self.services = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for BatchGetServicesResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.serviceusage.v1.BatchGetServicesResponse"
    }
}

/// Whether or not a service has been enabled for use by a consumer.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State(i32);

impl State {
    /// The default value, which indicates that the enabled state of the service
    /// is unspecified or not meaningful. Currently, all consumers other than
    /// projects (such as folders and organizations) are always in this state.
    pub const STATE_UNSPECIFIED: State = State::new(0);

    /// The service cannot be used by this consumer. It has either been explicitly
    /// disabled, or has never been enabled.
    pub const DISABLED: State = State::new(1);

    /// The service has been explicitly enabled for use by this consumer.
    pub const ENABLED: State = State::new(2);

    /// Creates a new State instance.
    pub(crate) const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the enum value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Gets the enum value as a string.
    pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
        match self.0 {
            0 => std::borrow::Cow::Borrowed("STATE_UNSPECIFIED"),
            1 => std::borrow::Cow::Borrowed("DISABLED"),
            2 => std::borrow::Cow::Borrowed("ENABLED"),
            _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
        }
    }

    /// Creates an enum value from the value name.
    pub fn from_str_name(name: &str) -> std::option::Option<Self> {
        match name {
            "STATE_UNSPECIFIED" => std::option::Option::Some(Self::STATE_UNSPECIFIED),
            "DISABLED" => std::option::Option::Some(Self::DISABLED),
            "ENABLED" => std::option::Option::Some(Self::ENABLED),
            _ => std::option::Option::None,
        }
    }
}

impl std::convert::From<i32> for State {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl std::default::Default for State {
    fn default() -> Self {
        Self::new(0)
    }
}
