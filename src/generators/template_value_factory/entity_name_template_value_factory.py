from caseconverter import snakecase
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.global_template_value_factory import GlobalTemplateValueFactory
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory

class EntityNameTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema, entity_name: str):
        self._global_schema = global_schema
        self._entity_name = entity_name

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            *GlobalTemplateValueFactory(self._global_schema).keyvals(),
            TemplateKeyValPair("entity", self._entity_name),
            TemplateKeyValPair("entity_lower", snakecase(self._entity_name)),
        ]
