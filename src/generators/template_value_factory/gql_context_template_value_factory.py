from generators.schema.entity_schema import EntitySchema
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory

class GQLContextTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema):
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            TemplateKeyValPair("methods", self._generate_methods()),
            TemplateKeyValPair("db_imports", self._generate_struct_imports("DB")),
            TemplateKeyValPair("service_imports", self._generate_struct_imports("Service")),
        ]
        
    def _generate_struct_imports(self, struct_type: str) -> str:
        imports = ""

        for entity_schema in self._global_schema.entities():
            struct_import = f"{entity_schema.name()}{struct_type}"
            trait_import = f"{struct_import}Trait"
            imports += f"\n\t{entity_schema.name_snakecase()}::{'{'}{struct_import}, {trait_import}{'}'},"

        return imports
    
    def _generate_methods(self) -> str:
        service_methods = ""
        db_methods = ""

        for entity_schema in self._global_schema.entities():
            db_method_name = f"{entity_schema.name_snakecase()}_db"

            service_methods += f"""
    pub fn {entity_schema.name_snakecase()}_service(&self) -> impl {entity_schema.name()}ServiceTrait {'{'}
        {entity_schema.name()}Service::new(self.{db_method_name}())
    {'}'}
"""
            
            db_methods += f"""
    fn {db_method_name}(&self) -> impl {entity_schema.name()}DBTrait {'{'}
        {entity_schema.name()}DB::new(self.db_connection())
    {'}'}
"""

        return service_methods + db_methods
