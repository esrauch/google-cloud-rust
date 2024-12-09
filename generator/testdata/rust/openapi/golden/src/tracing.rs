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

/// Implement a [SecretManagerService](crate::client::) decorator
/// for tracing and logging.
#[derive(Clone, Debug)]
pub struct SecretManagerService<T>
where T: crate::traits::SecretManagerService + std::fmt::Debug + Send + Sync {
    inner: T,
}

impl<T> SecretManagerService<T>
where T: crate::traits::SecretManagerService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::traits::SecretManagerService for SecretManagerService<T>
where T: crate::traits::SecretManagerService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn list_locations(&self, req: crate::model::ListLocationsRequest) -> Result<crate::model::ListLocationsResponse> {
        self.inner.list_locations(req).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(&self, req: crate::model::GetLocationRequest) -> Result<crate::model::Location> {
        self.inner.get_location(req).await
    }

    #[tracing::instrument(ret)]
    async fn list_secrets(&self, req: crate::model::ListSecretsRequest) -> Result<crate::model::ListSecretsResponse> {
        self.inner.list_secrets(req).await
    }

    #[tracing::instrument(ret)]
    async fn create_secret(&self, req: crate::model::CreateSecretRequest) -> Result<crate::model::Secret> {
        self.inner.create_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn list_secrets_by_project_and_location(&self, req: crate::model::ListSecretsByProjectAndLocationRequest) -> Result<crate::model::ListSecretsResponse> {
        self.inner.list_secrets_by_project_and_location(req).await
    }

    #[tracing::instrument(ret)]
    async fn create_secret_by_project_and_location(&self, req: crate::model::CreateSecretByProjectAndLocationRequest) -> Result<crate::model::Secret> {
        self.inner.create_secret_by_project_and_location(req).await
    }

    #[tracing::instrument(ret)]
    async fn add_secret_version(&self, req: crate::model::AddSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.add_secret_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn add_secret_version_by_project_and_location_and_secret(&self, req: crate::model::AddSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.add_secret_version_by_project_and_location_and_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret(&self, req: crate::model::GetSecretRequest) -> Result<crate::model::Secret> {
        self.inner.get_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn delete_secret(&self, req: crate::model::DeleteSecretRequest) -> Result<crate::model::Empty> {
        self.inner.delete_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn update_secret(&self, req: crate::model::UpdateSecretRequest) -> Result<crate::model::Secret> {
        self.inner.update_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret_by_project_and_location_and_secret(&self, req: crate::model::GetSecretByProjectAndLocationAndSecretRequest) -> Result<crate::model::Secret> {
        self.inner.get_secret_by_project_and_location_and_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn delete_secret_by_project_and_location_and_secret(&self, req: crate::model::DeleteSecretByProjectAndLocationAndSecretRequest) -> Result<crate::model::Empty> {
        self.inner.delete_secret_by_project_and_location_and_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn update_secret_by_project_and_location_and_secret(&self, req: crate::model::UpdateSecretByProjectAndLocationAndSecretRequest) -> Result<crate::model::Secret> {
        self.inner.update_secret_by_project_and_location_and_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn list_secret_versions(&self, req: crate::model::ListSecretVersionsRequest) -> Result<crate::model::ListSecretVersionsResponse> {
        self.inner.list_secret_versions(req).await
    }

    #[tracing::instrument(ret)]
    async fn list_secret_versions_by_project_and_location_and_secret(&self, req: crate::model::ListSecretVersionsByProjectAndLocationAndSecretRequest) -> Result<crate::model::ListSecretVersionsResponse> {
        self.inner.list_secret_versions_by_project_and_location_and_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret_version(&self, req: crate::model::GetSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.get_secret_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret_version_by_project_and_location_and_secret_and_version(&self, req: crate::model::GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.get_secret_version_by_project_and_location_and_secret_and_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn access_secret_version(&self, req: crate::model::AccessSecretVersionRequest) -> Result<crate::model::AccessSecretVersionResponse> {
        self.inner.access_secret_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn access_secret_version_by_project_and_location_and_secret_and_version(&self, req: crate::model::AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest) -> Result<crate::model::AccessSecretVersionResponse> {
        self.inner.access_secret_version_by_project_and_location_and_secret_and_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn disable_secret_version(&self, req: crate::model::DisableSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.disable_secret_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn disable_secret_version_by_project_and_location_and_secret_and_version(&self, req: crate::model::DisableSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.disable_secret_version_by_project_and_location_and_secret_and_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn enable_secret_version(&self, req: crate::model::EnableSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.enable_secret_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn enable_secret_version_by_project_and_location_and_secret_and_version(&self, req: crate::model::EnableSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.enable_secret_version_by_project_and_location_and_secret_and_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn destroy_secret_version(&self, req: crate::model::DestroySecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.destroy_secret_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn destroy_secret_version_by_project_and_location_and_secret_and_version(&self, req: crate::model::DestroySecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.destroy_secret_version_by_project_and_location_and_secret_and_version(req).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(&self, req: crate::model::SetIamPolicyRequest) -> Result<crate::model::Policy> {
        self.inner.set_iam_policy(req).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy_by_project_and_location_and_secret(&self, req: crate::model::SetIamPolicyRequest) -> Result<crate::model::Policy> {
        self.inner.set_iam_policy_by_project_and_location_and_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(&self, req: crate::model::GetIamPolicyRequest) -> Result<crate::model::Policy> {
        self.inner.get_iam_policy(req).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy_by_project_and_location_and_secret(&self, req: crate::model::GetIamPolicyByProjectAndLocationAndSecretRequest) -> Result<crate::model::Policy> {
        self.inner.get_iam_policy_by_project_and_location_and_secret(req).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(&self, req: crate::model::TestIamPermissionsRequest) -> Result<crate::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions_by_project_and_location_and_secret(&self, req: crate::model::TestIamPermissionsRequest) -> Result<crate::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions_by_project_and_location_and_secret(req).await
    }

}

