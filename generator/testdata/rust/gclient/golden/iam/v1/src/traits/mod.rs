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

use gax::error::Error;

pub(crate) mod dyntraits;

/// API Overview
///
/// Manages Identity and Access Management (IAM) policies.
///
/// Any implementation of an API that offers access control features
/// implements the google.iam.v1.IAMPolicy interface.
///
/// ## Data model
///
/// Access control is applied when a principal (user or service account), takes
/// some action on a resource exposed by a service. Resources, identified by
/// URI-like names, are the unit of access control specification. Service
/// implementations can choose the granularity of access control and the
/// supported permissions for their resources.
/// For example one database service may allow access control to be
/// specified only at the Table level, whereas another might allow access control
/// to also be specified at the Column level.
///
/// ## Policy Structure
///
/// See google.iam.v1.Policy
///
/// This is intentionally not a CRUD style API because access control policies
/// are created and deleted implicitly with the resources to which they are
/// attached.
///
/// # Notes
///
/// Application developers may use this trait to mock the  clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Iampolicy: std::fmt::Debug + Send + Sync {

    /// Sets the access control policy on the specified resource. Replaces any
    /// existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    fn set_iam_policy(
        &self,
        _req: crate::model::SetIamPolicyRequest,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Policy>> + Send {
        std::future::ready::<crate::Result<crate::model::Policy>>(Err(Error::other("unimplemented")))
    }

    /// Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy
    /// set.
    fn get_iam_policy(
        &self,
        _req: crate::model::GetIamPolicyRequest,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Policy>> + Send {
        std::future::ready::<crate::Result<crate::model::Policy>>(Err(Error::other("unimplemented")))
    }

    /// Returns permissions that a caller has on the specified resource.
    /// If the resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    fn test_iam_permissions(
        &self,
        _req: crate::model::TestIamPermissionsRequest,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TestIamPermissionsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::TestIamPermissionsResponse>>(Err(Error::other("unimplemented")))
    }
}

