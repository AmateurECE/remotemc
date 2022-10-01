///////////////////////////////////////////////////////////////////////////////
// NAME:            service_root.rs
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

use crate::object_link::ObjectLink;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use derive_builder::Builder;

#[derive(Builder, Clone, Default, Deserialize, Serialize)]
pub struct ServiceRoot {
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RedfishVersion")]
    pub redfish_version: String,
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Systems")]
    pub systems: ObjectLink,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
}

///////////////////////////////////////////////////////////////////////////////
