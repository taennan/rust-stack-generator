use core_services_interface::occupant as service;
use db_interface::occupant as db;

pub(crate) struct OccupantConverter(db::Occupant);

impl From<db::Occupant> for OccupantConverter {
    fn from(model: db::Occupant) -> Self {
        Self(model)
    }
}

impl From<OccupantConverter> for service::Occupant {
    fn from(converter: OccupantConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			name: model.name,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
