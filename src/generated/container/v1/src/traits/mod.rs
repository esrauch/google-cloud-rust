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

/// Google Kubernetes Engine Cluster Manager v1
///
/// # Mocking
///
/// Application developers may use this trait to mock the container clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait ClusterManager: std::fmt::Debug + Send + Sync {
    /// Lists all clusters owned by a project in either the specified zone or all
    /// zones.
    fn list_clusters(
        &self,
        _req: crate::model::ListClustersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListClustersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListClustersResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the details of a specific cluster.
    fn get_cluster(
        &self,
        _req: crate::model::GetClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Cluster>> + Send {
        std::future::ready::<crate::Result<crate::model::Cluster>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a cluster, consisting of the specified number and type of Google
    /// Compute Engine instances.
    ///
    /// By default, the cluster is created in the project's
    /// [default
    /// network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks).
    ///
    /// One firewall is added for the cluster. After cluster creation,
    /// the Kubelet creates routes for each node to allow the containers
    /// on that node to communicate with all other instances in the
    /// cluster.
    ///
    /// Finally, an entry is added to the project's global metadata indicating
    /// which CIDR range the cluster is using.
    fn create_cluster(
        &self,
        _req: crate::model::CreateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates the settings of a specific cluster.
    fn update_cluster(
        &self,
        _req: crate::model::UpdateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates the version and/or image type for the specified node pool.
    fn update_node_pool(
        &self,
        _req: crate::model::UpdateNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the autoscaling settings for the specified node pool.
    fn set_node_pool_autoscaling(
        &self,
        _req: crate::model::SetNodePoolAutoscalingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the logging service for a specific cluster.
    fn set_logging_service(
        &self,
        _req: crate::model::SetLoggingServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the monitoring service for a specific cluster.
    fn set_monitoring_service(
        &self,
        _req: crate::model::SetMonitoringServiceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the addons for a specific cluster.
    fn set_addons_config(
        &self,
        _req: crate::model::SetAddonsConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the locations for a specific cluster.
    /// Deprecated. Use
    /// [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/update)
    /// instead.
    fn set_locations(
        &self,
        _req: crate::model::SetLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates the master for a specific cluster.
    fn update_master(
        &self,
        _req: crate::model::UpdateMasterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets master auth materials. Currently supports changing the admin password
    /// or a specific cluster, either via password generation or explicitly setting
    /// the password.
    fn set_master_auth(
        &self,
        _req: crate::model::SetMasterAuthRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes the cluster, including the Kubernetes endpoint and all worker
    /// nodes.
    ///
    /// Firewalls and routes that were configured during cluster creation
    /// are also deleted.
    ///
    /// Other Google Compute Engine resources that might be in use by the cluster,
    /// such as load balancer resources, are not deleted if they weren't present
    /// when the cluster was initially created.
    fn delete_cluster(
        &self,
        _req: crate::model::DeleteClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists all operations in a project in a specific zone or all zones.
    fn list_operations(
        &self,
        _req: crate::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListOperationsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Gets the specified operation.
    fn get_operation(
        &self,
        _req: crate::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Cancels the specified operation.
    fn cancel_operation(
        &self,
        _req: crate::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns configuration info about the Google Kubernetes Engine service.
    fn get_server_config(
        &self,
        _req: crate::model::GetServerConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ServerConfig>> + Send {
        std::future::ready::<crate::Result<crate::model::ServerConfig>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the public component of the cluster signing keys in
    /// JSON Web Key format.
    fn get_json_web_keys(
        &self,
        _req: crate::model::GetJSONWebKeysRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::GetJSONWebKeysResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::GetJSONWebKeysResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Lists the node pools for a cluster.
    fn list_node_pools(
        &self,
        _req: crate::model::ListNodePoolsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListNodePoolsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListNodePoolsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Retrieves the requested node pool.
    fn get_node_pool(
        &self,
        _req: crate::model::GetNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::NodePool>> + Send {
        std::future::ready::<crate::Result<crate::model::NodePool>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a node pool for a cluster.
    fn create_node_pool(
        &self,
        _req: crate::model::CreateNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a node pool from a cluster.
    fn delete_node_pool(
        &self,
        _req: crate::model::DeleteNodePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// CompleteNodePoolUpgrade will signal an on-going node pool upgrade to
    /// complete.
    fn complete_node_pool_upgrade(
        &self,
        _req: crate::model::CompleteNodePoolUpgradeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Rolls back a previously Aborted or Failed NodePool upgrade.
    /// This makes no changes if the last upgrade successfully completed.
    fn rollback_node_pool_upgrade(
        &self,
        _req: crate::model::RollbackNodePoolUpgradeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the NodeManagement options for a node pool.
    fn set_node_pool_management(
        &self,
        _req: crate::model::SetNodePoolManagementRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets labels on a cluster.
    fn set_labels(
        &self,
        _req: crate::model::SetLabelsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Enables or disables the ABAC authorization mechanism on a cluster.
    fn set_legacy_abac(
        &self,
        _req: crate::model::SetLegacyAbacRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Starts master IP rotation.
    fn start_ip_rotation(
        &self,
        _req: crate::model::StartIPRotationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Completes master IP rotation.
    fn complete_ip_rotation(
        &self,
        _req: crate::model::CompleteIPRotationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the size for a specific node pool. The new size will be used for all
    /// replicas, including future replicas created by modifying
    /// [NodePool.locations][google.container.v1.NodePool.locations].
    ///
    /// [google.container.v1.NodePool.locations]: crate::model::NodePool::locations
    fn set_node_pool_size(
        &self,
        _req: crate::model::SetNodePoolSizeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Enables or disables Network Policy for a cluster.
    fn set_network_policy(
        &self,
        _req: crate::model::SetNetworkPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the maintenance policy for a cluster.
    fn set_maintenance_policy(
        &self,
        _req: crate::model::SetMaintenancePolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Operation>> + Send {
        std::future::ready::<crate::Result<crate::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists subnetworks that are usable for creating clusters in a project.
    fn list_usable_subnetworks(
        &self,
        _req: crate::model::ListUsableSubnetworksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListUsableSubnetworksResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListUsableSubnetworksResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Checks the cluster compatibility with Autopilot mode, and returns a list of
    /// compatibility issues.
    fn check_autopilot_compatibility(
        &self,
        _req: crate::model::CheckAutopilotCompatibilityRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::CheckAutopilotCompatibilityResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::CheckAutopilotCompatibilityResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }
}
