use core_services_interface::org as service;
use db_interface::org as db;

pub(crate) struct OrgConverter(db::Org);

impl From<db::Org> for OrgConverter {
    fn from(model: db::Org) -> Self {
        Self(model)
    }
}

impl From<OrgConverter> for service::Org {
    fn from(converter: OrgConverter) -> Self {
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
