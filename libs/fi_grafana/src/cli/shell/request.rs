use clap::{Args, Parser, Subcommand};

use crate::api::grafana::GrafanaClient;
use crate::cli::annotation::add::handle_add_annotation;
use crate::cli::annotation::options::AnnotationOptions;
use crate::cli::dashboard::add::handle_add_dashboard;
use crate::cli::dashboard::options::DashboardOptions;
use crate::cli::folder::add::handle_add_folder;
use crate::cli::folder::get::handle_get_folder;
use crate::cli::folder::options::FolderOptions;
use crate::cli::folder::permission::options::FolderPermissionOptions;
use crate::cli::folder::permission::set::handle_set_folder_permissions;
use crate::cli::role::Role;
use crate::cli::service_account::ServiceAccount;
use crate::cli::team::add::handle_add_team;
use crate::cli::team::delete::handle_del_team;
use crate::cli::team::get::handle_get_team;
use crate::cli::team::options::TeamOptions;
use crate::cli::user::User;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub method: RequestMethod,
}

#[derive(Subcommand)]
pub enum RequestMethod {
    #[clap(arg_required_else_help = true)]
    Add(AddRequest),
    #[clap(arg_required_else_help = true)]
    Get(GetRequest),
    #[clap(arg_required_else_help = true)]
    Set(SetRequest),
    #[clap(arg_required_else_help = true)]
    Del(DelRequest),
}

#[derive(Debug, Args)]
pub struct AddRequest {
    #[clap(subcommand)]
    pub resource: NamedResource,
}

#[derive(Debug, Args)]
pub struct GetRequest {
    #[clap(subcommand)]
    pub resource: NamedResource,
}

#[derive(Debug, Args)]
pub struct SetRequest {
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
    Annotation(AnnotationOptions),
    A(AnnotationOptions),
    Dashboard(DashboardOptions),
    D(DashboardOptions),
    ServiceAccount(ServiceAccount),
    SA(ServiceAccount),
    User(User),
    U(User),
    Team(TeamOptions),
    T(TeamOptions),
    Folder(FolderOptions),
    F(FolderOptions),
    Permission(FolderPermissionOptions),
    P(FolderPermissionOptions),
    Role(Role),
    R(Role),
}

pub async fn handle_add(grafana_client: &GrafanaClient, request: AddRequest) {
    use NamedResource;
    match request.resource {
        NamedResource::Annotation(opt) => {
            handle_add_annotation(grafana_client, &opt).await;
        }
        NamedResource::A(opt) => {
            handle_add_annotation(grafana_client, &opt).await;
        }
        NamedResource::Dashboard(opt) => {
            handle_add_dashboard(grafana_client, &opt).await;
        }
        NamedResource::D(opt) => {
            handle_add_dashboard(grafana_client, &opt).await;
        }
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(opt) => {
            handle_add_team(grafana_client, &opt).await;
        }
        NamedResource::T(opt) => {
            handle_add_team(grafana_client, &opt).await;
        }
        NamedResource::Folder(opt) => {
            handle_add_folder(grafana_client, &opt).await;
        }
        NamedResource::F(opt) => {
            handle_add_folder(grafana_client, &opt).await;
        }
        NamedResource::Permission(_) => {}
        NamedResource::P(_) => {}
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}

pub async fn handle_get(grafana_client: &GrafanaClient, request: GetRequest) {
    match request.resource {
        NamedResource::Annotation(_) => {}
        NamedResource::A(_) => {}
        NamedResource::Dashboard(_) => {}
        NamedResource::D(_) => {}
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(opt) => {
            handle_get_team(grafana_client, &opt).await;
        }
        NamedResource::T(opt) => {
            handle_get_team(grafana_client, &opt).await;
        }
        NamedResource::Folder(opt) => {
            handle_get_folder(grafana_client, &opt).await;
        }
        NamedResource::F(opt) => {
            handle_get_folder(grafana_client, &opt).await;
        }
        NamedResource::Permission(_) => {}
        NamedResource::P(_) => {}
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}

pub async fn handle_set(grafana_client: &GrafanaClient, request: SetRequest) {
    match request.resource {
        NamedResource::Annotation(_) => {}
        NamedResource::A(_) => {}
        NamedResource::Dashboard(_) => {}
        NamedResource::D(_) => {}
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(_) => {}
        NamedResource::T(_) => {}
        NamedResource::Folder(_) => {}
        NamedResource::F(_) => {}
        NamedResource::Permission(opt) => {
            handle_set_folder_permissions(grafana_client, &opt).await;
        }
        NamedResource::P(opt) => {
            handle_set_folder_permissions(grafana_client, &opt).await;
        }
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}

pub async fn handle_del(grafana_client: &GrafanaClient, request: DelRequest) {
    match request.resource {
        NamedResource::Annotation(_) => {}
        NamedResource::A(_) => {}
        NamedResource::Dashboard(_) => {}
        NamedResource::D(_) => {}
        NamedResource::ServiceAccount(_) => {}
        NamedResource::SA(_) => {}
        NamedResource::User(_) => {}
        NamedResource::U(_) => {}
        NamedResource::Team(opt) => {
            handle_del_team(grafana_client, &opt).await;
        }
        NamedResource::T(opt) => {
            handle_del_team(grafana_client, &opt).await;
        }
        NamedResource::Folder(_) => {}
        NamedResource::F(_) => {}
        NamedResource::Permission(_) => {}
        NamedResource::P(_) => {}
        NamedResource::Role(_) => {}
        NamedResource::R(_) => {}
    }
}