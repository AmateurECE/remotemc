///////////////////////////////////////////////////////////////////////////////
// NAME:            ping.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Module for sending ICMP ECHO_REQUEST messages to a remote
//                  host.
//
// CREATED:         10/01/2022
//
// LAST EDITED:     10/04/2022
//
// Copyright 2022, Ethan D. Twardy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
////

use std::net::Ipv4Addr;
use std::process::Command;

pub async fn ping(address: Ipv4Addr) -> bool {
    // I'm lazy, so I'm just going to fork to ping(1) here.
    let result = Command::new("ping")
        .arg("-c1")
        .arg("-W1")
        .arg(address.to_string().as_str())
        .status()
        .expect("Failed to execute ping");
    return result.success();
}

///////////////////////////////////////////////////////////////////////////////
