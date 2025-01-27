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

/// Creates and manages Identity and Access Management (IAM) resources.
///
/// You can use this service to work with all of the following resources:
///
/// * **Service accounts**, which identify an application or a virtual machine
///   (VM) instance rather than a person
/// * **Service account keys**, which service accounts use to authenticate with
///   Google APIs
/// * **IAM policies for service accounts**, which specify the roles that a
///   principal has for the service account
/// * **IAM custom roles**, which help you limit the number of permissions that
///   you grant to principals
///
/// In addition, you can use this service to complete the following tasks, among
/// others:
///
/// * Test whether a service account can use specific permissions
/// * Check which roles you can grant for a specific resource
/// * Lint, or validate, condition expressions in an IAM policy
///
/// When you read data from the IAM API, each read is eventually consistent. In
/// other words, if you write data with the IAM API, then immediately read that
/// data, the read operation might return an older version of the data. To deal
/// with this behavior, your application can retry the request with truncated
/// exponential backoff.
///
/// In contrast, writing data to the IAM API is sequentially consistent. In other
/// words, write operations are always processed in the order in which they were
/// received.
///
/// # Mocking
///
/// Application developers may use this trait to mock the iam clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait Iam: std::fmt::Debug + Send + Sync {
    /// Lists every [ServiceAccount][google.iam.admin.v1.ServiceAccount] that belongs to a specific project.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn list_service_accounts(
        &self,
        _req: crate::model::ListServiceAccountsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListServiceAccountsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListServiceAccountsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Gets a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn get_service_account(
        &self,
        _req: crate::model::GetServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceAccount>> + Send {
        std::future::ready::<crate::Result<crate::model::ServiceAccount>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn create_service_account(
        &self,
        _req: crate::model::CreateServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceAccount>> + Send {
        std::future::ready::<crate::Result<crate::model::ServiceAccount>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// **Note:** We are in the process of deprecating this method. Use
    /// [PatchServiceAccount][google.iam.admin.v1.IAM.PatchServiceAccount] instead.
    ///
    /// Updates a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// You can update only the `display_name` field.
    ///
    /// [google.iam.admin.v1.IAM.PatchServiceAccount]: crate::client::Iam::patch_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn update_service_account(
        &self,
        _req: crate::model::ServiceAccount,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceAccount>> + Send {
        std::future::ready::<crate::Result<crate::model::ServiceAccount>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Patches a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn patch_service_account(
        &self,
        _req: crate::model::PatchServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceAccount>> + Send {
        std::future::ready::<crate::Result<crate::model::ServiceAccount>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// **Warning:** After you delete a service account, you might not be able to
    /// undelete it. If you know that you need to re-enable the service account in
    /// the future, use [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount] instead.
    ///
    /// If you delete a service account, IAM permanently removes the service
    /// account 30 days later. Google Cloud cannot recover the service account
    /// after it is permanently removed, even if you file a support request.
    ///
    /// To help avoid unplanned outages, we recommend that you disable the service
    /// account before you delete it. Use [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount] to disable the
    /// service account, then wait at least 24 hours and watch for unintended
    /// consequences. If there are no unintended consequences, you can delete the
    /// service account.
    ///
    /// [google.iam.admin.v1.IAM.DisableServiceAccount]: crate::client::Iam::disable_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn delete_service_account(
        &self,
        _req: crate::model::DeleteServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Restores a deleted [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// **Important:** It is not always possible to restore a deleted service
    /// account. Use this method only as a last resort.
    ///
    /// After you delete a service account, IAM permanently removes the service
    /// account 30 days later. There is no way to restore a deleted service account
    /// that has been permanently removed.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn undelete_service_account(
        &self,
        _req: crate::model::UndeleteServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::UndeleteServiceAccountResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::UndeleteServiceAccountResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Enables a [ServiceAccount][google.iam.admin.v1.ServiceAccount] that was disabled by
    /// [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount].
    ///
    /// If the service account is already enabled, then this method has no effect.
    ///
    /// If the service account was disabled by other means—for example, if Google
    /// disabled the service account because it was compromised—you cannot use this
    /// method to enable the service account.
    ///
    /// [google.iam.admin.v1.IAM.DisableServiceAccount]: crate::client::Iam::disable_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn enable_service_account(
        &self,
        _req: crate::model::EnableServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Disables a [ServiceAccount][google.iam.admin.v1.ServiceAccount] immediately.
    ///
    /// If an application uses the service account to authenticate, that
    /// application can no longer call Google APIs or access Google Cloud
    /// resources. Existing access tokens for the service account are rejected, and
    /// requests for new access tokens will fail.
    ///
    /// To re-enable the service account, use [EnableServiceAccount][google.iam.admin.v1.IAM.EnableServiceAccount]. After you
    /// re-enable the service account, its existing access tokens will be accepted,
    /// and you can request new access tokens.
    ///
    /// To help avoid unplanned outages, we recommend that you disable the service
    /// account before you delete it. Use this method to disable the service
    /// account, then wait at least 24 hours and watch for unintended consequences.
    /// If there are no unintended consequences, you can delete the service account
    /// with [DeleteServiceAccount][google.iam.admin.v1.IAM.DeleteServiceAccount].
    ///
    /// [google.iam.admin.v1.IAM.DeleteServiceAccount]: crate::client::Iam::delete_service_account
    /// [google.iam.admin.v1.IAM.EnableServiceAccount]: crate::client::Iam::enable_service_account
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn disable_service_account(
        &self,
        _req: crate::model::DisableServiceAccountRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Lists every [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey] for a service account.
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    fn list_service_account_keys(
        &self,
        _req: crate::model::ListServiceAccountKeysRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListServiceAccountKeysResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListServiceAccountKeysResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Gets a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey].
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    fn get_service_account_key(
        &self,
        _req: crate::model::GetServiceAccountKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceAccountKey>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ServiceAccountKey>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey].
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    fn create_service_account_key(
        &self,
        _req: crate::model::CreateServiceAccountKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceAccountKey>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ServiceAccountKey>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Uploads the public key portion of a key pair that you manage, and
    /// associates the public key with a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// After you upload the public key, you can use the private key from the key
    /// pair as a service account key.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn upload_service_account_key(
        &self,
        _req: crate::model::UploadServiceAccountKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServiceAccountKey>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ServiceAccountKey>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]. Deleting a service account key does not
    /// revoke short-lived credentials that have been issued based on the service
    /// account key.
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    fn delete_service_account_key(
        &self,
        _req: crate::model::DeleteServiceAccountKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Disable a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]. A disabled service account key can be
    /// re-enabled with [EnableServiceAccountKey][google.iam.admin.v1.IAM.EnableServiceAccountKey].
    ///
    /// [google.iam.admin.v1.IAM.EnableServiceAccountKey]: crate::client::Iam::enable_service_account_key
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    fn disable_service_account_key(
        &self,
        _req: crate::model::DisableServiceAccountKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Enable a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey].
    ///
    /// [google.iam.admin.v1.ServiceAccountKey]: crate::model::ServiceAccountKey
    fn enable_service_account_key(
        &self,
        _req: crate::model::EnableServiceAccountKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// **Note:** This method is deprecated. Use the
    /// [`signBlob`](https://cloud.google.com/iam/help/rest-credentials/v1/projects.serviceAccounts/signBlob)
    /// method in the IAM Service Account Credentials API instead. If you currently
    /// use this method, see the [migration
    /// guide](https://cloud.google.com/iam/help/credentials/migrate-api) for
    /// instructions.
    ///
    /// Signs a blob using the system-managed private key for a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn sign_blob(
        &self,
        _req: crate::model::SignBlobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SignBlobResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SignBlobResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// **Note:** This method is deprecated. Use the
    /// [`signJwt`](https://cloud.google.com/iam/help/rest-credentials/v1/projects.serviceAccounts/signJwt)
    /// method in the IAM Service Account Credentials API instead. If you currently
    /// use this method, see the [migration
    /// guide](https://cloud.google.com/iam/help/credentials/migrate-api) for
    /// instructions.
    ///
    /// Signs a JSON Web Token (JWT) using the system-managed private key for a
    /// [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn sign_jwt(
        &self,
        _req: crate::model::SignJwtRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SignJwtResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::SignJwtResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the IAM policy that is attached to a [ServiceAccount][google.iam.admin.v1.ServiceAccount]. This IAM
    /// policy specifies which principals have access to the service account.
    ///
    /// This method does not tell you whether the service account has been granted
    /// any roles on other resources. To check whether a service account has role
    /// grants on a resource, use the `getIamPolicy` method for that resource. For
    /// example, to view the role grants for a project, call the Resource Manager
    /// API's
    /// [`projects.getIamPolicy`](https://cloud.google.com/resource-manager/reference/rest/v1/projects/getIamPolicy)
    /// method.
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the IAM policy that is attached to a [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// Use this method to grant or revoke access to the service account. For
    /// example, you could grant a principal the ability to impersonate the service
    /// account.
    ///
    /// This method does not enable the service account to access other resources.
    /// To grant roles to a service account on a resource, follow these steps:
    ///
    /// . Call the resource's `getIamPolicy` method to get its current IAM policy.
    /// . Edit the policy so that it binds the service account to an IAM role for
    ///   the resource.
    /// . Call the resource's `setIamPolicy` method to update its IAM policy.
    ///
    /// For detailed instructions, see
    /// [Manage access to project, folders, and
    /// organizations](https://cloud.google.com/iam/help/service-accounts/granting-access-to-service-accounts)
    /// or [Manage access to other
    /// resources](https://cloud.google.com/iam/help/access/manage-other-resources).
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Tests whether the caller has the specified permissions on a
    /// [ServiceAccount][google.iam.admin.v1.ServiceAccount].
    ///
    /// [google.iam.admin.v1.ServiceAccount]: crate::model::ServiceAccount
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

    /// Lists roles that can be granted on a Google Cloud resource. A role is
    /// grantable if the IAM policy for the resource can contain bindings to the
    /// role.
    fn query_grantable_roles(
        &self,
        _req: crate::model::QueryGrantableRolesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::QueryGrantableRolesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::QueryGrantableRolesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Lists every predefined [Role][google.iam.admin.v1.Role] that IAM supports, or every custom role
    /// that is defined for an organization or project.
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    fn list_roles(
        &self,
        _req: crate::model::ListRolesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRolesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRolesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the definition of a [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    fn get_role(
        &self,
        _req: crate::model::GetRoleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Role>> + Send {
        std::future::ready::<crate::Result<crate::model::Role>>(Err(Error::other("unimplemented")))
    }

    /// Creates a new custom [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    fn create_role(
        &self,
        _req: crate::model::CreateRoleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Role>> + Send {
        std::future::ready::<crate::Result<crate::model::Role>>(Err(Error::other("unimplemented")))
    }

    /// Updates the definition of a custom [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    fn update_role(
        &self,
        _req: crate::model::UpdateRoleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Role>> + Send {
        std::future::ready::<crate::Result<crate::model::Role>>(Err(Error::other("unimplemented")))
    }

    /// Deletes a custom [Role][google.iam.admin.v1.Role].
    ///
    /// When you delete a custom role, the following changes occur immediately:
    ///
    /// * You cannot bind a principal to the custom role in an IAM
    ///   [Policy][google.iam.v1.Policy].
    /// * Existing bindings to the custom role are not changed, but they have no
    ///   effect.
    /// * By default, the response from [ListRoles][google.iam.admin.v1.IAM.ListRoles] does not include the custom
    ///   role.
    ///
    /// You have 7 days to undelete the custom role. After 7 days, the following
    /// changes occur:
    ///
    /// * The custom role is permanently deleted and cannot be recovered.
    /// * If an IAM policy contains a binding to the custom role, the binding is
    ///   permanently removed.
    ///
    /// [google.iam.admin.v1.IAM.ListRoles]: crate::client::Iam::list_roles
    /// [google.iam.admin.v1.Role]: crate::model::Role
    /// [google.iam.v1.Policy]: iam_v1::model::Policy
    fn delete_role(
        &self,
        _req: crate::model::DeleteRoleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Role>> + Send {
        std::future::ready::<crate::Result<crate::model::Role>>(Err(Error::other("unimplemented")))
    }

    /// Undeletes a custom [Role][google.iam.admin.v1.Role].
    ///
    /// [google.iam.admin.v1.Role]: crate::model::Role
    fn undelete_role(
        &self,
        _req: crate::model::UndeleteRoleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Role>> + Send {
        std::future::ready::<crate::Result<crate::model::Role>>(Err(Error::other("unimplemented")))
    }

    /// Lists every permission that you can test on a resource. A permission is
    /// testable if you can check whether a principal has that permission on the
    /// resource.
    fn query_testable_permissions(
        &self,
        _req: crate::model::QueryTestablePermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::QueryTestablePermissionsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::QueryTestablePermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Returns a list of services that allow you to opt into audit logs that are
    /// not generated by default.
    ///
    /// To learn more about audit logs, see the [Logging
    /// documentation](https://cloud.google.com/logging/docs/audit).
    fn query_auditable_services(
        &self,
        _req: crate::model::QueryAuditableServicesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::QueryAuditableServicesResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::QueryAuditableServicesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Lints, or validates, an IAM policy. Currently checks the
    /// [google.iam.v1.Binding.condition][google.iam.v1.Binding.condition] field, which contains a condition
    /// expression for a role binding.
    ///
    /// Successful calls to this method always return an HTTP `200 OK` status code,
    /// even if the linter detects an issue in the IAM policy.
    ///
    /// [google.iam.v1.Binding.condition]: iam_v1::model::Binding::condition
    fn lint_policy(
        &self,
        _req: crate::model::LintPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::LintPolicyResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::LintPolicyResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
