use core_services_interface::org_user as service;
use db_interface::org_user as db;

pub(crate) struct OrgUserConverter(db::OrgUser);

impl From<db::OrgUser> for OrgUserConverter {
    fn from(model: db::OrgUser) -> Self {
        Self(model)
    }
}

impl From<OrgUserConverter> for service::OrgUser {
    fn from(converter: OrgUserConverter) -> Self {
        let model = converter.0;
        Self {
			id: model.id,
			claw_auth_id: model.claw_auth_id,
			email: model.email,
			description: model.description,
			created: model.created,
			updated: model.updated,
        }
    }
}
