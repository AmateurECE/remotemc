///////////////////////////////////////////////////////////////////////////////
// NAME:            computer_system_collection.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Service root implementation.
//
// CREATED:         09/26/2022
//
// LAST EDITED:     09/30/2022
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

use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use crate::object_link::ObjectLink;

#[derive(Builder, Clone, Default, Deserialize, Serialize)]
pub struct ComputerSystemCollection {
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Members@odata.count")]
    pub count: i32,
    #[serde(rename = "Members")]
    pub members: Vec<ObjectLink>,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
}

///////////////////////////////////////////////////////////////////////////////
