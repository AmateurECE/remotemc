///////////////////////////////////////////////////////////////////////////////
// NAME:            computer_system.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Service root implementation.
//
// CREATED:         09/26/2022
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

use crate::serde;
use ::serde::{Deserialize, Serialize};
use derive_builder::Builder;
use uuid::Uuid;

#[derive(Builder, Clone, Default, Deserialize, Serialize)]
pub struct ComputerSystem {
    #[builder(
        setter(skip),
        default = "\"#ComputerSystem.v1_16_1.ComputerSystem\".to_string()"
    )]
    #[serde(rename = "@odata.type")]
    pub odata_type: String,

    #[builder(default)]
    #[serde(rename = "Id")]
    pub id: String,
    #[builder(default)]
    #[serde(rename = "Name")]
    pub name: String,

    #[builder(default)]
    #[serde(rename = "SystemType")]
    pub system_type: String,
    #[builder(default)]
    #[serde(rename = "AssetTag")]
    pub asset_tag: String,

    #[builder(default)]
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[builder(default)]
    #[serde(rename = "Model")]
    pub model: String,
    #[builder(default)]
    #[serde(rename = "SubModel")]
    pub sub_model: String,
    #[builder(default)]
    #[serde(rename = "SKU")]
    pub sku: String,
    #[builder(default)]
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
    #[builder(default)]
    #[serde(rename = "PartNumber")]
    pub part_number: String,

    #[builder(default)]
    #[serde(rename = "Description")]
    pub description: String,
    #[builder(default)]
    #[serde(rename = "UUID")]
    pub uuid: Uuid,

    #[builder(default)]
    #[serde(rename = "HostName")]
    pub host_name: String,

    #[builder(default)]
    #[serde(rename = "PowerState", with = "serde::off_on")]
    pub power_state: bool,

    #[serde(rename = "@odata.id")]
    pub odata_id: String,
}

///////////////////////////////////////////////////////////////////////////////
