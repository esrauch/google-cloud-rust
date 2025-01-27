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

/// Google Cloud Functions is used to deploy functions that are executed by
/// Google in response to various events. Data connected with that event is
/// passed to a function as the input data.
///
/// A **function** is a resource which describes a function that should be
/// executed and how it is triggered.
///
/// # Mocking
///
/// Application developers may use this trait to mock the cloudfunctions clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait FunctionService: std::fmt::Debug + Send + Sync {
    /// Returns a function with the given name from the requested project.
    fn get_function(
        &self,
        _req: crate::model::GetFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Function>> + Send {
        std::future::ready::<crate::Result<crate::model::Function>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns a list of functions that belong to the requested project.
    fn list_functions(
        &self,
        _req: crate::model::ListFunctionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListFunctionsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListFunctionsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a new function. If a function with the given name already exists in
    /// the specified project, the long running operation will return
    /// `ALREADY_EXISTS` error.
    fn create_function(
        &self,
        _req: crate::model::CreateFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates existing function.
    fn update_function(
        &self,
        _req: crate::model::UpdateFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a function with the given name from the specified project. If the
    /// given function is used by some trigger, the trigger will be updated to
    /// remove this function.
    fn delete_function(
        &self,
        _req: crate::model::DeleteFunctionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns a signed URL for uploading a function source code.
    /// For more information about the signed URL usage see:
    /// <https://cloud.google.com/storage/docs/access-control/signed-urls>.
    /// Once the function source code upload is complete, the used signed
    /// URL should be provided in CreateFunction or UpdateFunction request
    /// as a reference to the function source code.
    ///
    /// When uploading source code to the generated signed URL, please follow
    /// these restrictions:
    ///
    /// * Source file type should be a zip file.
    /// * No credentials should be attached - the signed URLs provide access to the
    ///   target bucket using internal service identity; if credentials were
    ///   attached, the identity from the credentials would be used, but that
    ///   identity does not have permissions to upload files to the URL.
    ///
    /// When making a HTTP PUT request, specify this header:
    ///
    /// * `content-type: application/zip`
    ///
    /// Do not specify this header:
    ///
    /// * `Authorization: Bearer YOUR_TOKEN`
    fn generate_upload_url(
        &self,
        _req: crate::model::GenerateUploadUrlRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GenerateUploadUrlResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GenerateUploadUrlResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Returns a signed URL for downloading deployed function source code.
    /// The URL is only valid for a limited period and should be used within
    /// 30 minutes of generation.
    /// For more information about the signed URL usage see:
    /// <https://cloud.google.com/storage/docs/access-control/signed-urls>
    fn generate_download_url(
        &self,
        _req: crate::model::GenerateDownloadUrlRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GenerateDownloadUrlResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GenerateDownloadUrlResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Returns a list of runtimes that are supported for the requested project.
    fn list_runtimes(
        &self,
        _req: crate::model::ListRuntimesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRuntimesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRuntimesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists information about the supported locations for this service.
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
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
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
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
