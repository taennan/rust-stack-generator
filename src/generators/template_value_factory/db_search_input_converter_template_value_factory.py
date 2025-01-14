from generators.schema.entity_schema import EntitySchema
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory
from constants.rust_types import RustTypes
from caseconverter import pascalcase, snakecase

class DBSearchInputConverterTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema, entity_schema: EntitySchema):
        self._entity_schema = entity_schema
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._global_schema, self._entity_schema.name())
        return [
            TemplateKeyValPair("mapped_methods", self._generate_mapped_fields()),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_mapped_fields(self) -> str:
        value = ""

        for field in self._entity_schema.fields():
            field_name = field.field_name()
            field_method = self._field_converter_method(field.type_name())

            if self._is_ignored_field(field_name):
                continue

            field_name_uppercase = pascalcase(field_name)
            value += f"""
            .apply_if(input.{field_name}, SearchFieldConverter::new(Column::{field_name_uppercase}).{field_method}())
            """

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
