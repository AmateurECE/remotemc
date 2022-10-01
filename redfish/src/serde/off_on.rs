///////////////////////////////////////////////////////////////////////////////
// NAME:            off_on.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Module for serialization of "Off" and "On" to booleans.
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

use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
};
use std::fmt;

pub fn serialize<S>(value: &bool, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        false => ser.serialize_str("Off"),
        true => ser.serialize_str("On"),
    }
}

struct OnOffVisitor;

impl<'de> Visitor<'de> for OnOffVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("On or Off")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "Off" => Ok(false),
            "On" => Ok(true),
            _ => Err(E::custom(format!("Unrecognized input: {}", value))),
        }
    }
}

pub fn deserialize<'de, D>(deser: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deser.deserialize_bool(OnOffVisitor)
}

///////////////////////////////////////////////////////////////////////////////
