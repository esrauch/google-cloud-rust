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

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Google Workspace add-ons API.
///
/// # Service Description
///
/// A service for managing Google Workspace add-ons deployments.
///
/// A Google Workspace add-on is a third-party embedded component that can be
/// installed in Google Workspace Applications like Gmail, Calendar, Drive, and
/// the Google Docs, Sheets, and Slides editors. Google Workspace add-ons can
/// display UI cards, receive contextual information from the host application,
/// and perform actions in the host application (See:
/// <https://developers.google.com/gsuite/add-ons/overview> for more information).
///
/// A Google Workspace add-on deployment resource specifies metadata about the
/// add-on, including a specification of the entry points in the host application
/// that trigger add-on executions (see:
/// <https://developers.google.com/gsuite/add-ons/concepts/gsuite-manifests>).
/// Add-on deployments defined via the Google Workspace add-ons API define their
/// entrypoints using HTTPS URLs (See:
/// <https://developers.google.com/gsuite/add-ons/guides/alternate-runtimes>),
///
/// A Google Workspace add-on deployment can be installed in developer mode,
/// which allows an add-on developer to test the experience an end-user would see
/// when installing and running the add-on in their G Suite applications.  When
/// running in developer mode, more detailed error messages are exposed in the
/// add-on UI to aid in debugging.
///
/// A Google Workspace add-on deployment can be published to Google Workspace
/// Marketplace, which allows other Google Workspace users to discover and
/// install the add-on.  See:
/// <https://developers.google.com/gsuite/add-ons/how-tos/publish-add-on-overview>
/// for details.
///
/// # Configuration
///
/// `GSuiteAddOns` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `GSuiteAddOns` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `GSuiteAddOns` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct GSuiteAddOns {
    inner: Arc<dyn crate::stubs::dynamic::GSuiteAddOns>,
}

impl GSuiteAddOns {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::stubs::GSuiteAddOns + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::GSuiteAddOns>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::GSuiteAddOns> {
        crate::transport::GSuiteAddOns::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::GSuiteAddOns> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::GSuiteAddOns::new)
    }

    /// Gets the authorization information for deployments in a given project.
    pub fn get_authorization(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::GetAuthorization {
        crate::builders::g_suite_add_ons::GetAuthorization::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a deployment with the specified name and configuration.
    pub fn create_deployment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::CreateDeployment {
        crate::builders::g_suite_add_ons::CreateDeployment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates or replaces a deployment with the specified name.
    pub fn replace_deployment(
        &self,
        deployment: impl Into<crate::model::Deployment>,
    ) -> crate::builders::g_suite_add_ons::ReplaceDeployment {
        crate::builders::g_suite_add_ons::ReplaceDeployment::new(self.inner.clone())
            .set_deployment(deployment.into())
    }

    /// Gets the deployment with the specified name.
    pub fn get_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::GetDeployment {
        crate::builders::g_suite_add_ons::GetDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all deployments in a particular project.
    pub fn list_deployments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::ListDeployments {
        crate::builders::g_suite_add_ons::ListDeployments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes the deployment with the given name.
    pub fn delete_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::DeleteDeployment {
        crate::builders::g_suite_add_ons::DeleteDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Installs a deployment in developer mode.
    /// See:
    /// <https://developers.google.com/gsuite/add-ons/how-tos/testing-gsuite-addons>.
    pub fn install_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::InstallDeployment {
        crate::builders::g_suite_add_ons::InstallDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Uninstalls a developer mode deployment.
    /// See:
    /// <https://developers.google.com/gsuite/add-ons/how-tos/testing-gsuite-addons>.
    pub fn uninstall_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::UninstallDeployment {
        crate::builders::g_suite_add_ons::UninstallDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Fetches the install status of a developer mode deployment.
    pub fn get_install_status(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::g_suite_add_ons::GetInstallStatus {
        crate::builders::g_suite_add_ons::GetInstallStatus::new(self.inner.clone())
            .set_name(name.into())
    }
}
