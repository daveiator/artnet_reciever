use std::{net::{UdpSocket, SocketAddr, Ipv4Addr, SocketAddrV4}, thread};
use socket2::Socket;
use std::sync::mpsc;
use artnet_protocol::{ArtCommand, PollReply};



pub type ArtnetReciever = mpsc::Receiver<artnet_protocol::Output>;

pub struct ArtnetRecieverBuilder {
    address: SocketAddr,
    reuse_address: bool,
    poll_reply_data: Option<PollReply>,
}

impl Default for ArtnetRecieverBuilder {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([0, 0, 0, 0], 6454)),
            reuse_address: true,
            poll_reply_data: None,
        }
    }
}

impl ArtnetRecieverBuilder {
    
    pub fn socket_address(mut self, address: SocketAddrV4) -> Self {
        self.address = address.into();
        self
    }

    pub fn ip_address(mut self, ip: Ipv4Addr) -> Self {
        self.address.set_ip(ip.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.address.set_port(port);
        self
    }
    pub fn reuse_address(mut self, reuse_address: bool) -> Self {
        self.reuse_address = reuse_address;
        self
    }

    pub fn poll_reply(mut self, poll_reply_data: PollReply) -> Self {
        self.poll_reply_data = Some(poll_reply_data);
        self
    }

    pub fn build(&self) -> std::io::Result<ArtnetReciever> {
        let socket = Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
        socket.set_reuse_address(self.reuse_address)?;
        socket.bind(&self.address.into())?;
        let socket: UdpSocket = socket.into();
        let poll_reply_data = clone_poll_reply_data(&self.poll_reply_data);
        let (tx, rx) = mpsc::channel();

        let _ = thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                let (size, controller_address) = match socket.recv_from(&mut buffer) {
                    Ok(packet) => packet,
                    Err(_) => {
                        //couldn't reveive data
                        continue;
                    }
                };

                let command = match ArtCommand::from_buffer(&buffer[..size]) {
                    Ok(command) => command,
                    Err(_) => {
                        //couldn't parse data
                        continue;
                    }
                };

                match command {
                    ArtCommand::Poll(_) => {
                        if poll_reply_data.is_none() {
                            continue;
                        }
                        let mut reply = clone_poll_reply_data(&poll_reply_data).unwrap();
                        let default_socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 6454);
                        reply.address = match socket.local_addr().unwrap_or(default_socket.into()).clone().ip() {
                            std::net::IpAddr::V4(ip) => ip,
                            std::net::IpAddr::V6(_) => continue,
                        };
                        // reply.port = socket.local_addr().unwrap_or(default_socket.into()).port();
                        reply.bind_ip = match controller_address.ip() {
                            std::net::IpAddr::V4(ip) => ip.octets(),
                            std::net::IpAddr::V6(_) => continue,
                        };

                        let reply_bytes = match ArtCommand::PollReply(Box::new(reply)).write_to_buffer() {
                            Ok(bytes) => bytes,
                            Err(_) => {
                                //couldn't write poll reply
                                continue;
                            },
                        };
                        match socket.send_to(&reply_bytes, &controller_address) {
                            Ok(_) => {},
                            Err(_) => {
                                //couldn't send poll reply
                                continue;
                            },
                        }
                    },
                    ArtCommand::Output(output) => {
                        match tx.send(output) {
                            Ok(_) => {}
                            Err(_) => {
                                //couldn't send data, receiver has been dropped
                                break;
                            }
                        }
                    }
                    ArtCommand::PollReply(_) => {} //ignore poll replies
                    _ => {} //unimplemented commands
                }
            }
        });
        Ok(rx)
    }
}

// Why can't the crate jsut derive clone???
fn clone_poll_reply_data(poll_reply_data: &Option<artnet_protocol::PollReply>) -> Option<artnet_protocol::PollReply> {
    match poll_reply_data {
        Some(poll_reply) => {
            Some(artnet_protocol::PollReply {
                address: poll_reply.address,
                port: poll_reply.port,
                version: poll_reply.version,
                port_address: poll_reply.port_address,
                oem: poll_reply.oem,
                ubea_version: poll_reply.ubea_version,
                status_1: poll_reply.status_1,
                esta_code: poll_reply.esta_code,
                short_name: poll_reply.short_name,
                long_name: poll_reply.long_name,
                node_report: poll_reply.node_report,
                num_ports: poll_reply.num_ports,
                port_types: poll_reply.port_types,
                good_input: poll_reply.good_input,
                good_output: poll_reply.good_output,
                swin: poll_reply.swin,
                swout: poll_reply.swout,
                sw_video: poll_reply.sw_video,
                sw_macro: poll_reply.sw_macro,
                sw_remote: poll_reply.sw_remote,
                spare: poll_reply.spare,
                style: poll_reply.style,
                mac: poll_reply.mac,
                bind_ip: poll_reply.bind_ip,
                bind_index: poll_reply.bind_index,
                status_2: poll_reply.status_2,
                filler: poll_reply.filler,
            })
        },
        None => None,
    }
}