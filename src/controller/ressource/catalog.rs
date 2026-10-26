use std::sync::Arc;

use axum::Router;
use axum::extract::{Path, State};
use axum::http::{HeaderName, StatusCode, header};
use axum::routing::{ get, post, put, delete};
use axum::extract::{ Json as JsonExtract, rejection::JsonRejection };
use axum::response::{Json};
use business_object::repository::Repository;
use business_object::{HasIdentity, PersistantBusinessObject};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;
use crate::AppState;
use crate::controller::ressource::ApiError;
use crate::domain::model::catalog::Catalog;

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list))
        .route("/", post(new))
        .route("/{id_catalog}", get(read))
        .route("/{id_catalog}", put(update))
        .route("/{id_catalog}", delete(remove))
}

#[derive(Serialize, Deserialize)]
pub struct CatalogComponent {
    id: Option<String>,
    description: Option<String>
}

pub async fn list(state: State<Arc<AppState>>) -> Result<(StatusCode, Json<Vec<CatalogComponent>>), ApiError> {
    let repository = state.catalog_repository.as_ref();
    let list_result = Catalog::list(repository);

    match list_result {
        Ok(list) => {
            Ok((
                StatusCode::OK,
                Json(list.iter()
                    .map(|catalog| {
                        CatalogComponent {
                            id: Some(String::from(catalog.identity().id())),
                            description: catalog.description.clone()
                        }
                    }).collect()
                )
            ))
        }
        Err(error) => {
            error!("Error when listing catalogs: {}", error);
            Err(ApiError::Internal)
        }
    }
}

pub async fn new(state: State<Arc<AppState>>, body_result: Result<JsonExtract<CatalogComponent>, JsonRejection>)
    -> Result<(StatusCode, [(HeaderName, String);1]), ApiError> {
    match body_result {
        Ok(ref catalog_component) => {
            let id_catalog = catalog_component.id.clone().unwrap_or(Uuid::new_v4().to_string());
            let mut catalog = Catalog::new(
                id_catalog.clone(),
                state.catalog_repository.as_ref());
            catalog.description = catalog_component.description.clone();

            if let Err(error) = catalog.save() {
                error!("Error when creating business object : {}", error);
                Err(ApiError::Internal)
            } else {
                Ok((StatusCode::CREATED, [(header::LOCATION, format!("http://localhost:3000/v1/catalog/{}", &id_catalog))]))
            }
        },
        Err(rejection) => {
            error!("Invalid Json : {}", rejection);
            Err(ApiError::BadRequest)
        }
    }
}

pub async fn read(Path(id_catalog) : Path<String>, state: State<Arc<AppState>>) -> Result<(StatusCode, Json<CatalogComponent>), ApiError>  {

    let rehydrate_result = Catalog::rehydrate(
        id_catalog.as_str(),
        state.catalog_repository.as_ref());

    match rehydrate_result {
        Ok(catalog) => Ok(
                (
                    StatusCode::ACCEPTED,
                    Json(CatalogComponent {
                            id: Some(String::from(catalog.identity().id())),
                            description: catalog.description
                        }
                    )
                )
            ),
        Err(_) => Err(ApiError::NotFound)
    }

}

pub async fn update( Path(id_catalog) : Path<String>, state: State<Arc<AppState>>, body_result: Result<JsonExtract<CatalogComponent>, JsonRejection>)
    -> Result<StatusCode, ApiError> {

    match body_result {
        Ok(catalog_component) => {
            let catalog_result =
                Catalog::rehydrate(id_catalog.as_str(),
                    state.catalog_repository.as_ref());

            match catalog_result {
                Ok(mut catalog) => {
                    catalog.description = catalog_component.description.clone();

                    if let Err(error) = catalog.save() {
                        error!("Error when saving catalog: {}", error);
                        Err(ApiError::Internal)
                    } else {
                        Ok(StatusCode::ACCEPTED)
                    }
                },
                Err(error) => {
                    error!("Error when rehydrating catalog: {}", error);
                    Err(ApiError::Internal)
                }
            }
        },
        Err(rejection) => {
            error!("Invalid Json: {}", rejection);
            Err(ApiError::Internal)
        }
    }
}

pub async fn remove(Path(id_catalog) : Path<String>, state: State<Arc<AppState>>) -> Result<StatusCode, ApiError> {

    let catalog_repository = state.catalog_repository.as_ref();
    let catalog_result = Catalog::exists(&id_catalog, catalog_repository);

    match catalog_result {
        Ok(true) => {
            if let Err(error) = catalog_repository.delete(&id_catalog) {
                error!("Error when removing catalog : {}", error);
                Err(ApiError::Internal)
            } else {
                Ok(StatusCode::OK)
            }
        }
        Ok(false) => Ok(StatusCode::OK),
        Err(error) => {
            error!("Error when removing catalog : {}", error);
            Err(ApiError::Internal)
        }
    }

}