from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory

class GlobalTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema):
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            TemplateKeyValPair("project", self._global_schema.project()),
            TemplateKeyValPair("project_lower", self._global_schema.project_lower()),
            TemplateKeyValPair("project_prefix", self._global_schema.project_prefix()),
            TemplateKeyValPair("project_prefix_lower", self._global_schema.project_prefix_lower()),
        ]
