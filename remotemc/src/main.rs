///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the application.
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

use axum::{routing::get, Json, Router};
use clap::{self, Parser};
use serde::Deserialize;
use std::fs::File;
use uuid::Uuid;

mod computer_system;
mod computer_system_collection;
mod object_link;
mod service_root;

use computer_system_collection::ComputerSystemCollectionBuilder;
use object_link::ObjectLink;
use service_root::ServiceRootBuilder;

#[derive(clap::Parser)]
#[clap(author, version)]
struct Args {
    #[clap(value_parser)]
    /// Configuration file
    pub file: String,
}

#[derive(Deserialize)]
struct Configuration {
    pub listen_address: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let configuration: Configuration =
        serde_yaml::from_reader(File::open(&args.file)?)?;

    let service_root = ServiceRootBuilder::default()
        .odata_type("#ServiceRoot.v1_12_0.ServiceRoot".to_string())
        .id("RootService".to_string())
        .name("Root Service".to_string())
        .redfish_version("1.6.0".to_string())
        .uuid(Uuid::new_v4())
        .systems(ObjectLink::from("/redfish/v1/Systems"))
        .odata_id("/redfish/v1".to_string())
        .build()?;

    let computer_system_collection = ComputerSystemCollectionBuilder::default()
        .odata_type("#ComputerSystemCollection.ComputerSystemCollection"
                    .to_string())
        .name("Computer System Collection".to_string())
        .count(1)
        .members(vec![
            ObjectLink::from("/redfish/v1/Systems/1")
        ])
        .odata_id("/redfish/v1/Systems".to_string())
        .build()?;

    let app = Router::new()
        .route("/redfish/v1", get(|| async move {
            Json(service_root.clone())
        }))
        .route("/redfish/v1/Systems", get(|| async move {
            Json(computer_system_collection.clone())
        }))
        .route("/redfish/v1/Systems/1", get(computer_system::get));

    axum::Server::bind(&configuration.listen_address.parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
