use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use tokio::io::AsyncRead;
use tokio::net::UnixStream;
use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;

use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};

use crate::bridge_capnp::bridge;

use address::{Address, Length};
use arch::Architecture;
use mem::{PhysicalRead, VirtualRead, PhysicalWrite, VirtualWrite};

pub struct BridgeConnector {
    bridge: bridge::Client,
    runtime: Runtime,
}

impl BridgeConnector {
    pub fn connect<'a, P: AsRef<Path>>(path: P) -> Result<BridgeConnector> {
        let mut runtime = Runtime::new().unwrap();
        let stream = runtime.block_on(UnixStream::connect(path))?;
        let (reader, writer) = stream.split();

        let network = Box::new(twoparty::VatNetwork::new(
            reader,
            std::io::BufWriter::new(writer),
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));

        let mut rpc_system = RpcSystem::new(network, None);
        let bridge: bridge::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        runtime.spawn(rpc_system.map_err(|_e| ()));

        Ok(BridgeConnector {
            bridge: bridge,
            runtime: runtime,
        })
    }

    pub fn read_registers(&mut self) -> Result<Vec<u8>> {
        let request = self.bridge.read_registers_request();
        self.runtime
            .block_on(request.send().promise.and_then(|_r| Promise::ok(())))
            .map_err(|_e| Error::new(ErrorKind::Other, "unable to read registers"))
            .and_then(|_v| Ok(Vec::new()))
    }
}

impl PhysicalRead for BridgeConnector {
    // physRead @0 (address :UInt64, length :UInt64) -> (data :Data);
    fn phys_read(&mut self, addr: Address, len: Length) -> Result<Vec<u8>> {
        let mut request = self.bridge.phys_read_request();
        request.get().set_address(addr.addr);
        request.get().set_length(len.len);
        self.runtime
            .block_on(
                request.send().promise.and_then(|response| {
                    Promise::ok(Vec::from(pry!(pry!(response.get()).get_data())))
                }),
            )
            .map_err(|_e| Error::new(ErrorKind::Other, "unable to read memory"))
            .and_then(|v| Ok(v))
    }
}

impl VirtualRead for BridgeConnector {
    // virtRead @2 (arch: UInt8, dtb :UInt64, address :UInt64, length :UInt64) -> (data: Data);
    fn virt_read(&mut self, arch: Architecture, dtb: Address, addr: Address, len: Length) -> Result<Vec<u8>> {
        let mut request = self.bridge.virt_read_request();
        request.get().set_arch(arch.instruction_set.to_u8());
        request.get().set_dtb(dtb.addr);
        request.get().set_address(addr.addr);
        request.get().set_length(len.len);
        self.runtime
            .block_on(
                request.send().promise.and_then(|response| {
                    Promise::ok(Vec::from(pry!(pry!(response.get()).get_data())))
                }),
            )
            .map_err(|_e| Error::new(ErrorKind::Other, "unable to read memory"))
            .and_then(|v| Ok(v))
    }
}

impl PhysicalWrite for BridgeConnector {
    // physWrite @1 (address :UInt64, data: Data) -> (length :UInt64);
    fn phys_write(&mut self, addr: Address, data: &Vec<u8>) -> Result<Length> {
        let mut request = self.bridge.phys_write_request();
        request.get().set_address(addr.addr);
        request.get().set_data(data);
        self.runtime
            .block_on(
                request
                    .send()
                    .promise
                    .and_then(|response| Promise::ok(Length::from(pry!(response.get()).get_length()))),
            )
            .map_err(|_e| Error::new(ErrorKind::Other, "unable to write memory"))
            .and_then(|v| Ok(v))
    }
}

impl VirtualWrite for BridgeConnector {
    // virtWrite @3 (arch: UInt8, dtb: UInt64, address :UInt64, data: Data) -> (length :UInt64);
    fn virt_write(&mut self, arch: Architecture, dtb: Address, addr: Address, data: &Vec<u8>) -> Result<Length> {
        let mut request = self.bridge.virt_write_request();
        request.get().set_arch(arch.instruction_set.to_u8());
        request.get().set_dtb(dtb.addr);
        request.get().set_address(addr.addr);
        request.get().set_data(data);
        self.runtime
            .block_on(
                request
                    .send()
                    .promise
                    .and_then(|response| Promise::ok(Length::from(pry!(response.get()).get_length()))),
            )
            .map_err(|_e| Error::new(ErrorKind::Other, "unable to write memory"))
            .and_then(|v| Ok(v))
    }
}
