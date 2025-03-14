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
extern crate grafeas;
extern crate iam_v1;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Request to get a vulnerability summary for some set of occurrences.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetVulnerabilityOccurrencesSummaryRequest {
    /// Required. The name of the project to get a vulnerability summary for in the form of
    /// `projects/[PROJECT_ID]`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// The filter expression.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,
}

impl GetVulnerabilityOccurrencesSummaryRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::GetVulnerabilityOccurrencesSummaryRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::GetVulnerabilityOccurrencesSummaryRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

impl wkt::message::Message for GetVulnerabilityOccurrencesSummaryRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.containeranalysis.v1.GetVulnerabilityOccurrencesSummaryRequest"
    }
}

/// A summary of how many vulnerability occurrences there are per resource and
/// severity type.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct VulnerabilityOccurrencesSummary {
    /// A listing by resource of the number of fixable and total vulnerabilities.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub counts:
        std::vec::Vec<crate::model::vulnerability_occurrences_summary::FixableTotalByDigest>,
}

impl VulnerabilityOccurrencesSummary {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [counts][crate::model::VulnerabilityOccurrencesSummary::counts].
    pub fn set_counts<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<
            crate::model::vulnerability_occurrences_summary::FixableTotalByDigest,
        >,
    {
        use std::iter::Iterator;
        self.counts = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for VulnerabilityOccurrencesSummary {
    fn typename() -> &'static str {
        "type.googleapis.com/google.devtools.containeranalysis.v1.VulnerabilityOccurrencesSummary"
    }
}

/// Defines additional types related to VulnerabilityOccurrencesSummary
pub mod vulnerability_occurrences_summary {
    #[allow(unused_imports)]
    use super::*;

    /// Per resource and severity counts of fixable and total vulnerabilities.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(default, rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct FixableTotalByDigest {
        /// The affected resource.
        #[serde(skip_serializing_if = "std::string::String::is_empty")]
        pub resource_uri: std::string::String,

        /// The severity for this count. SEVERITY_UNSPECIFIED indicates total across
        /// all severities.
        pub severity: grafeas::model::Severity,

        /// The number of fixable vulnerabilities associated with this resource.
        #[serde_as(as = "serde_with::DisplayFromStr")]
        pub fixable_count: i64,

        /// The total number of vulnerabilities associated with this resource.
        #[serde_as(as = "serde_with::DisplayFromStr")]
        pub total_count: i64,
    }

    impl FixableTotalByDigest {
        pub fn new() -> Self {
            std::default::Default::default()
        }

        /// Sets the value of [resource_uri][crate::model::vulnerability_occurrences_summary::FixableTotalByDigest::resource_uri].
        pub fn set_resource_uri<T: std::convert::Into<std::string::String>>(
            mut self,
            v: T,
        ) -> Self {
            self.resource_uri = v.into();
            self
        }

        /// Sets the value of [severity][crate::model::vulnerability_occurrences_summary::FixableTotalByDigest::severity].
        pub fn set_severity<T: std::convert::Into<grafeas::model::Severity>>(
            mut self,
            v: T,
        ) -> Self {
            self.severity = v.into();
            self
        }

        /// Sets the value of [fixable_count][crate::model::vulnerability_occurrences_summary::FixableTotalByDigest::fixable_count].
        pub fn set_fixable_count<T: std::convert::Into<i64>>(mut self, v: T) -> Self {
            self.fixable_count = v.into();
            self
        }

        /// Sets the value of [total_count][crate::model::vulnerability_occurrences_summary::FixableTotalByDigest::total_count].
        pub fn set_total_count<T: std::convert::Into<i64>>(mut self, v: T) -> Self {
            self.total_count = v.into();
            self
        }
    }

    impl wkt::message::Message for FixableTotalByDigest {
        fn typename() -> &'static str {
            "type.googleapis.com/google.devtools.containeranalysis.v1.VulnerabilityOccurrencesSummary.FixableTotalByDigest"
        }
    }
}
