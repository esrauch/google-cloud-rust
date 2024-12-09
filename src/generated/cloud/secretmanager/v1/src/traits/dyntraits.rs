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

/// A dyn-compatible, crate-private version of `SecretManagerService`.
#[async_trait::async_trait]
pub trait SecretManagerService: std::fmt::Debug + Send + Sync {
    /// Lists [Secrets][google.cloud.secretmanager.v1.Secret].
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
    ) -> crate::Result<crate::model::ListSecretsResponse>;

    /// Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
    ) -> crate::Result<crate::model::Secret>;

    /// Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
    /// containing secret data and attaches it to an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion>;

    /// Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret].
    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
    ) -> crate::Result<crate::model::Secret>;

    /// Updates metadata of an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
    ) -> crate::Result<crate::model::Secret>;

    /// Deletes a [Secret][google.cloud.secretmanager.v1.Secret].
    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
    ) -> crate::Result<wkt::Empty>;

    /// Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This
    /// call does not return secret data.
    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
    ) -> crate::Result<crate::model::ListSecretVersionsResponse>;

    /// Gets metadata for a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion>;

    /// Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    /// This call returns the secret data.
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
    ) -> crate::Result<crate::model::AccessSecretVersionResponse>;

    /// Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED].
    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion>;

    /// Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED].
    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion>;

    /// Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED]
    /// and irrevocably destroys the secret data.
    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion>;

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced
    /// according to the policy set on the associated
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
    ) -> crate::Result<iam_v1::model::Policy>;

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
    ) -> crate::Result<iam_v1::model::Policy>;

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;
}

/// All implementations of [crate::traits::SecretManagerService] also implement [SecretManagerService].
#[async_trait::async_trait]
impl<T: crate::traits::SecretManagerService> SecretManagerService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
    ) -> crate::Result<crate::model::ListSecretsResponse> {
        let response = T::list_secrets(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
    ) -> crate::Result<crate::model::Secret> {
        let response = T::create_secret(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion> {
        let response = T::add_secret_version(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
    ) -> crate::Result<crate::model::Secret> {
        let response = T::get_secret(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
    ) -> crate::Result<crate::model::Secret> {
        let response = T::update_secret(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
    ) -> crate::Result<wkt::Empty> {
        let response = T::delete_secret(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
    ) -> crate::Result<crate::model::ListSecretVersionsResponse> {
        let response = T::list_secret_versions(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion> {
        let response = T::get_secret_version(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
    ) -> crate::Result<crate::model::AccessSecretVersionResponse> {
        let response = T::access_secret_version(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion> {
        let response = T::disable_secret_version(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion> {
        let response = T::enable_secret_version(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
    ) -> crate::Result<crate::model::SecretVersion> {
        let response = T::destroy_secret_version(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
    ) -> crate::Result<iam_v1::model::Policy> {
        let response = T::set_iam_policy(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
    ) -> crate::Result<iam_v1::model::Policy> {
        let response = T::get_iam_policy(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        let response = T::test_iam_permissions(self, req).await?;
        Ok(response)
    }
}

/// A dyn-compatible, crate-private version of `Locations`.
#[async_trait::async_trait]
pub trait Locations: std::fmt::Debug + Send + Sync {
    /// Lists information about the supported locations for this service.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    /// Gets information about a location.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
    ) -> crate::Result<location::model::Location>;
}

/// All implementations of [crate::traits::Locations] also implement [Locations].
#[async_trait::async_trait]
impl<T: crate::traits::Locations> Locations for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        let response = T::list_locations(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
    ) -> crate::Result<location::model::Location> {
        let response = T::get_location(self, req).await?;
        Ok(response)
    }
}
