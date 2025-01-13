from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.global_template_value_factory import GlobalTemplateValueFactory
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory

class GQLDBTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema):
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            *GlobalTemplateValueFactory(self._global_schema).keyvals(),
            TemplateKeyValPair("imports", self._generate_imports()),
            TemplateKeyValPair("methods", self._generate_methods()),
        ]
        
    def _generate_imports(self) -> str:
        imports = ""

        for entity_schema in self._global_schema.entities():
            struct_import = f"{entity_schema.name()}DB"
            trait_import = f"{struct_import}Trait"
            imports += f"\n\t{entity_schema.name_lower()}::{'{'}{struct_import}, {trait_import}{'}'},"

        return imports
    
    def _generate_methods(self) -> str:
        methods = ""

        for entity_schema in self._global_schema.entities():
            methods += f"""
    fn {entity_schema.name_lower()}_db(&self) -> impl {entity_schema.name()}DBTrait {'{'}
        {entity_schema.name()}DB::new(self.db_connection())
    {'}'}
"""

        return methods
