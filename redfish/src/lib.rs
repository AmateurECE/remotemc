///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Library entrypoints for the Redfish crate
//
// CREATED:         10/01/2022
//
// LAST EDITED:     10/01/2022
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

mod computer_system;
mod computer_system_collection;
mod object_link;
mod serde;
mod service_root;

pub use crate::computer_system::*;
pub use crate::computer_system_collection::*;
pub use crate::object_link::*;
pub use crate::serde::*;
pub use crate::service_root::*;

///////////////////////////////////////////////////////////////////////////////
