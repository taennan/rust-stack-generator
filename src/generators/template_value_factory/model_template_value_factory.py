from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory
from generators.schema.global_schema import GlobalSchema
from generators.schema.entity_schema import EntitySchema, EntityField
from constants.rust_types import RustTypes
from caseconverter import camelcase
from typing import Callable

class ModelTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema, entity_schema: EntitySchema, camelize_fields: bool=False, ignored_fields: list[str] = []):
        self._global_schema = global_schema
        self._entity_schema = entity_schema
        self._camelize_fields = camelize_fields
        self._ignored_fields = ignored_fields

    def keyvals(self) -> list[TemplateKeyValPair]:
        entity_name_factory = EntityNameTemplateValueFactory(self._global_schema, self._entity_schema.name())
        return [
            self._model_fields(),
            self._create_input_fields(),
            self._update_input_fields(),
            self._search_input_fields(),
            *entity_name_factory.keyvals(),
        ]

    def _model_fields(self) -> TemplateKeyValPair:
        return self._generate_input_fields(
            "model_fields",
            lambda field: not self._is_ignored_field(field.field_name()),
            lambda field: field.type_name()
        )

    def _create_input_fields(self) -> TemplateKeyValPair:
        return self._generate_input_fields(
            "create_input_fields",
            lambda field: (
                not self._is_ignored_mutation_field(field.field_name()) and
                not self._is_ignored_field(field.field_name()) and
                field.field_name() != "id"
            ),
            lambda field: field.type_name()
        )

    def _update_input_fields(self) -> TemplateKeyValPair:
        return self._generate_input_fields(
            "update_input_fields",
            lambda field: (
                not self._is_ignored_mutation_field(field.field_name()) and
                not self._is_id_field(field.field_name()) and
                not self._is_ignored_field(field.field_name())
            ),
            lambda field: f"Option<{field.type_name()}>"
        )

    def _search_input_fields(self) -> TemplateKeyValPair:
        def generate_field_type(field: EntityField) -> str:
            operator_type = self._search_operator_from_type(field.type_name())
            return f"Option<{operator_type}<{field.type_name()}>>"

        return self._generate_input_fields(
            "search_input_fields",
            lambda field: not self._is_ignored_field(field.field_name()),
            generate_field_type,
        )

    def _generate_input_fields(
        self, 
        key: str, 
        field_predicate: Callable[[EntityField], bool], 
        type_factory: Callable[[EntityField], str]
    ) -> TemplateKeyValPair:
        value = ""
        for field in self._entity_schema.fields():
            if field_predicate(field):
                field_type = type_factory(field)
                field_name = self._get_input_field_name(field.field_name())
                value += f"\tpub {field_name}: {field_type},\n"

        return TemplateKeyValPair(key, value.rstrip("\n"))

    def _get_input_field_name(self, name: str) -> str:
        if self._camelize_fields:
            return camelcase(name)
        return name

    def _is_id_field(self, field_name: str) -> bool:
            return field_name == "id"

    def _is_ignored_field(self, field_name: str) -> bool:
        return field_name in self._ignored_fields

    def _is_ignored_mutation_field(self, field_name: str) -> bool:
            return field_name in ["created", "updated"]

    def _search_operator_from_type(self, field_type: str) -> str:
        if RustTypes.is_exact_type(field_type):
            return "SearchExactOperator"
        if RustTypes.is_ranged_type(field_type):
            return "SearchRangedOperator"
        if RustTypes.is_iterable_type(field_type):
            return "SearchIterableOperator"

        raise Exception(f"Unknown field_type '{field_type}'")
