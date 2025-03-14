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
extern crate location;
extern crate longrunning;
extern crate lro;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Definition of a Serverless VPC Access connector.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Connector {
    /// The resource name in the format `projects/*/locations/*/connectors/*`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Name of a VPC network.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub network: std::string::String,

    /// The range of internal addresses that follows RFC 4632 notation.
    /// Example: `10.132.0.0/28`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub ip_cidr_range: std::string::String,

    /// Output only. State of the VPC access connector.
    pub state: crate::model::connector::State,

    /// Minimum throughput of the connector in Mbps. Default and min is 200.
    pub min_throughput: i32,

    /// Maximum throughput of the connector in Mbps. Default is 300, max is 1000.
    pub max_throughput: i32,

    /// Output only. List of projects using the connector.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub connected_projects: std::vec::Vec<std::string::String>,

    /// The subnet in which to house the VPC Access Connector.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub subnet: std::option::Option<crate::model::connector::Subnet>,

    /// Machine type of VM Instance underlying connector. Default is e2-micro
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub machine_type: std::string::String,

    /// Minimum value of instances in autoscaling group underlying the connector.
    pub min_instances: i32,

    /// Maximum value of instances in autoscaling group underlying the connector.
    pub max_instances: i32,
}

impl Connector {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Connector::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [network][crate::model::Connector::network].
    pub fn set_network<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.network = v.into();
        self
    }

    /// Sets the value of [ip_cidr_range][crate::model::Connector::ip_cidr_range].
    pub fn set_ip_cidr_range<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.ip_cidr_range = v.into();
        self
    }

    /// Sets the value of [state][crate::model::Connector::state].
    pub fn set_state<T: std::convert::Into<crate::model::connector::State>>(
        mut self,
        v: T,
    ) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of [min_throughput][crate::model::Connector::min_throughput].
    pub fn set_min_throughput<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.min_throughput = v.into();
        self
    }

    /// Sets the value of [max_throughput][crate::model::Connector::max_throughput].
    pub fn set_max_throughput<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.max_throughput = v.into();
        self
    }

    /// Sets the value of [subnet][crate::model::Connector::subnet].
    pub fn set_subnet<
        T: std::convert::Into<std::option::Option<crate::model::connector::Subnet>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.subnet = v.into();
        self
    }

    /// Sets the value of [machine_type][crate::model::Connector::machine_type].
    pub fn set_machine_type<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.machine_type = v.into();
        self
    }

    /// Sets the value of [min_instances][crate::model::Connector::min_instances].
    pub fn set_min_instances<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.min_instances = v.into();
        self
    }

    /// Sets the value of [max_instances][crate::model::Connector::max_instances].
    pub fn set_max_instances<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.max_instances = v.into();
        self
    }

    /// Sets the value of [connected_projects][crate::model::Connector::connected_projects].
    pub fn set_connected_projects<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.connected_projects = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for Connector {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.vpcaccess.v1.Connector"
    }
}

/// Defines additional types related to Connector
pub mod connector {
    #[allow(unused_imports)]
    use super::*;

    /// The subnet in which to house the connector
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Subnet {
        /// Subnet name (relative, not fully qualified).
        /// E.g. if the full subnet selfLink is
        /// <https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetName}>
        /// the correct input for this field would be {subnetName}
        #[serde(skip_serializing_if = "std::string::String::is_empty")]
        pub name: std::string::String,

        /// Project in which the subnet exists.
        /// If not set, this project is assumed to be the project for which
        /// the connector create request was issued.
        #[serde(skip_serializing_if = "std::string::String::is_empty")]
        pub project_id: std::string::String,
    }

    impl Subnet {
        pub fn new() -> Self {
            std::default::Default::default()
        }

        /// Sets the value of [name][crate::model::connector::Subnet::name].
        pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
            self.name = v.into();
            self
        }

        /// Sets the value of [project_id][crate::model::connector::Subnet::project_id].
        pub fn set_project_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
            self.project_id = v.into();
            self
        }
    }

    impl wkt::message::Message for Subnet {
        fn typename() -> &'static str {
            "type.googleapis.com/google.cloud.vpcaccess.v1.Connector.Subnet"
        }
    }

    /// State of a connector.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(i32);

    impl State {
        /// Invalid state.
        pub const STATE_UNSPECIFIED: State = State::new(0);

        /// Connector is deployed and ready to receive traffic.
        pub const READY: State = State::new(1);

        /// An Insert operation is in progress. Transient condition.
        pub const CREATING: State = State::new(2);

        /// A Delete operation is in progress. Transient condition.
        pub const DELETING: State = State::new(3);

        /// Connector is in a bad state, manual deletion recommended.
        pub const ERROR: State = State::new(4);

        /// The connector is being updated.
        pub const UPDATING: State = State::new(5);

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
                1 => std::borrow::Cow::Borrowed("READY"),
                2 => std::borrow::Cow::Borrowed("CREATING"),
                3 => std::borrow::Cow::Borrowed("DELETING"),
                4 => std::borrow::Cow::Borrowed("ERROR"),
                5 => std::borrow::Cow::Borrowed("UPDATING"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "STATE_UNSPECIFIED" => std::option::Option::Some(Self::STATE_UNSPECIFIED),
                "READY" => std::option::Option::Some(Self::READY),
                "CREATING" => std::option::Option::Some(Self::CREATING),
                "DELETING" => std::option::Option::Some(Self::DELETING),
                "ERROR" => std::option::Option::Some(Self::ERROR),
                "UPDATING" => std::option::Option::Some(Self::UPDATING),
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
}

/// Request for creating a Serverless VPC Access connector.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateConnectorRequest {
    /// Required. The project and location in which the configuration should be created,
    /// specified in the format `projects/*/locations/*`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Required. The ID to use for this connector.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub connector_id: std::string::String,

    /// Required. Resource to create.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub connector: std::option::Option<crate::model::Connector>,
}

impl CreateConnectorRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::CreateConnectorRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [connector_id][crate::model::CreateConnectorRequest::connector_id].
    pub fn set_connector_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.connector_id = v.into();
        self
    }

    /// Sets the value of [connector][crate::model::CreateConnectorRequest::connector].
    pub fn set_connector<T: std::convert::Into<std::option::Option<crate::model::Connector>>>(
        mut self,
        v: T,
    ) -> Self {
        self.connector = v.into();
        self
    }
}

impl wkt::message::Message for CreateConnectorRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.vpcaccess.v1.CreateConnectorRequest"
    }
}

/// Request for getting a Serverless VPC Access connector.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetConnectorRequest {
    /// Required. Name of a Serverless VPC Access connector to get.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetConnectorRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetConnectorRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetConnectorRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.vpcaccess.v1.GetConnectorRequest"
    }
}

/// Request for listing Serverless VPC Access connectors in a location.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListConnectorsRequest {
    /// Required. The project and location from which the routes should be listed.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Maximum number of functions to return per call.
    pub page_size: i32,

    /// Continuation token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,
}

impl ListConnectorsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListConnectorsRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListConnectorsRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListConnectorsRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListConnectorsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.vpcaccess.v1.ListConnectorsRequest"
    }
}

/// Response for listing Serverless VPC Access connectors.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListConnectorsResponse {
    /// List of Serverless VPC Access connectors.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub connectors: std::vec::Vec<crate::model::Connector>,

    /// Continuation token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,
}

impl ListConnectorsResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListConnectorsResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [connectors][crate::model::ListConnectorsResponse::connectors].
    pub fn set_connectors<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Connector>,
    {
        use std::iter::Iterator;
        self.connectors = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListConnectorsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.vpcaccess.v1.ListConnectorsResponse"
    }
}

impl gax::paginator::PageableResponse for ListConnectorsResponse {
    type PageItem = crate::model::Connector;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.connectors
    }

    fn next_page_token(&self) -> std::string::String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

/// Request for deleting a Serverless VPC Access connector.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteConnectorRequest {
    /// Required. Name of a Serverless VPC Access connector to delete.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl DeleteConnectorRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DeleteConnectorRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for DeleteConnectorRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.vpcaccess.v1.DeleteConnectorRequest"
    }
}

/// Metadata for google.longrunning.Operation.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationMetadata {
    /// Output only. Method that initiated the operation e.g.
    /// google.cloud.vpcaccess.v1.Connectors.CreateConnector.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub method: std::string::String,

    /// Output only. Time when the operation was created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Output only. Time when the operation completed.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub end_time: std::option::Option<wkt::Timestamp>,

    /// Output only. Name of the resource that this operation is acting on e.g.
    /// projects/my-project/locations/us-central1/connectors/v1.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub target: std::string::String,
}

impl OperationMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [method][crate::model::OperationMetadata::method].
    pub fn set_method<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.method = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::OperationMetadata::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [end_time][crate::model::OperationMetadata::end_time].
    pub fn set_end_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.end_time = v.into();
        self
    }

    /// Sets the value of [target][crate::model::OperationMetadata::target].
    pub fn set_target<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.target = v.into();
        self
    }
}

impl wkt::message::Message for OperationMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.vpcaccess.v1.OperationMetadata"
    }
}
