from generators.schema.entity_schema import EntitySchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory

class ModelConverterTemplateValueFactory(TemplateValueFactory):

    @classmethod
    def db_model_converter(cls, entity_schema: EntitySchema):
        return ModelConverterTemplateValueFactory(entity_schema)
    
    @classmethod
    def service_model_converter(cls, entity_schema: EntitySchema):
        return ModelConverterTemplateValueFactory(entity_schema, ignored_fields=["org_id"])
    
    @classmethod
    def service_input_converter(cls, entity_schema: EntitySchema):
        return ModelConverterTemplateValueFactory(entity_schema, ignored_fields=["org_id"], object_accessor="converter.input")

    def __init__(self, entity_schema: EntitySchema, ignored_fields: list[str] = [], object_accessor = "model"):
        self._entity_schema = entity_schema
        self._ignored_fields = ignored_fields
        self._object_accessor = object_accessor

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._entity_schema.name())
        return [
            TemplateKeyValPair("mapped_fields", self._generate_mapped_fields()),
            *entity_name_factory.keyvals(),
        ]
    
    def _generate_mapped_fields(self) -> str:
        value = ""

        for field in self._entity_schema.fields():
            if not field.field_name() in self._ignored_fields:
                value += f"\t\t\t{field.field_name()}: {self._object_accessor}.{field.field_name()},\n"

        return value.rstrip("\n")
