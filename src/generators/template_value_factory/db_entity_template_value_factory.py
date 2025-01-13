from generators.schema.entity_schema import EntitySchema, EntityField
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory
from caseconverter import pascalcase

class DBEntityTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema, entity_schema: EntitySchema):
        self._entity_schema = entity_schema
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._global_schema, self._entity_schema.name())
        return [
            TemplateKeyValPair("model_fields", self._generate_model_fields()),
            TemplateKeyValPair("relation_enum", self._generate_relation_enum()),
            TemplateKeyValPair("related_impls", self._generate_related_impls()),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_model_fields(self) -> str:
        value = ""

        for field in self._entity_schema.fields():
            field_type = self._model_type_from_type_name(field.type_name())
            field_macro = self._generate_model_field_macro(field.field_name())

            if field_macro:
                value += f"\t{field_macro}\n"

            value += f"\t{field.field_name()}: {field_type},\n"

        return value.rstrip("\n")
    
    def _generate_model_field_macro(self, field_name: str) -> str:
        if field_name == "id":
            return "#[sea_orm(primary_key)]"
        return ""

    def _generate_relation_enum(self) -> str:
        field_value = ""
        for field in self._relation_fields():
            entity_name = field.field_name().rstrip("_id")
            entity_name_uppercase = pascalcase(entity_name)
            field_name_uppercase = pascalcase(field.field_name())

            field_value += f"""
    #[sea_orm(
        belongs_to = "crate::{entity_name}::entity::Entity",
        from = "Column::{field_name_uppercase}",
        to = "crate::{entity_name}::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    {entity_name_uppercase},""".lstrip("\n")

        if field_value == "":
            return ""
        
        return f"""
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {'{'}
{field_value}
{'}'}""".lstrip("\n")
    
    def _relation_fields(self) -> list[EntityField]:
        result = []
        for field in self._entity_schema.fields():
            if self._is_relation_field(field):
                result.append(field)
        return result
    
    def _is_relation_field(self, field: EntityField) -> bool:
        return field.field_name().endswith("_id")
    
    def _generate_related_impls(self) -> str:
        value = ""

        for field in self._relation_fields():
            entity_name = field.field_name().rstrip("_id")
            entity_name_uppercase = pascalcase(entity_name)

            value += f"""
impl Related<crate::{entity_name}::entity::Entity> for Entity {'{'}
    fn to() -> RelationDef {'{'}
        Relation::{entity_name_uppercase}.def()
    {'}'}
{'}'}""".lstrip("\n")
            
        return value

    def _model_type_from_type_name(self, type_name: str) -> str:
        if type_name == "NaiveDateTime":
            return "DateTime"
        return type_name
