pub mod network;

use network::api::{Network, CreateNetworkRequest, AllocateNetworkRequest, NetworkRequest, CreateEndpointRequest, EndpointRequest, DiscoverRequest, ExternalConnectivityRequest, JoinRequest};

struct Test;
impl Network for Test {
    fn get_capabilities(&self) -> network::api::CapabilitiesResponse {
        todo!()
    }

    fn create_network(&self, request: &CreateNetworkRequest) {
        println!("{:?}", request);
    }

    fn allocate_network(&self, request: &AllocateNetworkRequest) -> Result<network::api::AllocateNetworkResponse, network::api::ErrorResponse> {
        todo!()
    }

    fn delete_network(&self, request: &NetworkRequest) {
        todo!()
    }

    fn free_network(&self, request: &NetworkRequest) {
        todo!()
    }

    fn create_endpoint(&self, request: &CreateEndpointRequest) -> Result<network::api::CreateEndpointResponse, network::api::ErrorResponse> {
        todo!()
    }

    fn delete_endpoint(&self, request: &EndpointRequest) {
        todo!()
    }

    fn endpoint_info(&self, request: &EndpointRequest) -> Result<network::api::InfoResponse, network::api::ErrorResponse> {
        todo!()
    }

    fn join(&self, request: &JoinRequest) -> Result<network::api::JoinResponse, network::api::ErrorResponse> {
        todo!()
    }

    fn leave(&self, request: &EndpointRequest) {
        todo!()
    }

    fn discover_new(&self, request: &DiscoverRequest) {
        todo!()
    }

    fn discover_delete(&self, request: &DiscoverRequest) {
        todo!()
    }

    fn program_external_connectivity(&self, request: &ExternalConnectivityRequest) {
        todo!()
    }

    fn revoke_external_connectivity(&self, request: &EndpointRequest) {
        todo!()
    }
}

fn main() {
    let t = Test{};
    println!("{}", network::api::post(&t, "/NetworkDriver.CreateNetwork", "{\"NetworkId\": \"poop\", \"Options\":{}, \"IPv4Data\":[], \"IPv6Data\":[]}"));
}
