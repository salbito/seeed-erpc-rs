/// eRPC request type
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum MsgType {
    Invocation = 0,
    Oneway = 1,
    Reply = 2,
    Notification = 3,
    Unknown = 255,
}

impl From<u8> for MsgType {
    fn from(mt: u8) -> MsgType {
        match mt {
            0 => MsgType::Invocation,
            1 => MsgType::Oneway,
            2 => MsgType::Reply,
            3 => MsgType::Notification,
            _ => MsgType::Unknown,
        }
    }
}

/// Wio Terminal services
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum Service {
    System = 1,
    BLEHost = 2,
    BLEGap = 3,
    BLEGapBone = 4,
    BLEGattClient = 11,
    BLEGattServer = 12,
    BLECallback = 13,
    Wifi = 14,
    TCPIP = 15,
    WifiCallback = 18,
    Unknown = 255,
}

impl From<u8> for Service {
    fn from(mt: u8) -> Service {
        match mt {
            1 => Service::System,
            2 => Service::BLEHost,
            3 => Service::BLEGap,
            4 => Service::BLEGapBone,
            11 => Service::BLEGattClient,
            12 => Service::BLEGattServer,
            13 => Service::BLECallback,
            14 => Service::Wifi,
            15 => Service::TCPIP,
            18 => Service::WifiCallback,
            _ => Service::Unknown,
        }
    }
}

/// Wio Terminal request IDs for the System service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum SystemRequest {
    VersionID = 1,
    AckID = 2,
}

impl From<SystemRequest> for u8 {
    fn from(r: SystemRequest) -> u8 {
        r as u8
    }
}

/// Wio Terminal request IDs for the BLEHost service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum BLEHostRequest {
    InitID = 1,
    StartID = 2,
    DeInitID = 3,
}

impl From<BLEHostRequest> for u8 {
    fn from(r: BLEHostRequest) -> u8 {
        r as u8
    }
}

/// Wio Terminal request IDs for the BLEGap service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum BLEGapRequest {
    SetParamID = 1,
    GetParamID = 2,
    SetPairableModeID = 3,
}

impl From<BLEGapRequest> for u8 {
    fn from(r: BLEGapRequest) -> u8 {
        r as u8
    }
}

/// Wio Terminal request IDs for the BLEGapBone service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum BLEGapBoneRequest {
    LeBondSetParamID = 1,
    LeBondGetParamID = 2,
    LeBondPairID = 3,
    LeBondGetDisplayKeyID = 4,
    LeBondPasskeyInputConfirmID = 5,
    LeBondOobInputConfirmID = 6,
    LeBondJustWorkConfirmID = 7,
    LeBondPasskeyDisplayConfirmID = 8,
    LeBondUserConfirmID = 9,
    LeBondCfgLocalKeyDistributeID = 10,
    LeBondClearAllKeysID = 11,
    LeBondDeleteByIdxID = 12,
    LeBondDeleteByBdID = 13,
    LeBondGetSecLevelID = 14,
}

impl From<BLEGapBoneRequest> for u8 {
    fn from(r: BLEGapBoneRequest) -> u8 {
        r as u8
    }
}

/// Wio Terminal request IDs for the BLEGattClient service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum BLEGattClientRequest {
    BLEClientInitID = 1,
    BLEClientAddClientID = 2,
    ClientInitID = 3,
    ClientAllPrimarySrvDiscoveryID = 4,
    ClientByUuidSrvDiscoveryID = 5,
    ClientByUuid128SrvDiscoveryID = 6,
    ClientRelationshipDiscoveryID = 7,
    ClientAllCharDiscoveryID = 8,
    ClientByUuidCharDiscoveryID = 9,
    ClientByUuid128CharDiscoveryID = 10,
    ClientAllCharDescriptorDiscoveryID = 11,
    ClientAttrReadID = 12,
    ClientAttrReadUsingUuidID = 13,
    ClientAttrWriteID = 14,
    ClientAttrIndConfirmID = 15,
}

impl From<BLEGattClientRequest> for u8 {
    fn from(r: BLEGattClientRequest) -> u8 {
        r as u8
    }
}

/// Wio Terminal request IDs for the BLEGattServer service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum BLEGattServerRequest {
    BLEServerInitID = 1,
    BLECreateServiceID = 2,
    BLEDeleteServiceID = 3,
    BLEServiceStartID = 4,
    BLEGetServiceHandleID = 5,
    BLECreateCharID = 6,
    BLECreateDescID = 7,
    ServerSendDataID = 8,
    BLEServerGetAttrValueID = 9,
    ServerExecWriteConfirmID = 10,
    ServerAttrWriteConfirmID = 11,
    ServerAttrReadConfirmID = 12,
}

impl From<BLEGattServerRequest> for u8 {
    fn from(r: BLEGattServerRequest) -> u8 {
        r as u8
    }
}

/// Wio Terminal request IDs for the Wifi service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum WifiRequest {
    Connect = 1,
    ConnectBSSID = 2,
    Disconnect = 3,
    IsConnectedToAP = 4,
    IsUp = 5,
    GetMacAddress = 8,
    TurnOn = 27,
    TurnOff = 28,
    ScanStart = 64,
    IsScanning = 65,
    ScanGetAP = 66,
    ScanGetNumAPs = 67,
}

impl From<WifiRequest> for u8 {
    fn from(r: WifiRequest) -> u8 {
        r as u8
    }
}

/// Wio Terminal request IDs for the TCPIP service
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum TCPIPRequest {
    AdapterInit = 1,
    StaStart = 2,
    APStart = 3,
    Stop = 4,
    Up = 5,
    Down = 6,
    GetIPInfo = 7,
    SetIPInfo = 8,
    SetDNSInfo = 9,
    GetDNSInfo = 10,
    DHCPServStart = 11,
    DHCPServStop = 12,
    DHCPClientStart = 13,
    DHCPClientStop = 14,
    SetHostname = 15,
    GetHostname = 16,
    GetMAC = 17,
    SetMAC = 18,
}

impl From<TCPIPRequest> for u8 {
    fn from(r: TCPIPRequest) -> u8 {
        r as u8
    }
}
