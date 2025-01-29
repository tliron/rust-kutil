use super::{extras::*, stack::*};

use std::{io, net::*};

/// Convert an unspecified address to reachable addresses.
///
/// This is useful for binding a server to a user-configured address. While many programs
/// and libraries treat unspecified addresses to mean all available interfaces, this function
/// allows you to bind to reachable addresses only.
///
/// * If address is None, will return all reachable addresses (both IPv6 and IPv4).
/// * If address is unspecified IPv6 (::0), will return all reachable IPv6 addresses.
/// * If address is unspecified IPv4 (0.0.0.0), will return all reachable IPv4 addresses.
/// * Otherwise, if the address is specific, will return None.
///
/// Note that the result could be an empty vector if no reachable addresses are found.
///
/// See also: [IPStack::get_reachable_addresses].
pub fn to_reachable_addresses(address: &Option<IpAddr>, include_loopbacks: bool) -> io::Result<Option<Vec<IpAddr>>> {
    Ok(match address {
        None => IPStack::Dual.get_reachable_addresses(include_loopbacks).ok(),

        Some(address) => {
            if address.is_unspecified() {
                match address {
                    IpAddr::V6(_) => IPStack::IPv6.get_reachable_addresses(include_loopbacks).ok(),
                    IpAddr::V4(_) => IPStack::IPv4.get_reachable_addresses(include_loopbacks).ok(),
                }
            } else {
                None
            }
        }
    })
}

impl IPStack {
    /// Get all reachable addresses.
    ///
    /// Note that the result could be an empty vector if no reachable addresses are found.
    ///
    /// See also: [to_reachable_addresses].
    pub fn get_reachable_addresses(&self, include_loopbacks: bool) -> io::Result<Vec<IpAddr>> {
        // interface: ethernet has "mq", docker has "noqueue"
        //            docker has "NO-CARRIER"
        // ip:        ethernet has "dynamic noprefixroute"
        //            docker doesn't

        let mut reachable_addresses = Vec::new();

        for interface in netdev::get_interfaces() {
            if interface.is_up() && interface.is_running() {
                //println!("{:?}", interface);
                if self.allows_ipv6() {
                    for address in &interface.ipv6 {
                        let address = address.addr();
                        if (include_loopbacks && address.is_loopback()) || address.is_reachable() {
                            reachable_addresses.push(address.into());
                        }
                    }
                }

                if self.allows_ipv4() {
                    for address in &interface.ipv4 {
                        let address = address.addr();
                        if (include_loopbacks && address.is_loopback()) || address.is_reachable() {
                            reachable_addresses.push(address.into());
                        }
                    }
                }
            }
        }

        Ok(reachable_addresses)

        // Dependency:
        // network-interface = "2.0.1"

        // let network_interfaces = NetworkInterface::show().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // for network_interface in network_interfaces {
        //     //println!("{:?}", network_interface);
        //     for address in network_interface.addr {
        //         let ip = address.ip();

        //         if self.allows(&ip) && ip.is_reachable() {
        //             reachable_addresses.push(ip);
        //         }
        //     }
        // }
    }
}
