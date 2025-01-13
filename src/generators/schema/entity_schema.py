from caseconverter import snakecase, pascalcase

class EntityField:

    def __init__(self, field_name: str, type_name: str):
        self._field_name = field_name
        self._type_name = type_name

    def type_name(self) -> str:
        return self._type_name

    def field_name(self) -> str:
        return self._field_name
    
    def field_name_pascal(self) -> str:
        return pascalcase(self.field_name())


class EntitySchema:

    def __init__(self, entity_data: dict):
        self._entity_data = entity_data

    def name(self) -> str:
        return self._entity_data["name"]

    def name_lower(self) -> str:
        return snakecase(self.name())

    def fields(self) -> list[EntityField]:
        result = []
        for field_name, field_type in self._entity_data["fields"].items():
            field = EntityField(field_name, field_type)
            result.append(field)
        return result

    def field(self, field_name: str) -> EntityField | None:
        for field in self.fields():
            if field.field_name() == field_name:
                return field
        return None
