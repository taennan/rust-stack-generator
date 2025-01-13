import yaml
from pathlib import Path
from generators.schema.entity_schema import EntitySchema
from caseconverter import pascalcase, snakecase

class GlobalSchema:

    def __init__(self, schema_name: str):
        self._schema_name = schema_name
        self._data = self._read_data()

    def _read_data(self) -> dict:
        schema_path = Path.cwd() / f"src/schema/{self._schema_name}.yml"
        with open(schema_path, "r") as file:
            return yaml.safe_load(file)

    def project(self) -> str:
        return self._data["project"]
    
    def project_lower(self) -> str:
        return snakecase(self.project())
    
    def project_prefix(self) -> str:
        default = self._data["project_prefix"]
        if default:
            return default
        return pascalcase(self.project())
    
    def project_prefix_lower(self) -> str:
        return self.project_prefix().lower()

    def entities(self) -> list[EntitySchema]:
        entities = []
        for entity_data in self._data["entities"]:
            entity = EntitySchema(entity_data)
            entities.append(entity)
        return entities
