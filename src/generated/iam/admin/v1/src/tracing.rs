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
use crate::Result;

/// Implements a [Iam](super::stub::Iam) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Iam<T>
where
    T: super::stub::Iam + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Iam<T>
where
    T: super::stub::Iam + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::Iam for Iam<T>
where
    T: super::stub::Iam + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_service_accounts(
        &self,
        req: crate::model::ListServiceAccountsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListServiceAccountsResponse> {
        self.inner.list_service_accounts(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service_account(
        &self,
        req: crate::model::GetServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
        self.inner.get_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service_account(
        &self,
        req: crate::model::CreateServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
        self.inner.create_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_service_account(
        &self,
        req: crate::model::ServiceAccount,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
        self.inner.update_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn patch_service_account(
        &self,
        req: crate::model::PatchServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccount> {
        self.inner.patch_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service_account(
        &self,
        req: crate::model::DeleteServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undelete_service_account(
        &self,
        req: crate::model::UndeleteServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::UndeleteServiceAccountResponse> {
        self.inner.undelete_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_service_account(
        &self,
        req: crate::model::EnableServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.enable_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_service_account(
        &self,
        req: crate::model::DisableServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.disable_service_account(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_service_account_keys(
        &self,
        req: crate::model::ListServiceAccountKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListServiceAccountKeysResponse> {
        self.inner.list_service_account_keys(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service_account_key(
        &self,
        req: crate::model::GetServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccountKey> {
        self.inner.get_service_account_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service_account_key(
        &self,
        req: crate::model::CreateServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccountKey> {
        self.inner.create_service_account_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn upload_service_account_key(
        &self,
        req: crate::model::UploadServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ServiceAccountKey> {
        self.inner.upload_service_account_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service_account_key(
        &self,
        req: crate::model::DeleteServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_service_account_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_service_account_key(
        &self,
        req: crate::model::DisableServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.disable_service_account_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_service_account_key(
        &self,
        req: crate::model::EnableServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.enable_service_account_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn sign_blob(
        &self,
        req: crate::model::SignBlobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SignBlobResponse> {
        self.inner.sign_blob(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn sign_jwt(
        &self,
        req: crate::model::SignJwtRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SignJwtResponse> {
        self.inner.sign_jwt(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn query_grantable_roles(
        &self,
        req: crate::model::QueryGrantableRolesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryGrantableRolesResponse> {
        self.inner.query_grantable_roles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_roles(
        &self,
        req: crate::model::ListRolesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRolesResponse> {
        self.inner.list_roles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_role(
        &self,
        req: crate::model::GetRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
        self.inner.get_role(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_role(
        &self,
        req: crate::model::CreateRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
        self.inner.create_role(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_role(
        &self,
        req: crate::model::UpdateRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
        self.inner.update_role(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_role(
        &self,
        req: crate::model::DeleteRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
        self.inner.delete_role(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undelete_role(
        &self,
        req: crate::model::UndeleteRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Role> {
        self.inner.undelete_role(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn query_testable_permissions(
        &self,
        req: crate::model::QueryTestablePermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryTestablePermissionsResponse> {
        self.inner.query_testable_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn query_auditable_services(
        &self,
        req: crate::model::QueryAuditableServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::QueryAuditableServicesResponse> {
        self.inner.query_auditable_services(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lint_policy(
        &self,
        req: crate::model::LintPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LintPolicyResponse> {
        self.inner.lint_policy(req, options).await
    }
}
