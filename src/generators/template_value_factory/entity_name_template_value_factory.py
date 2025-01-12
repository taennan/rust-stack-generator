from caseconverter import snakecase
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory

class EntityNameTemplateValueFactory(TemplateValueFactory):

    def __init__(self, entity_name: str):
        self._entity_name = entity_name

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            TemplateKeyValPair("entity_name", self._entity_name),
            TemplateKeyValPair("entity_name_lowercase", snakecase(self._entity_name)),
        ]
