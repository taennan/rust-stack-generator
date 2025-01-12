import yaml
from pathlib import Path
from generators.schema.entity_schema import EntitySchema

class GlobalSchema:

    def __init__(self, schema_name: str):
        self._schema_name = schema_name
        self._data = self._read_data()

    def _read_data(self) -> dict:
        schema_path = Path.cwd() / f"src/schema/{self._schema_name}.yml"
        with open(schema_path, "r") as file:
            return yaml.safe_load(file)

    def schema_name(self) -> str:
        return self._data["name"]

    def entities(self) -> list[EntitySchema]:
        entities = []
        for entity_data in self._data["entities"]:
            entity = EntitySchema(entity_data)
            entities.append(entity)
        return entities
