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
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate gclient;
extern crate lazy_static;
extern crate reqwest;
extern crate rpc;
extern crate rpc_context;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Request message for the Check method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CheckRequest {
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    ///
    /// See
    /// [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    /// for the definition of a service name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub service_name: std::string::String,

    /// Specifies the version of the service configuration that should be used to
    /// process the request. Must not be empty. Set this field to 'latest' to
    /// specify using the latest configuration.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub service_config_id: std::string::String,

    /// Describes attributes about the operation being executed by the service.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub attributes: std::option::Option<rpc_context::model::AttributeContext>,

    /// Describes the resources and the policies applied to each resource.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub resources: std::vec::Vec<crate::model::ResourceInfo>,

    /// Optional. Contains a comma-separated list of flags.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub flags: std::string::String,
}

impl CheckRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [service_name][crate::model::CheckRequest::service_name].
    pub fn set_service_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.service_name = v.into();
        self
    }

    /// Sets the value of [service_config_id][crate::model::CheckRequest::service_config_id].
    pub fn set_service_config_id<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.service_config_id = v.into();
        self
    }

    /// Sets the value of [attributes][crate::model::CheckRequest::attributes].
    pub fn set_attributes<
        T: std::convert::Into<std::option::Option<rpc_context::model::AttributeContext>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.attributes = v.into();
        self
    }

    /// Sets the value of [flags][crate::model::CheckRequest::flags].
    pub fn set_flags<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.flags = v.into();
        self
    }

    /// Sets the value of [resources][crate::model::CheckRequest::resources].
    pub fn set_resources<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::ResourceInfo>,
    {
        use std::iter::Iterator;
        self.resources = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for CheckRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.servicecontrol.v2.CheckRequest"
    }
}

/// Describes a resource referenced in the request.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResourceInfo {
    /// The name of the resource referenced in the request.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// The resource type in the format of "{service}/{kind}".
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub r#type: std::string::String,

    /// The resource permission needed for this request.
    /// The format must be "{service}/{plural}.{verb}".
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub permission: std::string::String,

    /// Optional. The identifier of the container of this resource. For Google
    /// Cloud APIs, the resource container must be one of the following formats:
    /// - `projects/<project-id or project-number>`
    /// - `folders/<folder-id>`
    /// - `organizations/<organization-id>`
    /// For the policy enforcement on the container level (VPCSC and Location
    /// Policy check), this field takes precedence on the container extracted from
    /// name when presents.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub container: std::string::String,

    /// Optional. The location of the resource. The value must be a valid zone,
    /// region or multiregion. For example: "europe-west4" or
    /// "northamerica-northeast1-a"
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub location: std::string::String,
}

impl ResourceInfo {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::ResourceInfo::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [r#type][crate::model::ResourceInfo::type].
    pub fn set_type<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.r#type = v.into();
        self
    }

    /// Sets the value of [permission][crate::model::ResourceInfo::permission].
    pub fn set_permission<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.permission = v.into();
        self
    }

    /// Sets the value of [container][crate::model::ResourceInfo::container].
    pub fn set_container<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.container = v.into();
        self
    }

    /// Sets the value of [location][crate::model::ResourceInfo::location].
    pub fn set_location<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

impl wkt::message::Message for ResourceInfo {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.servicecontrol.v2.ResourceInfo"
    }
}

/// Response message for the Check method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CheckResponse {
    /// Operation is allowed when this field is not set. Any non-'OK' status
    /// indicates a denial; [google.rpc.Status.details][google.rpc.Status.details]
    /// would contain additional details about the denial.
    ///
    /// [google.rpc.Status.details]: rpc::model::Status::details
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub status: std::option::Option<rpc::model::Status>,

    /// Returns a set of request contexts generated from the `CheckRequest`.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub headers: std::collections::HashMap<std::string::String, std::string::String>,
}

impl CheckResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [status][crate::model::CheckResponse::status].
    pub fn set_status<T: std::convert::Into<std::option::Option<rpc::model::Status>>>(
        mut self,
        v: T,
    ) -> Self {
        self.status = v.into();
        self
    }

    /// Sets the value of [headers][crate::model::CheckResponse::headers].
    pub fn set_headers<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.headers = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }
}

impl wkt::message::Message for CheckResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.servicecontrol.v2.CheckResponse"
    }
}

/// Request message for the Report method.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReportRequest {
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    ///
    /// See
    /// [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    /// for the definition of a service name.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub service_name: std::string::String,

    /// Specifies the version of the service configuration that should be used to
    /// process the request. Must not be empty. Set this field to 'latest' to
    /// specify using the latest configuration.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub service_config_id: std::string::String,

    /// Describes the list of operations to be reported. Each operation is
    /// represented as an AttributeContext, and contains all attributes around an
    /// API access.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub operations: std::vec::Vec<rpc_context::model::AttributeContext>,
}

impl ReportRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [service_name][crate::model::ReportRequest::service_name].
    pub fn set_service_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.service_name = v.into();
        self
    }

    /// Sets the value of [service_config_id][crate::model::ReportRequest::service_config_id].
    pub fn set_service_config_id<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.service_config_id = v.into();
        self
    }

    /// Sets the value of [operations][crate::model::ReportRequest::operations].
    pub fn set_operations<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<rpc_context::model::AttributeContext>,
    {
        use std::iter::Iterator;
        self.operations = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ReportRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.servicecontrol.v2.ReportRequest"
    }
}

/// Response message for the Report method.
/// If the request contains any invalid data, the server returns an RPC error.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReportResponse {}

impl ReportResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }
}

impl wkt::message::Message for ReportResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.servicecontrol.v2.ReportResponse"
    }
}

/// Message containing resource details in a batch mode.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResourceInfoList {
    /// The resource details.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub resources: std::vec::Vec<crate::model::ResourceInfo>,
}

impl ResourceInfoList {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [resources][crate::model::ResourceInfoList::resources].
    pub fn set_resources<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::ResourceInfo>,
    {
        use std::iter::Iterator;
        self.resources = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ResourceInfoList {
    fn typename() -> &'static str {
        "type.googleapis.com/google.api.servicecontrol.v2.ResourceInfoList"
    }
}
