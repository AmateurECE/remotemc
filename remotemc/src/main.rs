///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the application.
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

use ::serde::Deserialize;
use axum::{routing::get, Json, Router};
use clap::{self, Parser};
use redfish::{
    ComputerSystemBuilder, ComputerSystemCollectionBuilder, ObjectLink,
    ServiceRootBuilder,
};
use std::fs::File;
use uuid::Uuid;

mod remote_computer_system;

use remote_computer_system::RemoteComputerSystem;

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

    let service_root_path = "/redfish/v1".to_string();
    let computer_system_collection_path =
        service_root_path.to_owned() + "/Systems";
    let computer_system_path =
        computer_system_collection_path.to_owned() + "/1";

    let service_root = ServiceRootBuilder::default()
        .id("RootService".to_string())
        .name("Root Service".to_string())
        .redfish_version("1.6.0".to_string())
        .uuid(Uuid::new_v4())
        .systems(ObjectLink::from(computer_system_collection_path.as_str()))
        .odata_id(service_root_path.clone())
        .build()?;

    let computer_system_collection = ComputerSystemCollectionBuilder::default()
        .name("Computer System Collection".to_string())
        .count(1)
        .members(vec![ObjectLink::from(computer_system_path.as_str())])
        .odata_id(computer_system_collection_path.clone())
        .build()?;

    let computer_system = ComputerSystemBuilder::default()
        .id("1".to_string())
        .name("The Server".to_string())
        .system_type("Physical".to_string())
        .uuid(Uuid::new_v4())
        .host_name("twardyece.com".to_string())
        .power_state(true)
        .odata_id(computer_system_path.clone())
        .build()?;
    let mut remote_system = RemoteComputerSystem::new(computer_system);

    let app = Router::new()
        .route(
            &service_root_path,
            get(|| async move { Json(service_root.clone()) }),
        )
        .route(
            &computer_system_collection_path,
            get(|| async move { Json(computer_system_collection.clone()) }),
        )
        .route(
            &computer_system_path,
            get(|| async move { Json(remote_system.status()) }),
        );

    axum::Server::bind(&configuration.listen_address.parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
