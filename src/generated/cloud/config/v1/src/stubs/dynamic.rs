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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::Config].
#[async_trait::async_trait]
pub trait Config: std::fmt::Debug + Send + Sync {
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDeploymentsResponse>;

    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment>;

    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn update_deployment(
        &self,
        req: crate::model::UpdateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_revisions(
        &self,
        req: crate::model::ListRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRevisionsResponse>;

    async fn get_revision(
        &self,
        req: crate::model::GetRevisionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Revision>;

    async fn get_resource(
        &self,
        req: crate::model::GetResourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Resource>;

    async fn list_resources(
        &self,
        req: crate::model::ListResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListResourcesResponse>;

    async fn export_deployment_statefile(
        &self,
        req: crate::model::ExportDeploymentStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Statefile>;

    async fn export_revision_statefile(
        &self,
        req: crate::model::ExportRevisionStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Statefile>;

    async fn import_statefile(
        &self,
        req: crate::model::ImportStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Statefile>;

    async fn delete_statefile(
        &self,
        req: crate::model::DeleteStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn lock_deployment(
        &self,
        req: crate::model::LockDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn unlock_deployment(
        &self,
        req: crate::model::UnlockDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn export_lock_info(
        &self,
        req: crate::model::ExportLockInfoRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LockInfo>;

    async fn create_preview(
        &self,
        req: crate::model::CreatePreviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_preview(
        &self,
        req: crate::model::GetPreviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Preview>;

    async fn list_previews(
        &self,
        req: crate::model::ListPreviewsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPreviewsResponse>;

    async fn delete_preview(
        &self,
        req: crate::model::DeletePreviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn export_preview_result(
        &self,
        req: crate::model::ExportPreviewResultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ExportPreviewResultResponse>;

    async fn list_terraform_versions(
        &self,
        req: crate::model::ListTerraformVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTerraformVersionsResponse>;

    async fn get_terraform_version(
        &self,
        req: crate::model::GetTerraformVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TerraformVersion>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::Config] also implement [Config].
#[async_trait::async_trait]
impl<T: crate::stubs::Config> Config for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDeploymentsResponse> {
        T::list_deployments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Deployment> {
        T::get_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_deployment(
        &self,
        req: crate::model::UpdateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_revisions(
        &self,
        req: crate::model::ListRevisionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListRevisionsResponse> {
        T::list_revisions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_revision(
        &self,
        req: crate::model::GetRevisionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Revision> {
        T::get_revision(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_resource(
        &self,
        req: crate::model::GetResourceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Resource> {
        T::get_resource(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_resources(
        &self,
        req: crate::model::ListResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListResourcesResponse> {
        T::list_resources(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_deployment_statefile(
        &self,
        req: crate::model::ExportDeploymentStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Statefile> {
        T::export_deployment_statefile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_revision_statefile(
        &self,
        req: crate::model::ExportRevisionStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Statefile> {
        T::export_revision_statefile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn import_statefile(
        &self,
        req: crate::model::ImportStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Statefile> {
        T::import_statefile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_statefile(
        &self,
        req: crate::model::DeleteStatefileRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_statefile(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn lock_deployment(
        &self,
        req: crate::model::LockDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::lock_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn unlock_deployment(
        &self,
        req: crate::model::UnlockDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::unlock_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_lock_info(
        &self,
        req: crate::model::ExportLockInfoRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LockInfo> {
        T::export_lock_info(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_preview(
        &self,
        req: crate::model::CreatePreviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_preview(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_preview(
        &self,
        req: crate::model::GetPreviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Preview> {
        T::get_preview(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_previews(
        &self,
        req: crate::model::ListPreviewsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListPreviewsResponse> {
        T::list_previews(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_preview(
        &self,
        req: crate::model::DeletePreviewRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_preview(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn export_preview_result(
        &self,
        req: crate::model::ExportPreviewResultRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ExportPreviewResultResponse> {
        T::export_preview_result(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_terraform_versions(
        &self,
        req: crate::model::ListTerraformVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListTerraformVersionsResponse> {
        T::list_terraform_versions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_terraform_version(
        &self,
        req: crate::model::GetTerraformVersionRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::TerraformVersion> {
        T::get_terraform_version(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
