use std::collections::HashMap;

use serde::{Serialize, ser::SerializeMap, Deserialize, de::Visitor};

const DEFAULT_CONTENT_TYPE:&str = "application/vnd.docker.plugins.v1.1+json";
const ACTIVATE_URL:&str = "/Plugin.Activate";

const MANIFEST:&str = "{\"Implements\": [\"NetworkDriver\"]}";
const EMPTY: &str = "{}";
// LocalScope is the correct scope response for a local scope driver
const LOCAL_SCOPE:&str = "local";
// GlobalScope is the correct scope response for a global scope driver
const GLOBAL_SCOPE:&str = "global";

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



pub struct CapabilitiesResponse<'a> {
    pub scope: &'a str,
	pub connectivity_scope: &'a str,
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

#[derive(Debug)]
pub struct EndpointInterface<'a> {
	pub address:&'a str,
	pub address_ipv6:&'a str,
	pub address_mac:&'a str
}
impl<'a> Serialize for EndpointInterface<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("Address", self.address)?;
        map.serialize_entry("AddressIPv6", self.address_ipv6)?;
        map.serialize_entry("MacAddress", self.address_mac)?;
        map.end()
    }
}
struct EndpointInterfaceVisitor;
impl<'de> Visitor<'de> for EndpointInterfaceVisitor {
    type Value = EndpointInterface<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut address = None;
                let mut address_ipv6 = None;
                let mut address_mac = None;

                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "Address" => address = Some(map.next_value::<&str>()?),
                        "AddressIPv6" => address_ipv6 = Some(map.next_value::<&str>()?),
                        "MacAddress" => address_mac = Some(map.next_value::<&str>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{address:address.unwrap(), address_ipv6:address_ipv6.unwrap(), address_mac:address_mac.unwrap()})

    }
}
impl<'de> Deserialize<'de> for EndpointInterface<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(EndpointInterfaceVisitor{})
    }
}

pub struct CreateEndpointResponse<'a> {interface: EndpointInterface<'a>}
impl<'a> Serialize for CreateEndpointResponse<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Interface", &self.interface)?;
        map.end()
    }
}


pub struct InfoResponse<'a>{value: HashMap<&'a str, &'a str>}
impl<'a> Serialize for InfoResponse<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Value", &self.value)?;
        map.end()
    }
}
#[derive(Debug)]
pub struct JoinResponse<'a>{
    pub network_id: &'a str,
	pub endpoint_id: &'a str,
	pub sandbox_key: &'a str,
	pub options: HashMap<&'a str, &'a str>
}
impl<'a> Serialize for JoinResponse<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("NetworkID", self.network_id)?;
        map.serialize_entry("EndpointID", self.endpoint_id)?;
        map.serialize_entry("SandboxKey", self.sandbox_key)?;
        map.serialize_entry("Options", &self.options)?;
        map.end()
    }
}

#[derive(Debug)]
pub struct IPAMData<'a>{
    pub address_space: &'a str,
	pub pool: &'a str,
	pub gateway: &'a str,
	pub aux_addresses: HashMap<&'a str, &'a str>
}
struct IPAMDataVisitor;
impl<'de> Visitor<'de> for IPAMDataVisitor {
    type Value = IPAMData<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut address_space = None;
                let mut pool = None;
                let mut gateway = None;
                let mut aux_addresses = None;

                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "AddressSpace" => address_space = Some(map.next_value::<&str>()?),
                        "Pool" => pool = Some(map.next_value::<&str>()?),
                        "Gateway" => gateway = Some(map.next_value::<&str>()?),
                        "AuxAddresses" => aux_addresses = Some(map.next_value::<HashMap<&str, &str>>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{address_space:address_space.unwrap(), pool:pool.unwrap(), gateway:gateway.unwrap(), aux_addresses:aux_addresses.unwrap()})

    }
}
impl<'de> Deserialize<'de> for IPAMData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(IPAMDataVisitor{})
    }
}

impl<'a> Serialize for IPAMData<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("AddressSpace", self.address_space)?;
        map.serialize_entry("Pool", self.pool)?;
        map.serialize_entry("Gateway", self.gateway)?;
        map.serialize_entry("AuxAddresses", &self.aux_addresses)?;
        map.end()
    }
}
pub struct ErrorResponse<'a>{err: &'a str}
impl<'a> Serialize for ErrorResponse<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Err", self.err)?;
        map.end()
    }
}

#[derive(Debug)]
pub struct InterfaceName {
	pub src_name: String,
	pub dst_prefix: String,
}

#[derive(Debug)]
pub struct CreateNetworkRequest<'a>{
    pub network_id: &'a str,
	pub options: HashMap<&'a str, &'a str>,
	pub ipv4_data: Vec<IPAMData<'a>>,
	pub ipv6_data: Vec<IPAMData<'a>>,
}
struct CreateNetworkRequestVisitor;
impl<'de> Visitor<'de> for CreateNetworkRequestVisitor {
    type Value = CreateNetworkRequest<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut network_id = None;
                let mut options = None;
                let mut ipv4_data = None;
                let mut ipv6_data = None;

                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "NetworkId" => network_id = Some(map.next_value::<&str>()?),
                        "Options" => options = Some(map.next_value::<HashMap<&str, &str>>()?),
                        "IPv4Data" => ipv4_data = Some(map.next_value::<Vec<IPAMData>>()?),
                        "IPv6Data" => ipv6_data = Some(map.next_value::<Vec<IPAMData>>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{network_id:network_id.unwrap(), options:options.unwrap(), ipv4_data:ipv4_data.unwrap(), ipv6_data:ipv6_data.unwrap()})
    }
}
impl<'de> Deserialize<'de> for CreateNetworkRequest<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(CreateNetworkRequestVisitor{})
    }
}

#[derive(Debug)]
pub struct AllocateNetworkRequest<'a> {
    pub network_id: &'a str,
    pub options: HashMap<&'a str, &'a str>,
    pub ipv4_data: Vec<IPAMData<'a>>,
    pub ipv6_data: Vec<IPAMData<'a>>
}
struct AllocateNetworkRequestVisitor;
impl<'de> Visitor<'de> for AllocateNetworkRequestVisitor {
    type Value = AllocateNetworkRequest<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut network_id = None;
                let mut options = None;
                let mut ipv4_data = None;
                let mut ipv6_data = None;

                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "NetworkId" => network_id = Some(map.next_value::<&str>()?),
                        "Options" => options = Some(map.next_value::<HashMap<&str, &str>>()?),
                        "IPv4Data" => ipv4_data = Some(map.next_value::<Vec<IPAMData>>()?),
                        "IPv6Data" => ipv6_data = Some(map.next_value::<Vec<IPAMData>>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{network_id:network_id.unwrap(), options:options.unwrap(), ipv4_data:ipv4_data.unwrap(), ipv6_data:ipv6_data.unwrap()})
    }
}
impl<'de> Deserialize<'de> for AllocateNetworkRequest<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(AllocateNetworkRequestVisitor{})
    }
}
#[derive(Debug)]
pub struct NetworkRequest<'a> {
    pub network_id:&'a str
}
struct NetworkRequestVisitor;
impl<'de> Visitor<'de> for NetworkRequestVisitor {
    type Value = NetworkRequest<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut network_id = None;
                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "NetworkId" => network_id = Some(map.next_value::<&str>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{network_id:network_id.unwrap()})
    }
}
impl<'de> Deserialize<'de> for NetworkRequest<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(NetworkRequestVisitor{})
    }
}
#[derive(Debug)]
pub struct EndpointRequest<'a> {
    pub network_id:&'a str,
    pub endpoint_id:&'a str,
}
struct EndpointRequestVisitor;
impl<'de> Visitor<'de> for EndpointRequestVisitor {
    type Value = EndpointRequest<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut network_id = None;
                let mut endpoint_id = None;
                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "NetworkId" => network_id = Some(map.next_value::<&str>()?),
                        "EndpointId" => endpoint_id = Some(map.next_value::<&str>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{network_id:network_id.unwrap(), endpoint_id:endpoint_id.unwrap()})
    }
}
impl<'de> Deserialize<'de> for EndpointRequest<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(EndpointRequestVisitor{})
    }
}
#[derive(Debug)]
pub struct CreateEndpointRequest<'a> {
    pub network_id:&'a str,
    pub endpoint_id:&'a str,
    pub interface: EndpointInterface<'a>,
    pub options: HashMap<&'a str, &'a str>
}
struct CreateEndpointRequestVisitor;
impl<'de> Visitor<'de> for CreateEndpointRequestVisitor {
    type Value = CreateEndpointRequest<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut network_id = None;
                let mut endpoint_id = None;
                let mut interface = None;
                let mut options = None;

                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "NetworkId" => network_id = Some(map.next_value::<&str>()?),
                        "Options" => options = Some(map.next_value::<HashMap<&str, &str>>()?),
                        "EndpointId" => endpoint_id = Some(map.next_value::<&str>()?),
                        "Interface" => interface = Some(map.next_value::<EndpointInterface>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{network_id:network_id.unwrap(), options:options.unwrap(), endpoint_id:endpoint_id.unwrap(), interface:interface.unwrap()})
    }
}
impl<'de> Deserialize<'de> for CreateEndpointRequest<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(CreateEndpointRequestVisitor{})
    }
}
#[derive(Debug)]
pub struct DiscoverRequest<'a> {
    pub discovery_type: i64,
    pub discovery_data:&'a str
}
struct DiscoverRequestVisitor;
impl<'de> Visitor<'de> for DiscoverRequestVisitor {
    type Value = DiscoverRequest<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut discovery_type = None;
                let mut discovery_data = None;

                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "DiscoveryType" => discovery_type = Some(map.next_value::<i64>()?),
                        "DiscoveryData" => discovery_data = Some(map.next_value::<&str>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{discovery_type:discovery_type.unwrap(), discovery_data:discovery_data.unwrap()})
    }
}
impl<'de> Deserialize<'de> for DiscoverRequest<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(DiscoverRequestVisitor{})
    }
}
#[derive(Debug)]
pub struct ExternalConnectivityRequest<'a> {
    pub network_id:&'a str,
    pub endpoint_id: &'a str,
    pub options: HashMap<&'a str, &'a str>
}
struct ExternalConnectivityRequestVisitor;
impl<'de> Visitor<'de> for ExternalConnectivityRequestVisitor {
    type Value = ExternalConnectivityRequest<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected a map or a struct.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
                let mut network_id = None;
                let mut endpoint_id = None;
                let mut options = None;

                while let Some(k) = map.next_key::<&str>()? {
                    match k {
                        "NetworkId" => network_id = Some(map.next_value::<&str>()?),
                        "Options" => options = Some(map.next_value::<HashMap<&str, &str>>()?),
                        "EndpointId" => endpoint_id = Some(map.next_value::<&str>()?),
                        _ => (),
                    }
                }

                Ok(Self::Value{network_id:network_id.unwrap(), options:options.unwrap(), endpoint_id:endpoint_id.unwrap()})
    }
}
impl<'de> Deserialize<'de> for ExternalConnectivityRequest<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(ExternalConnectivityRequestVisitor{})
    }
}
pub trait Network {
    fn get_capabilities(&self) -> CapabilitiesResponse;
    fn create_network(&self, request: &CreateNetworkRequest);
	fn allocate_network(&self, request: &AllocateNetworkRequest) -> Result<AllocateNetworkResponse, ErrorResponse>;
	fn delete_network(&self, request: &NetworkRequest);
	fn free_network(&self, request: &NetworkRequest);
	fn create_endpoint(&self, request: &CreateEndpointRequest) -> Result<CreateEndpointResponse, ErrorResponse>;
	fn delete_endpoint(&self, request: &EndpointRequest);
	fn endpoint_info(&self, request: &EndpointRequest) -> Result<InfoResponse, ErrorResponse>;
	fn join(&self, request: &EndpointRequest) -> Result<JoinResponse, ErrorResponse>;
	fn leave(&self, request: &EndpointRequest);
	fn discover_new(&self, request: &DiscoverRequest);
	fn discover_delete(&self, request: &DiscoverRequest);
	fn program_external_connectivity(&self, request: &ExternalConnectivityRequest);
	fn revoke_external_connectivity(&self, request: &EndpointRequest);
}

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
        JOIN_PATH => { match driver.join(&serde_json::from_str::<EndpointRequest>(data).unwrap()) {
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
