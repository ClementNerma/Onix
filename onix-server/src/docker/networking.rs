use std::fmt::{Display, Formatter};

use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(
    SimpleObject, InputObject, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[graphql(input_name = "PortInput")]
pub struct Port {
    pub port: u16,
    pub port_type: PortType,
}

impl Port {
    pub fn collides_with(self, other: Self) -> bool {
        self.port == other.port && self.port_type.collides_with(other.port_type)
    }

    pub fn to_docker_port(self) -> String {
        format!(
            "{}{}",
            self.port,
            match self.port_type {
                PortType::TcpUdp => "",
                PortType::Tcp => "/tcp",
                PortType::Udp => "/udp",
            }
        )
    }

    // pub fn find_collision(ports: &[Self]) -> Option<(Self, Self)> {
    //     ports.iter().find_map(|port| {
    //         ports
    //             .iter()
    //             .find(|other_port| port.collides_with(**other_port))
    //             .map(|other_port| (*port, *other_port))
    //     })
    // }
}

impl Display for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_docker_port())
    }
}

#[derive(Enum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortType {
    TcpUdp,
    Tcp,
    Udp,
}

impl PortType {
    pub fn collides_with(self, other: PortType) -> bool {
        match (self, other) {
            (Self::TcpUdp, _)
            | (Self::Tcp, Self::TcpUdp | Self::Tcp)
            | (Self::Udp, Self::TcpUdp | Self::Udp) => false,

            (Self::Tcp, Self::Udp) | (Self::Udp, Self::Tcp) => false,
        }
    }

    pub fn covers_tcp(self) -> bool {
        match self {
            Self::TcpUdp | Self::Tcp => true,
            Self::Udp => false,
        }
    }

    pub fn covers_udp(self) -> bool {
        match self {
            Self::TcpUdp | Self::Udp => true,
            Self::Tcp => false,
        }
    }
}
