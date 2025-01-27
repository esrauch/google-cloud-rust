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
#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;

pub(crate) mod dyntraits;

/// Manages connections to source code repositories.
///
/// # Mocking
///
/// Application developers may use this trait to mock the cloudbuild clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait RepositoryManager: std::fmt::Debug + Send + Sync {
    /// Creates a Connection.
    fn create_connection(
        &self,
        _req: crate::model::CreateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets details of a single connection.
    fn get_connection(
        &self,
        _req: crate::model::GetConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Connection>> + Send {
        std::future::ready::<crate::Result<crate::model::Connection>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists Connections in a given project and location.
    fn list_connections(
        &self,
        _req: crate::model::ListConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListConnectionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListConnectionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Updates a single connection.
    fn update_connection(
        &self,
        _req: crate::model::UpdateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a single connection.
    fn delete_connection(
        &self,
        _req: crate::model::DeleteConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a Repository.
    fn create_repository(
        &self,
        _req: crate::model::CreateRepositoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates multiple repositories inside a connection.
    fn batch_create_repositories(
        &self,
        _req: crate::model::BatchCreateRepositoriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets details of a single repository.
    fn get_repository(
        &self,
        _req: crate::model::GetRepositoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Repository>> + Send {
        std::future::ready::<crate::Result<crate::model::Repository>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists Repositories in a given connection.
    fn list_repositories(
        &self,
        _req: crate::model::ListRepositoriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRepositoriesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRepositoriesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Deletes a single repository.
    fn delete_repository(
        &self,
        _req: crate::model::DeleteRepositoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Fetches read/write token of a given repository.
    fn fetch_read_write_token(
        &self,
        _req: crate::model::FetchReadWriteTokenRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FetchReadWriteTokenResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::FetchReadWriteTokenResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Fetches read token of a given repository.
    fn fetch_read_token(
        &self,
        _req: crate::model::FetchReadTokenRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FetchReadTokenResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::FetchReadTokenResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// FetchLinkableRepositories get repositories from SCM that are
    /// accessible and could be added to the connection.
    fn fetch_linkable_repositories(
        &self,
        _req: crate::model::FetchLinkableRepositoriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::FetchLinkableRepositoriesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::FetchLinkableRepositoriesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Fetch the list of branches or tags for a given repository.
    fn fetch_git_refs(
        &self,
        _req: crate::model::FetchGitRefsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FetchGitRefsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::FetchGitRefsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}
