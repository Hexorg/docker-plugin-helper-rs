use std::collections::HashMap;

use serde::{Serialize, ser::SerializeMap};

const DEFAULT_CONTENT_TYPE:&str = "application/vnd.docker.plugins.v1.1+json";
const ACTIVATE_URL:&str = "/Plugin.Activate";

const MANIFEST:&str = "{\"Implements\": [\"NetworkDriver\"]}";
// LocalScope is the correct scope response for a local scope driver
const LocalScope:&str = "local";
// GlobalScope is the correct scope response for a global scope driver
const GlobalScope:&str = "global";

const capabilitiesPath   :&str = "/NetworkDriver.GetCapabilities";
const allocateNetworkPath:&str = "/NetworkDriver.AllocateNetwork";
const freeNetworkPath    :&str = "/NetworkDriver.FreeNetwork";
const createNetworkPath  :&str = "/NetworkDriver.CreateNetwork";
const deleteNetworkPath  :&str = "/NetworkDriver.DeleteNetwork";
const createEndpointPath :&str = "/NetworkDriver.CreateEndpoint";
const endpointInfoPath   :&str = "/NetworkDriver.EndpointOperInfo";
const deleteEndpointPath :&str = "/NetworkDriver.DeleteEndpoint";
const joinPath           :&str = "/NetworkDriver.Join";
const leavePath          :&str = "/NetworkDriver.Leave";
const discoverNewPath    :&str = "/NetworkDriver.DiscoverNew";
const discoverDeletePath :&str = "/NetworkDriver.DiscoverDelete";
const programExtConnPath :&str = "/NetworkDriver.ProgramExternalConnectivity";
const revokeExtConnPath  :&str = "/NetworkDriver.RevokeExternalConnectivity";


pub struct CapabilitiesResponse<'a> {
    scope: &'a str,
	connectivity_scope: &'a str,
}

impl<'a> Serialize for CapabilitiesResponse<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("Scope", self.scope)?;
        map.serialize_entry("ConnectivityScope", self.connectivity_scope)?;
        map.end()
    }
}

pub struct AllocateNetworkResponse<'a>{options:HashMap<&'a str, &'a str>}
impl<'a> Serialize for AllocateNetworkResponse<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Options", &self.options)?;
        map.end()
    }
}


pub struct CreateEndpointResponse {}
pub struct InfoResponse {}
pub struct JoinResponse{}
pub struct IPAMData{}
pub struct ErrorResponse {}

pub struct EndpointInterface<'a> {
	address:&'a str,
	address_ipv6:&'a str,
	address_mac:&'a str
}
impl<'a> Serialize for EndpointInterface<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("Address", self.address)?;
        map.serialize_entry("AddressIPv6", self.address_ipv6)?;
        map.serialize_entry("MacAddress", self.address_mac)?;
        map.end()
    }
}
struct InterfaceName {
	src_name: String,
	dst_prefix: String,
}

trait Network {
    fn get_capabilities(&self) -> CapabilitiesResponse;
    fn create_network(&self, network_id:&str, options: HashMap<&str, &str>, ipv4_data: &[IPAMData], ipv6_data: &[IPAMData]);
	fn allocate_network(&self, network_id:&str, options: HashMap<&str, &str>, ipv4_data: &[IPAMData], ipv6_data: &[IPAMData]) -> Result<AllocateNetworkResponse, ErrorResponse>;
	fn delete_network(&self, network_id:&str);
	fn free_network(&self, network_id:&str);
	fn create_endpoint(&self, network_id:&str, endpoint_id: &str, interface: EndpointInterface, options: HashMap<&str, &str>) -> Result<CreateEndpointResponse, ErrorResponse>;
	fn delete_endpoint(&self, network_id:&str, endpoint_id: &str);
	fn endpoint_info(&self, network_id:&str, endpoint_id: &str) -> Result<InfoResponse, ErrorResponse>;
	fn join(&self, network_id:&str, endpoint_id: &str) -> Result<JoinResponse, ErrorResponse>;
	fn leave(&self, network_id:&str, endpoint_id: &str);
	fn discover_new(&self, discovery_type: i32, discovery_data:&str);
	fn discover_delete(&self, discovery_type: i32, discovery_data:&str);
	fn program_external_connectivity(&self, network_id:&str, endpoint_id: &str, options: HashMap<&str, &str>);
	fn revoke_external_connectivity(&self, network_id:&str, endpoint_id: &str);
}

pub fn post<N:Network>(driver:&N, url:&str, data:&str) -> String {
    const EMPTY:&str = "{}";
    const MANIFEST:&str = "{\"Implements\": [\"NetworkDriver\"]}";
    let response = match url {
        "/Plugin.Activate" => String::from(MANIFEST),
        "/NetworkDriver.GetCapabilities" => serde_json::to_string(&driver.get_capabilities()).unwrap(),
        "/NetworkDriver.AllocateNetwork" => {driver.allocate_network(network_id, options, ipv4_data, ipv6_data); String::from(EMPTY)},
        "/NetworkDriver.FreeNetwork" => 
        "/NetworkDriver.CreateNetwork" => 
        "/NetworkDriver.DeleteNetwork" => 
        "/NetworkDriver.CreateEndpoint" => 
        "/NetworkDriver.EndpointOperInfo" => 
        "/NetworkDriver.DeleteEndpoint" => 
        "/NetworkDriver.Join" => 
        "/NetworkDriver.Leave" => 
        "/NetworkDriver.DiscoverNew" => 
        "/NetworkDriver.DiscoverDelete" => 
        "/NetworkDriver.ProgramExternalConnectivity" => 
        "/NetworkDriver.RevokeExternalConnectivity" => 
        _ => (),
    };
    String::new()
}
