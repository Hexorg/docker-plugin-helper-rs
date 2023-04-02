use std::collections::HashMap;

use serde::{Serialize, Deserialize};

/// Content-type header value that docker expects in your responses.
pub const DEFAULT_CONTENT_TYPE:&str = "application/vnd.docker.plugins.v1.1+json";
const ACTIVATE_URL:&str = "/Plugin.Activate";

const MANIFEST:&str = "{\"Implements\": [\"NetworkDriver\"]}";
const EMPTY: &str = "{}";

/// LocalScope is the correct scope response for a local scope driver
pub const LOCAL_SCOPE:&str = "local";
/// GlobalScope is the correct scope response for a global scope driver
pub const GLOBAL_SCOPE:&str = "global";

const CAPABILITIES_PATH   :&str = "/NetworkDriver.GetCapabilities";
const ALLOCATE_NETWORK_PATH:&str = "/NetworkDriver.AllocateNetwork";
const FREE_NETWORK_PATH    :&str = "/NetworkDriver.FreeNetwork";
const CREATE_NETWORK_PATH  :&str = "/NetworkDriver.CreateNetwork";
const DELETE_NETWORK_PATH  :&str = "/NetworkDriver.DeleteNetwork";
const CREATE_ENDPOINT_PATH :&str = "/NetworkDriver.CreateEndpoint";
const ENDPOINT_INFO_PATH   :&str = "/NetworkDriver.EndpointOperInfo";
const DELETE_ENDPOINT_PATH :&str = "/NetworkDriver.DeleteEndpoint";
const JOIN_PATH           :&str = "/NetworkDriver.Join";
const LEAVE_PATH          :&str = "/NetworkDriver.Leave";
const DISCOVER_NEW_PATH    :&str = "/NetworkDriver.DiscoverNew";
const DISCOVER_DELETE_PATH :&str = "/NetworkDriver.DiscoverDelete";
const PROGRAM_EXT_CONN_PATH :&str = "/NetworkDriver.ProgramExternalConnectivity";
const REVOKE_EXT_CONN_PATH: &str = "/NetworkDriver.RevokeExternalConnectivity";

#[derive(Serialize)]
pub struct CapabilitiesResponse<'a> {
    #[serde(rename = "Scope")]
    pub scope: &'a str,
    #[serde(rename = "ConnectivityScope")]
	pub connectivity_scope: &'a str,
}

#[derive(Serialize)]
pub struct AllocateNetworkResponse<'a> {
    #[serde(rename = "Options")]
    options:HashMap<&'a str, &'a str>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointInterface<'a> {
    #[serde(rename = "Address")]
	pub address:&'a str,
    #[serde(rename = "AddressIPv6")]
	pub address_ipv6:&'a str,
    #[serde(rename = "MacAddress")]
	pub address_mac:&'a str
}

#[derive(Serialize)]
pub struct CreateEndpointResponse<'a> {
    #[serde(rename = "Interface")]
    interface: EndpointInterface<'a>
}

#[derive(Serialize)]
pub struct InfoResponse<'a>{
    #[serde(rename = "Value")]
    value: HashMap<&'a str, &'a str>
}

#[derive(Debug, Deserialize)]
pub struct JoinRequest<'a>{
    #[serde(rename = "NetworkId")]
    pub network_id: &'a str,
    #[serde(rename = "EndpointId")]
	pub endpoint_id: &'a str,
    #[serde(rename = "SandboxKey")]
	pub sandbox_key: &'a str,
    #[serde(rename = "Options")]
	pub options: HashMap<&'a str, &'a str>
}

#[derive(Serialize)]
struct StaticRoute<'a> {
    #[serde(rename = "Destination")]
	destination:&'a str,
    #[serde(rename = "RouteType")]
	route_type: i64,
    #[serde(rename = "NextHop")]
	next_hop: &'a str
}

#[derive(Serialize)]
pub struct JoinResponse<'a> {
    #[serde(rename = "InterfaceName")]
	interface_name: InterfaceName,
    #[serde(rename = "Gateway")]
	gateway: &'a str,
    #[serde(rename = "GatewayIPv6")]
	gateway_ipv6: &'a str,
    #[serde(rename = "StaticRoutes")]
	static_routes: Vec<StaticRoute<'a>>,
    #[serde(rename = "DisableGatewayService")]
	disable_gateway_service: bool
}



#[derive(Debug, Serialize, Deserialize)]
pub struct IPAMData<'a>{
    #[serde(rename = "AddressSpace")]
    pub address_space: &'a str,
    #[serde(rename = "Pool")]
	pub pool: &'a str,
    #[serde(rename = "Gateway")]
	pub gateway: &'a str,
    #[serde(rename = "AuxAddresses")]
	pub aux_addresses: HashMap<&'a str, &'a str>
}

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    #[serde(rename = "Err")]
    err: &'a str
}

#[derive(Debug, Serialize)]
/// InterfaceName consists of the name of the interface in the global netns and
/// the desired prefix to be appended to the interface inside the container netns
pub struct InterfaceName {
    #[serde(rename = "SrcName")]
	pub src_name: String,
    #[serde(rename = "DstPrefix")]
	pub dst_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNetworkRequest<'a>{
    #[serde(rename = "NetworkId")]
    pub network_id: &'a str,
    #[serde(rename = "Options")]
	pub options: HashMap<&'a str, &'a str>,
    #[serde(rename = "IPv4Data")]
	pub ipv4_data: Vec<IPAMData<'a>>,
    #[serde(rename = "IPv6Data")]
	pub ipv6_data: Vec<IPAMData<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct AllocateNetworkRequest<'a> {
    #[serde(rename = "NetworkId")]
    pub network_id: &'a str,
    #[serde(rename = "Options")]
	pub options: HashMap<&'a str, &'a str>,
    #[serde(rename = "IPv4Data")]
	pub ipv4_data: Vec<IPAMData<'a>>,
    #[serde(rename = "IPv6Data")]
	pub ipv6_data: Vec<IPAMData<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkRequest<'a> {
    #[serde(rename = "NetworkId")]
    pub network_id: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct EndpointRequest<'a> {
    #[serde(rename = "NetworkId")]
    pub network_id:&'a str,
    #[serde(rename = "EndpointId")]
    pub endpoint_id:&'a str,
}

#[derive(Debug, Deserialize)]
pub struct CreateEndpointRequest<'a> {
    #[serde(rename = "NetworkId")]
    pub network_id:&'a str,
    #[serde(rename = "EndpointId")]
    pub endpoint_id:&'a str,
    #[serde(rename = "Interface")]
    pub interface: EndpointInterface<'a>,
    #[serde(rename = "Options")]
    pub options: HashMap<&'a str, &'a str>
}

#[derive(Debug, Deserialize)]
pub struct DiscoverRequest<'a> {
    #[serde(rename = "DiscoveryType")]
    pub discovery_type: i64,
    #[serde(rename = "DiscoveryData")]
    pub discovery_data:&'a str
}

#[derive(Debug, Deserialize)]
pub struct ExternalConnectivityRequest<'a> {
    #[serde(rename = "NetworkId")]
    pub network_id:&'a str,
    #[serde(rename = "EndpointId")]
    pub endpoint_id: &'a str,
    #[serde(rename = "Options")]
    pub options: HashMap<&'a str, &'a str>
}

/// Represent the interface a Network driver must fulfill.
pub trait Network {
    fn get_capabilities(&self) -> CapabilitiesResponse;
    fn create_network(&self, request: &CreateNetworkRequest);
	fn allocate_network(&self, request: &AllocateNetworkRequest) -> Result<AllocateNetworkResponse, ErrorResponse>;
	fn delete_network(&self, request: &NetworkRequest);
	fn free_network(&self, request: &NetworkRequest);
	fn create_endpoint(&self, request: &CreateEndpointRequest) -> Result<CreateEndpointResponse, ErrorResponse>;
	fn delete_endpoint(&self, request: &EndpointRequest);
	fn endpoint_info(&self, request: &EndpointRequest) -> Result<InfoResponse, ErrorResponse>;
	fn join(&self, request: &JoinRequest) -> Result<JoinResponse, ErrorResponse>;
	fn leave(&self, request: &EndpointRequest);
	fn discover_new(&self, request: &DiscoverRequest);
	fn discover_delete(&self, request: &DiscoverRequest);
	fn program_external_connectivity(&self, request: &ExternalConnectivityRequest);
	fn revoke_external_connectivity(&self, request: &EndpointRequest);
}

/// Parse the data, route request to a proper function in the `Network` trait and serialize JSON string from the result if any.
pub fn post<N:Network>(driver:&N, url:&str, data:&str) -> String {
    match url {
        ACTIVATE_URL => String::from(MANIFEST),
        CAPABILITIES_PATH => serde_json::to_string(&driver.get_capabilities()).unwrap(),
        ALLOCATE_NETWORK_PATH => {match driver.allocate_network(&serde_json::from_str::<AllocateNetworkRequest>(data).unwrap()) {
            Ok(r) => serde_json::to_string(&r).unwrap(),
            Err(r) => serde_json::to_string(&r).unwrap(),
        }},
        FREE_NETWORK_PATH => {driver.free_network(&serde_json::from_str::<NetworkRequest>(data).unwrap()); String::from(EMPTY)},
        CREATE_NETWORK_PATH => {driver.create_network(&serde_json::from_str::<CreateNetworkRequest>(data).unwrap()); String::from(EMPTY)},
        DELETE_NETWORK_PATH => {driver.free_network(&serde_json::from_str::<NetworkRequest>(data).unwrap()); String::from(EMPTY)},
        CREATE_ENDPOINT_PATH => { match driver.create_endpoint(&serde_json::from_str::<CreateEndpointRequest>(data).unwrap()) {
            Ok(r) => serde_json::to_string(&r).unwrap(),
            Err(e) => serde_json::to_string(&e).unwrap(),
        }},
        ENDPOINT_INFO_PATH => { match driver.endpoint_info(&serde_json::from_str::<EndpointRequest>(data).unwrap()) {
            Ok(r) => serde_json::to_string(&r).unwrap(),
            Err(e) => serde_json::to_string(&e).unwrap(),
        }},
        DELETE_ENDPOINT_PATH => {driver.delete_endpoint(&serde_json::from_str::<EndpointRequest>(data).unwrap()); String::from(EMPTY)},
        JOIN_PATH => { match driver.join(&serde_json::from_str::<JoinRequest>(data).unwrap()) {
            Ok(r) => serde_json::to_string(&r).unwrap(),
            Err(e) => serde_json::to_string(&e).unwrap(),
        }},
        LEAVE_PATH => {driver.leave(&serde_json::from_str::<EndpointRequest>(data).unwrap()); String::from(EMPTY)},
        DISCOVER_NEW_PATH => {driver.discover_new(&serde_json::from_str::<DiscoverRequest>(data).unwrap()); String::from(EMPTY)},
        DISCOVER_DELETE_PATH => {driver.discover_delete(&serde_json::from_str::<DiscoverRequest>(data).unwrap()); String::from(EMPTY)},
        PROGRAM_EXT_CONN_PATH => {driver.program_external_connectivity(&serde_json::from_str::<ExternalConnectivityRequest>(data).unwrap()); String::from(EMPTY)},
        REVOKE_EXT_CONN_PATH => {driver.revoke_external_connectivity(&serde_json::from_str::<EndpointRequest>(data).unwrap()); String::from(EMPTY)},
        _ => panic!("Unexpected request endpoint."),
    }
}
