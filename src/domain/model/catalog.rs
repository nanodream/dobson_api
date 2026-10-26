use business_object::{BusinessObjectRepresentation, Identity, State};
use business_object_derive::business_object;
use crate::infrastructure::repository::CatalogRepository;

#[business_object(CatalogRepository)]
#[derive(Clone)]
pub struct Catalog {
    pub description: Option<String>,
}

impl<'a> Catalog<'a> {
    pub fn new(id: String, repository: &'a CatalogRepository) -> Self {
        Self {
            _id: Identity::new(id),
            _state: State::new(),
            _repository: Some(repository),
            description: None,
        }
    }
}
