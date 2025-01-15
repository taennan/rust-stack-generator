from generators.schema.entity_schema import EntitySchema
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory
from constants.rust_types import RustTypes

class DBUpdateInputConverterTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema, entity_schema: EntitySchema, ignored_fields: list[str]=[]):
        self._entity_schema = entity_schema
        self._global_schema = global_schema
        self._ignored_fields = ignored_fields

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._global_schema, self._entity_schema.name())
        return [
            TemplateKeyValPair("mapped_option_fields", self._generate_mapped_fields(self._is_option_field)),
            TemplateKeyValPair("mapped_maybe_fields", self._generate_mapped_fields(self._is_maybe_field)),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_mapped_fields(self, predicate) -> str:
        value = ""

        for field in self._entity_schema.fields():
            if self._is_ignored_field(field.field_name()):
                continue
            if predicate(field.type_name()):
                value += f"\t\t\t{field.field_name()},\n"

        return value.rstrip("\n")

    def _is_option_field(self, type_name: str) -> bool:
        return not RustTypes.is_optional(type_name)

    def _is_maybe_field(self, type_name: str) -> bool:
        return RustTypes.is_optional(type_name)

    def _is_ignored_field(self, field_name: str) -> bool:
        return field_name in ["id", "created", "updated"] or field_name in self._ignored_fields
