#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod controller {
    pub mod ressource {
        use std::sync::Arc;
        use axum::Router;
        use crate::AppState;
        pub mod catalog {
            use std::sync::Arc;
            use axum::Router;
            use axum::extract::{Path, State};
            use axum::routing::{get, post, put, delete};
            use axum::extract::{Json as JsonExtract, rejection::JsonRejection};
            use axum::response::Json;
            use business_object::{HasIdentity, Identity, PersistantBusinessObject};
            use serde::{Deserialize, Serialize};
            use crate::AppState;
            use crate::domain::model::catalog::Catalog;
            pub fn get_routes() -> Router<Arc<AppState>> {
                Router::new()
                    .route("/", get(list))
                    .route("/", post(new))
                    .route("/{id_catalog}", get(read))
                    .route("/{id_catalog}", put(update))
            }
            pub struct CatalogComponent {
                id: String,
                description: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CatalogComponent {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private229::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CatalogComponent",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "description",
                            &self.description,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CatalogComponent {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private229::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private229::Formatter,
                            ) -> _serde::__private229::fmt::Result {
                                _serde::__private229::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private229::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private229::Ok(__Field::__field0),
                                    1u64 => _serde::__private229::Ok(__Field::__field1),
                                    _ => _serde::__private229::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private229::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "id" => _serde::__private229::Ok(__Field::__field0),
                                    "description" => _serde::__private229::Ok(__Field::__field1),
                                    _ => _serde::__private229::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private229::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"id" => _serde::__private229::Ok(__Field::__field0),
                                    b"description" => {
                                        _serde::__private229::Ok(__Field::__field1)
                                    }
                                    _ => _serde::__private229::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private229::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private229::PhantomData<CatalogComponent>,
                            lifetime: _serde::__private229::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CatalogComponent;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private229::Formatter,
                            ) -> _serde::__private229::fmt::Result {
                                _serde::__private229::Formatter::write_str(
                                    __formatter,
                                    "struct CatalogComponent",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private229::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private229::Some(__value) => __value,
                                    _serde::__private229::None => {
                                        return _serde::__private229::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CatalogComponent with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private229::Some(__value) => __value,
                                    _serde::__private229::None => {
                                        return _serde::__private229::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CatalogComponent with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private229::Ok(CatalogComponent {
                                    id: __field0,
                                    description: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private229::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private229::Option<String> = _serde::__private229::None;
                                let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                                while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private229::Option::is_some(&__field0) {
                                                return _serde::__private229::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                                );
                                            }
                                            __field0 = _serde::__private229::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private229::Option::is_some(&__field1) {
                                                return _serde::__private229::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "description",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private229::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private229::Some(__field0) => __field0,
                                    _serde::__private229::None => {
                                        _serde::__private229::de::missing_field("id")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private229::Some(__field1) => __field1,
                                    _serde::__private229::None => {
                                        _serde::__private229::de::missing_field("description")?
                                    }
                                };
                                _serde::__private229::Ok(CatalogComponent {
                                    id: __field0,
                                    description: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["id", "description"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CatalogComponent",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private229::PhantomData::<
                                    CatalogComponent,
                                >,
                                lifetime: _serde::__private229::PhantomData,
                            },
                        )
                    }
                }
            };
            pub async fn list() -> String {
                return String::from("list_catalogs");
            }
            pub async fn new(
                catalog: Result<JsonExtract<CatalogComponent>, JsonRejection>,
            ) -> String {
                return String::from("new catalog");
            }
            pub async fn read(
                Path(id_catalog): Path<String>,
                state: State<Arc<AppState>>,
            ) -> Json<CatalogComponent> {
                let catalog_identity = Identity::new(id_catalog.clone());
                let catalog = Catalog::rehydrate(
                        catalog_identity,
                        state.catalog_repository.as_ref(),
                    )
                    .unwrap();
                Json(CatalogComponent {
                    id: String::from(catalog.identity().id()),
                    description: catalog.description.unwrap_or_default(),
                })
            }
            pub async fn update(
                Path(id_catalog): Path<String>,
                catalog: Result<JsonExtract<CatalogComponent>, JsonRejection>,
            ) -> String {
                String::from("update_catalog")
            }
        }
        pub mod celestial_body {
            use std::sync::Arc;
            use axum::Router;
            use axum::routing::get;
            use crate::AppState;
            pub fn get_routes() -> Router<Arc<AppState>> {
                Router::new().route("/", get(list_celestial_bodies))
            }
            pub async fn list_celestial_bodies() -> String {
                return String::from("list_celestial_bodies");
            }
        }
        pub fn get_routes() -> Router<Arc<AppState>> {
            Router::new()
                .nest("/catalog", catalog::get_routes())
                .nest("/celestial-body", celestial_body::get_routes())
        }
    }
    pub mod monitoring {
        use std::sync::Arc;
        use axum::Router;
        use axum::routing::get;
        use crate::AppState;
        pub fn get_routes() -> Router<Arc<AppState>> {
            Router::new().route("/", get(test))
        }
        pub async fn test() -> String {
            return String::from("Hello");
        }
    }
    use std::sync::Arc;
    use axum::Router;
    use crate::AppState;
    pub fn get_routes() -> Router<Arc<AppState>> {
        Router::new()
            .nest("/v1", ressource::get_routes())
            .nest("/ops", monitoring::get_routes())
    }
}
pub mod domain {
    pub mod model {
        pub mod catalog {
            use business_object::{
                BusinessObjectRepresentation, HasIdentity, HasState, Identity,
                Persistant, State,
            };
            use business_object_derive::business_object;
            use crate::infrastructure::repository::CatalogRepository;
            pub struct Catalog<'_a, 'a> {
                _id: ::business_object::Identity,
                _repository: Option<&'_a CatalogRepository>,
                _state: ::business_object::State<CatalogRepresentation>,
                pub description: Option<String>,
                pub test: Option<&'a String>,
            }
            #[automatically_derived]
            impl<'_a, 'a> ::core::clone::Clone for Catalog<'_a, 'a> {
                #[inline]
                fn clone(&self) -> Catalog<'_a, 'a> {
                    Catalog {
                        _id: ::core::clone::Clone::clone(&self._id),
                        _repository: ::core::clone::Clone::clone(&self._repository),
                        _state: ::core::clone::Clone::clone(&self._state),
                        description: ::core::clone::Clone::clone(&self.description),
                        test: ::core::clone::Clone::clone(&self.test),
                    }
                }
            }
            pub struct CatalogRepresentation<'a> {
                pub id: String,
                pub description: Option<String>,
                pub test: Option<&'a String>,
            }
            #[automatically_derived]
            impl<'a> ::core::clone::Clone for CatalogRepresentation<'a> {
                #[inline]
                fn clone(&self) -> CatalogRepresentation<'a> {
                    CatalogRepresentation {
                        id: ::core::clone::Clone::clone(&self.id),
                        description: ::core::clone::Clone::clone(&self.description),
                        test: ::core::clone::Clone::clone(&self.test),
                    }
                }
            }
            #[automatically_derived]
            impl<'a> ::core::fmt::Debug for CatalogRepresentation<'a> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "CatalogRepresentation",
                        "id",
                        &self.id,
                        "description",
                        &self.description,
                        "test",
                        &&self.test,
                    )
                }
            }
            impl BusinessObjectRepresentation for CatalogRepresentation {}
            impl<'_a, 'a> ::business_object::HasIdentity for Catalog<'_a, 'a> {
                fn identity(&self) -> &Identity {
                    &self._id
                }
            }
            impl<'_a, 'a> ::business_object::HasState for Catalog<'_a, 'a> {
                type BusinessObjectRepresentation = CatalogRepresentation;
                fn state(&self) -> &State<Self::BusinessObjectRepresentation> {
                    &self._state
                }
                fn state_mut(
                    &mut self,
                ) -> &mut State<Self::BusinessObjectRepresentation> {
                    &mut self._state
                }
            }
            impl<'_a, 'a> ::business_object::Persistant<'_a, 'a> for Catalog<'_a, 'a> {
                type Repository = CatalogRepository;
                fn set_repository(&mut self, repository: &'_a Self::Repository) {
                    self._repository = Some(repository);
                }
                fn repository(&self) -> Option<&Self::Repository> {
                    self._repository
                }
            }
            impl<'_a, 'a> From<CatalogRepresentation> for Catalog<'_a, 'a> {
                fn from(value: CatalogRepresentation) -> Self {
                    Self {
                        _id: Identity::new(value.id),
                        description: value.description,
                        test: value.test,
                        _state: State::new(),
                        _repository: None,
                    }
                }
            }
            impl<'_a, 'a> Into<CatalogRepresentation> for Catalog<'_a, 'a> {
                fn into(self) -> CatalogRepresentation {
                    CatalogRepresentation {
                        id: String::from(self._id.id()),
                        description: self.description.clone(),
                        test: self.test.clone(),
                    }
                }
            }
            impl<'_a, 'a> Catalog<'_a, 'a> {
                pub fn new(id: String) -> Self {
                    Self {
                        _id: Identity::new(id),
                        description: None,
                        test: None,
                        _state: State::new(),
                        _repository: None,
                    }
                }
                pub fn new_persistant(
                    id: String,
                    repository: &'_a CatalogRepository,
                ) -> Self {
                    Self {
                        _id: Identity::new(id),
                        description: None,
                        test: None,
                        _state: State::new(),
                        _repository: Some(repository),
                    }
                }
            }
        }
    }
    pub mod service {}
    pub mod port {
        pub mod business_object {}
        pub mod repository {}
    }
}
pub mod infrastructure {
    pub mod repository {
        use std::sync::{Arc, Mutex};
        use business_object::{Identity, repository::{Repository, RepositoryError}};
        use sqlite::{Connection, State};
        use crate::domain::model::catalog::{Catalog, CatalogRepresentation};
        /// Central méthod to initialise database for each repository
        /// It may manage différent database schema versions
        pub fn init_tables(connection: Arc<Mutex<Connection>>) {
            let table_init = "
        CREATE TABLE IF NOT EXISTS catalog (id TEXT, description TEXT);
        INSERT INTO catalog (id, description) VALUES ('123', 'hello');
    ";
            let _ = connection.lock().unwrap().execute(table_init);
        }
        /// ----------------------------------------------------------
        /// Catalog repository
        /// ----------------------------------------------------------
        pub struct CatalogRepository {
            connection: Arc<Mutex<Connection>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CatalogRepository {
            #[inline]
            fn clone(&self) -> CatalogRepository {
                CatalogRepository {
                    connection: ::core::clone::Clone::clone(&self.connection),
                }
            }
        }
        impl CatalogRepository {
            pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
                CatalogRepository { connection }
            }
        }
        impl<'a> Repository<Catalog<'a>> for CatalogRepository {
            fn get(
                &self,
                identity: &Identity,
            ) -> Result<CatalogRepresentation, RepositoryError> {
                let locked_connection = self.connection.lock().unwrap();
                let select_sql = "SELECT id, description FROM catalog WHERE id = :id";
                let mut select_statement = locked_connection
                    .prepare(select_sql)
                    .unwrap();
                select_statement.bind((":id", identity.id())).unwrap();
                if let Ok(State::Row) = select_statement.next() {
                    let id = select_statement.read::<String, usize>(0).unwrap();
                    let description = select_statement.read::<String, usize>(1).unwrap();
                    Ok(CatalogRepresentation {
                        id: id,
                        description: Some(description),
                    })
                } else {
                    Err(RepositoryError::EntityNotExisting(identity.clone()))
                }
            }
            fn save(
                &self,
                object: &CatalogRepresentation,
            ) -> Result<(), RepositoryError> {
                let locked_connection = self.connection.lock().unwrap();
                let exists_sql = "SELECT * FROM catalog WHERE id = :id";
                let mut exists_statement = locked_connection
                    .prepare(exists_sql)
                    .unwrap();
                exists_statement.bind((":id", object.id.as_str()));
                Ok(
                    if let Ok(State::Row) = exists_statement.next() {
                        let update_sql = "UPDATE catalog SET description = :description WHERE id = :id";
                        let mut update_statement = locked_connection
                            .prepare(update_sql)
                            .unwrap();
                        update_statement.bind((":id", object.id.as_str())).unwrap();
                        update_statement
                            .bind((
                                ":description",
                                object
                                    .description
                                    .clone()
                                    .unwrap_or_else(|| { String::from("") })
                                    .as_str(),
                            ))
                            .unwrap();
                    } else {
                        let insert_sql = "INSERT INTO catalog (id, description) VALUES ( :id, :description )";
                        let mut insert_statement = locked_connection
                            .prepare(insert_sql)
                            .unwrap();
                        insert_statement.bind((":id", object.id.as_str())).unwrap();
                        insert_statement
                            .bind((
                                ":description",
                                object
                                    .description
                                    .clone()
                                    .unwrap_or_else(|| { String::from("") })
                                    .as_str(),
                            ))
                            .unwrap();
                    },
                )
            }
            fn delete(&self, _id: &Identity) -> Result<(), RepositoryError> {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
}
use std::sync::{Arc, Mutex};
use rmcp::{
    ServerHandler, handler::server::wrapper::Parameters, tool, tool_handler, tool_router,
    transport::{
        StreamableHttpServerConfig, StreamableHttpService,
        streamable_http_server::session::local::LocalSessionManager,
    },
};
use sqlite::Connection;
use tracing_subscriber;
use tracing::info;
use controller::get_routes;
use crate::infrastructure::repository::{CatalogRepository, init_tables};
pub struct AppState {
    pub catalog_repository: Arc<CatalogRepository>,
}
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState {
            catalog_repository: ::core::clone::Clone::clone(&self.catalog_repository),
        }
    }
}
fn main() {
    let body = async {
        tracing_subscriber::fmt::init();
        let sqlite_connection = Arc::new(
            Mutex::new(Connection::open("./catalog.db").unwrap()),
        );
        init_tables(Arc::clone(&sqlite_connection));
        let app_state = Arc::new(AppState {
            catalog_repository: Arc::new(
                CatalogRepository::new(Arc::clone(&sqlite_connection)),
            ),
        });
        let config = StreamableHttpServerConfig::default()
            .with_legacy_session_mode(false)
            .with_json_response(true);
        let service = StreamableHttpService::new(
            || Ok(Calculator),
            LocalSessionManager::default().into(),
            config,
        );
        let app = get_routes().with_state(app_state).nest_service("/mcp", service);
        let address = "0.0.0.0:3000";
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:47",
                        "dobson_api",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(47u32),
                        ::tracing_core::__macro_support::Option::Some("dobson_api"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set_all(
                            &[
                                (::tracing::__macro_support::Option::Some(
                                    &format_args!("Server started at 🎧 {0}", address)
                                        as &dyn ::tracing::field::Value,
                                )),
                            ],
                        )
                });
            } else {
                if match ::tracing::Level::INFO {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                } <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set_all(
                                                    &[
                                                        (::tracing::__macro_support::Option::Some(
                                                            &format_args!("Server started at 🎧 {0}", address)
                                                                as &dyn ::tracing::field::Value,
                                                        )),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        let _ = axum::serve(listener, app).await;
    };
    let body = {
        if false {
            let _: &dyn ::core::future::Future<Output = ()> = &body;
        }
        body
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
struct AddParams {
    a: i32,
    b: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for AddParams {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "AddParams",
            "a",
            &self.a,
            "b",
            &&self.b,
        )
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for AddParams {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private229::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private229::Ok(__Field::__field0),
                        1u64 => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "a" => _serde::__private229::Ok(__Field::__field0),
                        "b" => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"a" => _serde::__private229::Ok(__Field::__field0),
                        b"b" => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private229::PhantomData<AddParams>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AddParams;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "struct AddParams",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private229::Some(__value) => __value,
                        _serde::__private229::None => {
                            return _serde::__private229::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct AddParams with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private229::Some(__value) => __value,
                        _serde::__private229::None => {
                            return _serde::__private229::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct AddParams with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private229::Ok(AddParams {
                        a: __field0,
                        b: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private229::Option<i32> = _serde::__private229::None;
                    let mut __field1: _serde::__private229::Option<i32> = _serde::__private229::None;
                    while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private229::Option::is_some(&__field0) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                    );
                                }
                                __field0 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private229::Option::is_some(&__field1) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                    );
                                }
                                __field1 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private229::Some(__field0) => __field0,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("a")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private229::Some(__field1) => __field1,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("b")?
                        }
                    };
                    _serde::__private229::Ok(AddParams {
                        a: __field0,
                        b: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["a", "b"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AddParams",
                FIELDS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<AddParams>,
                    lifetime: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for AddParams {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("AddParams")
        }
        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("dobson_api::AddParams")
        }
        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            {
                let mut schema = <::schemars::Schema as ::core::convert::TryFrom<
                    _,
                >>::try_from(
                        ::serde_json::Value::Object({
                            let mut object = ::serde_json::Map::new();
                            let _ = object
                                .insert(
                                    ("type").into(),
                                    ::serde_json::to_value(&"object").unwrap(),
                                );
                            object
                        }),
                    )
                    .unwrap();
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "a",
                        generator.contract().is_deserialize()
                            && <i32 as schemars::JsonSchema>::_schemars_private_is_option(),
                        { generator.subschema_for::<i32>() },
                    );
                }
                {
                    schemars::_private::insert_object_property(
                        &mut schema,
                        "b",
                        generator.contract().is_deserialize()
                            && <i32 as schemars::JsonSchema>::_schemars_private_is_option(),
                        { generator.subschema_for::<i32>() },
                    );
                }
                schema
            }
        }
        fn inline_schema() -> bool {
            false
        }
    }
};
struct Calculator;
#[automatically_derived]
impl ::core::clone::Clone for Calculator {
    #[inline]
    fn clone(&self) -> Calculator {
        Calculator
    }
}
impl Calculator {
    ///Generated tool metadata function for add
    pub fn add_tool_attr() -> rmcp::model::Tool {
        rmcp::model::Tool::new_with_raw(
            "add",
            Some("Add two numbers".into()),
            rmcp::handler::server::common::schema_for_input::<Parameters<AddParams>>()
                .unwrap_or_else(|e| {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Invalid input schema for `{0}`: {1}",
                                std::any::type_name::<Parameters<AddParams>>(),
                                e,
                            ),
                        );
                    }
                }),
        )
    }
    fn add(&self, Parameters(AddParams { a, b }): Parameters<AddParams>) -> String {
        (a + b).to_string()
    }
    fn tool_router() -> rmcp::handler::server::router::tool::ToolRouter<Self> {
        rmcp::handler::server::router::tool::ToolRouter::<Self>::new()
            .with_route((Self::add_tool_attr(), Self::add))
    }
}
impl ServerHandler for Calculator {
    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResponse, rmcp::ErrorData> {
        let tcc = rmcp::handler::server::tool::ToolCallContext::new(
            self,
            request,
            context,
        );
        Self::tool_router().call(tcc).await
    }
    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
        let supports_cache_hints = context
            .protocol_version()
            .is_some_and(|version| {
                version >= rmcp::model::ProtocolVersion::V_2026_07_28
            });
        Ok(rmcp::model::ListToolsResult {
            result_type: Some(rmcp::model::ResultType::COMPLETE),
            tools: Self::tool_router().list_all(),
            meta: None,
            next_cursor: None,
            ttl_ms: supports_cache_hints.then_some(0),
            cache_scope: supports_cache_hints.then_some(rmcp::model::CacheScope::Public),
        })
    }
    fn get_tool(&self, name: &str) -> Option<rmcp::model::Tool> {
        Self::tool_router().get(name).cloned()
    }
    fn get_info(&self) -> rmcp::model::ServerConfig {
        rmcp::model::ServerConfig::new(
                rmcp::model::ServerCapabilities::builder().enable_tools().build(),
            )
            .with_server_info(rmcp::model::Implementation::new("calculator", "1.0.0"))
            .with_instructions("A simple calculator".to_string())
    }
}
