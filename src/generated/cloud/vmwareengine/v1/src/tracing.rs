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

/// Implements a [VmwareEngine](super::stub::VmwareEngine) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct VmwareEngine<T>
where
    T: super::stub::VmwareEngine + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> VmwareEngine<T>
where
    T: super::stub::VmwareEngine + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::VmwareEngine for VmwareEngine<T>
where
    T: super::stub::VmwareEngine + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_private_clouds(
        &self,
        req: crate::model::ListPrivateCloudsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPrivateCloudsResponse> {
        self.inner.list_private_clouds(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_private_cloud(
        &self,
        req: crate::model::GetPrivateCloudRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PrivateCloud> {
        self.inner.get_private_cloud(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_private_cloud(
        &self,
        req: crate::model::CreatePrivateCloudRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_private_cloud(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_private_cloud(
        &self,
        req: crate::model::UpdatePrivateCloudRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_private_cloud(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_private_cloud(
        &self,
        req: crate::model::DeletePrivateCloudRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_private_cloud(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undelete_private_cloud(
        &self,
        req: crate::model::UndeletePrivateCloudRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.undelete_private_cloud(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListClustersResponse> {
        self.inner.list_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Cluster> {
        self.inner.get_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_cluster(
        &self,
        req: crate::model::UpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_nodes(
        &self,
        req: crate::model::ListNodesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNodesResponse> {
        self.inner.list_nodes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_node(
        &self,
        req: crate::model::GetNodeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Node> {
        self.inner.get_node(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_external_addresses(
        &self,
        req: crate::model::ListExternalAddressesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListExternalAddressesResponse> {
        self.inner.list_external_addresses(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_network_policy_external_addresses(
        &self,
        req: crate::model::FetchNetworkPolicyExternalAddressesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FetchNetworkPolicyExternalAddressesResponse> {
        self.inner
            .fetch_network_policy_external_addresses(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_external_address(
        &self,
        req: crate::model::GetExternalAddressRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExternalAddress> {
        self.inner.get_external_address(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_external_address(
        &self,
        req: crate::model::CreateExternalAddressRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_external_address(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_external_address(
        &self,
        req: crate::model::UpdateExternalAddressRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_external_address(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_external_address(
        &self,
        req: crate::model::DeleteExternalAddressRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_external_address(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_subnets(
        &self,
        req: crate::model::ListSubnetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSubnetsResponse> {
        self.inner.list_subnets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_subnet(
        &self,
        req: crate::model::GetSubnetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Subnet> {
        self.inner.get_subnet(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_subnet(
        &self,
        req: crate::model::UpdateSubnetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_subnet(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_external_access_rules(
        &self,
        req: crate::model::ListExternalAccessRulesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListExternalAccessRulesResponse> {
        self.inner.list_external_access_rules(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_external_access_rule(
        &self,
        req: crate::model::GetExternalAccessRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExternalAccessRule> {
        self.inner.get_external_access_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_external_access_rule(
        &self,
        req: crate::model::CreateExternalAccessRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_external_access_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_external_access_rule(
        &self,
        req: crate::model::UpdateExternalAccessRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_external_access_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_external_access_rule(
        &self,
        req: crate::model::DeleteExternalAccessRuleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_external_access_rule(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_logging_servers(
        &self,
        req: crate::model::ListLoggingServersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLoggingServersResponse> {
        self.inner.list_logging_servers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_logging_server(
        &self,
        req: crate::model::GetLoggingServerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LoggingServer> {
        self.inner.get_logging_server(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_logging_server(
        &self,
        req: crate::model::CreateLoggingServerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_logging_server(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_logging_server(
        &self,
        req: crate::model::UpdateLoggingServerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_logging_server(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_logging_server(
        &self,
        req: crate::model::DeleteLoggingServerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_logging_server(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_node_types(
        &self,
        req: crate::model::ListNodeTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNodeTypesResponse> {
        self.inner.list_node_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_node_type(
        &self,
        req: crate::model::GetNodeTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::NodeType> {
        self.inner.get_node_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn show_nsx_credentials(
        &self,
        req: crate::model::ShowNsxCredentialsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Credentials> {
        self.inner.show_nsx_credentials(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn show_vcenter_credentials(
        &self,
        req: crate::model::ShowVcenterCredentialsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Credentials> {
        self.inner.show_vcenter_credentials(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reset_nsx_credentials(
        &self,
        req: crate::model::ResetNsxCredentialsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.reset_nsx_credentials(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn reset_vcenter_credentials(
        &self,
        req: crate::model::ResetVcenterCredentialsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.reset_vcenter_credentials(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dns_forwarding(
        &self,
        req: crate::model::GetDnsForwardingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DnsForwarding> {
        self.inner.get_dns_forwarding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_dns_forwarding(
        &self,
        req: crate::model::UpdateDnsForwardingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_dns_forwarding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_network_peering(
        &self,
        req: crate::model::GetNetworkPeeringRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::NetworkPeering> {
        self.inner.get_network_peering(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_network_peerings(
        &self,
        req: crate::model::ListNetworkPeeringsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNetworkPeeringsResponse> {
        self.inner.list_network_peerings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_network_peering(
        &self,
        req: crate::model::CreateNetworkPeeringRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_network_peering(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_network_peering(
        &self,
        req: crate::model::DeleteNetworkPeeringRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_network_peering(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_network_peering(
        &self,
        req: crate::model::UpdateNetworkPeeringRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_network_peering(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_peering_routes(
        &self,
        req: crate::model::ListPeeringRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPeeringRoutesResponse> {
        self.inner.list_peering_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_hcx_activation_key(
        &self,
        req: crate::model::CreateHcxActivationKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_hcx_activation_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_hcx_activation_keys(
        &self,
        req: crate::model::ListHcxActivationKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListHcxActivationKeysResponse> {
        self.inner.list_hcx_activation_keys(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_hcx_activation_key(
        &self,
        req: crate::model::GetHcxActivationKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HcxActivationKey> {
        self.inner.get_hcx_activation_key(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_network_policy(
        &self,
        req: crate::model::GetNetworkPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::NetworkPolicy> {
        self.inner.get_network_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_network_policies(
        &self,
        req: crate::model::ListNetworkPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNetworkPoliciesResponse> {
        self.inner.list_network_policies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_network_policy(
        &self,
        req: crate::model::CreateNetworkPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_network_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_network_policy(
        &self,
        req: crate::model::UpdateNetworkPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_network_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_network_policy(
        &self,
        req: crate::model::DeleteNetworkPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_network_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_management_dns_zone_bindings(
        &self,
        req: crate::model::ListManagementDnsZoneBindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListManagementDnsZoneBindingsResponse> {
        self.inner
            .list_management_dns_zone_bindings(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_management_dns_zone_binding(
        &self,
        req: crate::model::GetManagementDnsZoneBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ManagementDnsZoneBinding> {
        self.inner
            .get_management_dns_zone_binding(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn create_management_dns_zone_binding(
        &self,
        req: crate::model::CreateManagementDnsZoneBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .create_management_dns_zone_binding(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn update_management_dns_zone_binding(
        &self,
        req: crate::model::UpdateManagementDnsZoneBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .update_management_dns_zone_binding(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn delete_management_dns_zone_binding(
        &self,
        req: crate::model::DeleteManagementDnsZoneBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .delete_management_dns_zone_binding(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn repair_management_dns_zone_binding(
        &self,
        req: crate::model::RepairManagementDnsZoneBindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .repair_management_dns_zone_binding(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn create_vmware_engine_network(
        &self,
        req: crate::model::CreateVmwareEngineNetworkRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_vmware_engine_network(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_vmware_engine_network(
        &self,
        req: crate::model::UpdateVmwareEngineNetworkRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_vmware_engine_network(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_vmware_engine_network(
        &self,
        req: crate::model::DeleteVmwareEngineNetworkRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_vmware_engine_network(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_vmware_engine_network(
        &self,
        req: crate::model::GetVmwareEngineNetworkRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::VmwareEngineNetwork> {
        self.inner.get_vmware_engine_network(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_vmware_engine_networks(
        &self,
        req: crate::model::ListVmwareEngineNetworksRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListVmwareEngineNetworksResponse> {
        self.inner.list_vmware_engine_networks(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_private_connection(
        &self,
        req: crate::model::CreatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_private_connection(
        &self,
        req: crate::model::GetPrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PrivateConnection> {
        self.inner.get_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_private_connections(
        &self,
        req: crate::model::ListPrivateConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPrivateConnectionsResponse> {
        self.inner.list_private_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_private_connection(
        &self,
        req: crate::model::UpdatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_private_connection(
        &self,
        req: crate::model::DeletePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_private_connection_peering_routes(
        &self,
        req: crate::model::ListPrivateConnectionPeeringRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPrivateConnectionPeeringRoutesResponse> {
        self.inner
            .list_private_connection_peering_routes(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn grant_dns_bind_permission(
        &self,
        req: crate::model::GrantDnsBindPermissionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.grant_dns_bind_permission(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dns_bind_permission(
        &self,
        req: crate::model::GetDnsBindPermissionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DnsBindPermission> {
        self.inner.get_dns_bind_permission(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn revoke_dns_bind_permission(
        &self,
        req: crate::model::RevokeDnsBindPermissionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.revoke_dns_bind_permission(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
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
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
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
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
