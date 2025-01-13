from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.global_template_value_factory import GlobalTemplateValueFactory
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory

class GQLEndpointsModTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema):
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            *GlobalTemplateValueFactory(self._global_schema).keyvals(),
            TemplateKeyValPair("modules", self._generate_modules()),
        ]
        
    def _generate_modules(self) -> str:
        modules = ""
        for entity_schema in self._global_schema.entities():
            modules += f"mod {entity_schema.name_lower()};\n"
        return modules
