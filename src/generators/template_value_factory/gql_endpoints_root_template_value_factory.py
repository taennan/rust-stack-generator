from generators.schema.entity_schema import EntitySchema
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory

class GQLEndpointsRootTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema):
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            TemplateKeyValPair("object_imports", self._generate_object_imports()),
            TemplateKeyValPair("queries", self._generate_objects("Query")),
            TemplateKeyValPair("mutations", self._generate_objects("Mutation")),
        ]
    
    def _generate_object_imports(self) -> str:
        imports = ""

        for entity_schema in self._global_schema.entities():
            query_import = f"{entity_schema.name()}Queries"
            mutation_import = f"{entity_schema.name()}Mutations"
            imports += f"\n\t{entity_schema.name_snakecase()}::{'{'}{query_import}, {mutation_import}{'}'},"

        return imports
    
    def _generate_objects(self, object_suffix: str) -> str:
        objects = ""

        for entity_schema in self._global_schema.entities():
            objects += f"\t{entity_schema.name()}{object_suffix},\n"

        return objects
