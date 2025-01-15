from generators.schema.entity_schema import EntitySchema
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory
from constants.rust_types import RustTypes

class DBSearchInputConverterTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema, entity_schema: EntitySchema):
        self._entity_schema = entity_schema
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._global_schema, self._entity_schema.name())
        return [
            TemplateKeyValPair("mapped_fields", self._generate_mapped_fields()),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_mapped_fields(self) -> str:
        value = ""

        for field in self._entity_schema.fields():
            field_name = field.field_name()
            column_name = field.field_name_pascal()
            field_method = self._field_converter_method(field.type_name())
            if not self._is_ignored_field(field_name):
                value += f"({field_name}, Column::{column_name}, {field_method}),\n"

        return value.rstrip("\n")

    def _is_ignored_field(self, field_name: str) -> bool:
        return field_name in ["id", "created", "updated"]
    
    def _field_converter_method(self, type_name: str) -> str:
        if RustTypes.is_type_or_optional(type_name, RustTypes.STRING):
            return "string"
        if RustTypes.is_number_type(type_name):
            return "int"
        if RustTypes.is_type_or_optional(type_name, RustTypes.UUID):
            return "id"
        if RustTypes.is_time_type(type_name):
            return "date_time"
        
        raise Exception(f"Unknown field type '{type_name}' for entity '{self._entity_schema.name()}'")
