use std::sync::{Arc, Mutex};

use business_object::{repository::{Repository, RepositoryError}};
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
#[derive(Clone)]
pub struct CatalogRepository {
    connection: Arc<Mutex<Connection>>,
}

impl CatalogRepository {

    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        CatalogRepository { connection }
    }

}

impl<'a> Repository<Catalog<'a>> for CatalogRepository {

    fn list(&self) -> Result<Vec<CatalogRepresentation>, RepositoryError> {
        let locked_connection = self.connection.lock().unwrap();

        let list_sql = "SELECT id, description FROM catalog";
        let mut list_statement = locked_connection.prepare(list_sql).unwrap();

        let mut catalog_list: Vec<CatalogRepresentation> = vec!();
        while let Ok(State::Row) = list_statement.next() {
            let id   = list_statement.read::<String, usize>(0).unwrap();
            let description = list_statement.read::<String, usize>(1).unwrap();

            catalog_list.push(
                CatalogRepresentation {
                    id,
                    description: Some(description) }
            );
        }

        Ok(catalog_list)
    }

    fn exists(&self, id: &str) -> Result<bool, RepositoryError> {
        let locked_connection = self.connection.lock().unwrap();

        let exists_sql = "SELECT id FROM catalog WHERE id = :id";
        let mut exists_statement = locked_connection.prepare(exists_sql).unwrap();
        exists_statement.bind((":id", id)).expect("Unable to bind id");

        if let Ok(State::Row) = exists_statement.next() {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    fn get(&self, id: &str) -> Result<CatalogRepresentation, RepositoryError> {
        let locked_connection = self.connection.lock().unwrap();

        let select_sql = "SELECT id, description FROM catalog WHERE id = :id";
        let mut select_statement = locked_connection.prepare(select_sql).unwrap();
        select_statement.bind((":id", id)).unwrap();

        if let Ok(State::Row) = select_statement.next() {
            let id = select_statement.read::<String, usize>(0).unwrap();
            let description = select_statement.read::<String, usize>(1).unwrap();
            Ok(
                CatalogRepresentation {
                    id: id,
                    description: Some(description)
                }
            )
        }
        else {
            Err(RepositoryError::EntityNotExisting(String::from(id)))
        }

    }

    fn save(&self, object_rep: &CatalogRepresentation) -> Result<(), RepositoryError> {
        // Check if catalog already exists
        let exists = self.exists(object_rep.id.as_str()).unwrap();

        let locked_connection = self.connection.lock().unwrap();
        Ok(
            if exists {
                // Update existing catalog
                let update_sql = "UPDATE catalog SET description = :description WHERE id = :id";
                let mut update_statement = locked_connection.prepare(update_sql).unwrap();
                update_statement.bind((":id", object_rep.id.as_str())).unwrap();
                update_statement.bind((":description", object_rep.description.clone().unwrap_or_else(|| { String::from("") } ).as_str() )).unwrap();

                let _ = update_statement.next().expect("Error when updating");
            }
            else {
                // Create new catalog
                let insert_sql = "INSERT INTO catalog (id, description) VALUES ( :id, :description )";
                let mut insert_statement = locked_connection.prepare(insert_sql).unwrap();
                insert_statement.bind((":id", object_rep.id.as_str())).unwrap();
                insert_statement.bind((":description", object_rep.description.clone().unwrap_or_else(|| { String::from("") } ).as_str() )).unwrap();

                let _ = insert_statement.next().expect("Error when inserting");
            }
        )
    }

    fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        // Check if catalog already exists
        let exists = self.exists(id).unwrap();

        let locked_connection = self.connection.lock().unwrap();
        if exists {
            let delete_sql = "DELETE FROM catalog WHERE id = :id";
            let mut delete_statement = locked_connection.prepare(delete_sql).unwrap();
            delete_statement.bind((":id", id)).unwrap();
            delete_statement.next().expect("Error when deleting");
            Ok(())
        }
        else {
            Ok(())
        }
    }

}