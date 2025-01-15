from generators.schema.entity_schema import EntitySchema
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory

class DBCreateInputConverterTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema, entity_schema: EntitySchema, ignored_fields: list[str]=[]):
        self._entity_schema = entity_schema
        self._global_schema = global_schema
        self._ignored_fields = ignored_fields

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._global_schema, self._entity_schema.name())
        return [
            TemplateKeyValPair("mapped_fields", self._generate_mapped_fields()),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_mapped_fields(self) -> str:
        value = ""
        for field in self._entity_schema.fields():
            if not self._is_ignored_field(field.field_name()):
                value += f"\t\t\t{field.field_name()},\n"
        return value.rstrip("\n")


    def _is_ignored_field(self, field_name: str) -> bool:
        return field_name in ["id", "created", "updated"] or field_name in self._ignored_fields
