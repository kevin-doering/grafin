use clap::{Args, Parser, Subcommand};

use crate::api::grafana::GrafanaClient;
use crate::cli::folder::get::handle_get_folder;
use crate::cli::folder::options::FolderOptions;
use crate::cli::folder::post::handle_post_folder;
use crate::cli::permission::options::PermissionOptions;
use crate::cli::permission::post::handle_post_permission;
use crate::cli::role::Role;
use crate::cli::service_account::ServiceAccount;
use crate::cli::team::delete::handle_del_team;
use crate::cli::team::get::handle_get_team;
use crate::cli::team::options::TeamOptions;
use crate::cli::team::post::handle_post_team;
use crate::cli::user::User;

pub mod input;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct FI {
    #[clap(subcommand)]
    pub method: CrudRequest,
}

#[derive(Subcommand)]
pub enum CrudRequest {
    #[clap(arg_required_else_help = true)]
    Add(PostRequest),
    #[clap(arg_required_else_help = true)]
    Get(GetRequest),
    #[clap(arg_required_else_help = true)]
    Set(PutRequest),
    #[clap(arg_required_else_help = true)]
    Del(DelRequest),
}

#[derive(Debug, Args)]
pub struct PostRequest {
    #[clap(subcommand)]
    pub resource: NamedResource,
}

#[derive(Debug, Args)]
pub struct GetRequest {
    #[clap(subcommand)]
    pub resource: NamedResource,
}

#[derive(Debug, Args)]
pub struct PutRequest {
    #[clap(subcommand)]
    pub resource: NamedResource,
}

#[derive(Debug, Args)]
pub struct DelRequest {
    #[clap(subcommand)]
    pub resource: NamedResource,
}

#[derive(Debug, Subcommand)]
pub enum NamedResource {
    ServiceAccount(ServiceAccount),
    SA(ServiceAccount),
    User(User),
    U(User),
    Team(TeamOptions),
    T(TeamOptions),
    Folder(FolderOptions),
    F(FolderOptions),
    Permission(PermissionOptions),
    P(PermissionOptions),
    Role(Role),
    R(Role),
}

pub async fn handle_post(client: &GrafanaClient, request: PostRequest) {
    match request.resource {
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(opt) => {
            handle_post_team(client, &opt).await;
        }
        NamedResource::T(opt) => {
            handle_post_team(client, &opt).await;
        }
        NamedResource::Folder(opt) => {
            handle_post_folder(client, &opt).await;
        }
        NamedResource::F(opt) => {
            handle_post_folder(client, &opt).await;
        }
        NamedResource::Permission(opt) => {
            handle_post_permission(client, &opt).await;
        }
        NamedResource::P(opt) => {
            handle_post_permission(client, &opt).await;
        }
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}

pub async fn handle_get(client: &GrafanaClient, request: GetRequest) {
    match request.resource {
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(opt) => {
            handle_get_team(client, &opt).await;
        }
        NamedResource::T(opt) => {
            handle_get_team(client, &opt).await;
        }
        NamedResource::Folder(opt) => {
            handle_get_folder(client, &opt).await;
        }
        NamedResource::F(opt) => {
            handle_get_folder(client, &opt).await;
        }
        NamedResource::Permission(_) => {}
        NamedResource::P(_) => {}
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}

pub async fn handle_put(_: &GrafanaClient, request: PutRequest) {
    match request.resource {
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(_) => {}
        NamedResource::T(_) => {}
        NamedResource::Folder(_) => {}
        NamedResource::F(_) => {}
        NamedResource::Permission(_) => {}
        NamedResource::P(_) => {}
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}

pub async fn handle_del(client: &GrafanaClient, request: DelRequest) {
    match request.resource {
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(opt) => {
            handle_del_team(client, &opt).await;
        }
        NamedResource::T(opt) => {
            handle_del_team(client, &opt).await;
        }
        NamedResource::Folder(_) => {}
        NamedResource::F(_) => {}
        NamedResource::Permission(_) => {}
        NamedResource::P(_) => {}
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}