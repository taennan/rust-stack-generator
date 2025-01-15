from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory

class EntityModulesTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema):
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            TemplateKeyValPair("modules", self._gen_modules()),
            TemplateKeyValPair("public_uses", self._gen_public_uses())
        ]

    def _gen_modules(self) -> str:
        modules = ""
        for entity_schema in self._global_schema.entities():
            modules += f"pub mod {entity_schema.name_lower()};\n"
        return modules
    
    def _gen_public_uses(self) -> str:
        uses = ""
        for entity_schema in self._global_schema.entities():
            uses += f"pub use {entity_schema.name_lower()}::*;\n"
        return uses
