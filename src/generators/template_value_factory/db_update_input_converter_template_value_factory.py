from generators.schema.entity_schema import EntitySchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory

class DBUpdateInputConverterTemplateValueFactory(TemplateValueFactory):

    def __init__(self, entity_schema: EntitySchema):
        self._entity_schema = entity_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._entity_schema.name())
        return [
            TemplateKeyValPair("mapped_fields", self._generate_mapped_fields()),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_mapped_fields(self) -> str:
        value = ""

        for field in self._entity_schema.fields():
            if not self._is_ignored_field(field.field_name()):
                value += f"\t\t\t{field.field_name()}: input.{field.field_name()}.map_or(NotSet, |v| Set(v)),\n"

        return value.rstrip("\n")


    def _is_ignored_field(self, field_name: str) -> bool:
        return field_name in ["id", "created", "updated"]
