from generators.schema.entity_schema import EntitySchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory
from constants.rust_types import RustTypes
from caseconverter import pascalcase

class DBSearchInputConverterTemplateValueFactory(TemplateValueFactory):

    def __init__(self, entity_schema: EntitySchema):
        self._entity_schema = entity_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._entity_schema.name())
        return [
            TemplateKeyValPair("mapped_methods", self._generate_mapped_fields()),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_mapped_fields(self) -> str:
        value = ""

        for field in self._entity_schema.fields():
            field_name = field.field_name()

            if self._is_ignored_field(field_name):
                continue

            field_name_uppercase = pascalcase(field_name)
            select_filter = self._select_filter_from_field_type(field.type_name())
            
            value += f"""
            .apply_if(input.{field_name}, |select, {field_name}| {{
                let expression = SimpleExpr::from({select_filter}::new(Column::{field_name_uppercase}, {field_name}));
                select.filter(expression)
            }})"""

        return value.rstrip("\n")

    def _is_ignored_field(self, field_name: str) -> bool:
        return field_name in ["id", "created", "updated"]

    def _select_filter_from_field_type(self, type_name: str) -> str:
        if RustTypes.is_ranged_type(type_name):
            return "SelectRangedFilter"
        if RustTypes.is_exact_type(type_name):
            return "SelectExactFilter"
        if RustTypes.is_iterable_type(type_name):
            return "SelectIterableFilter"
        
        raise Exception(f"Unknown field type '{type_name}' for entity '{self._entity_schema.name()}'")
