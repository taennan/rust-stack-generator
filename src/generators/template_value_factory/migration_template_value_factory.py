from generators.schema.entity_schema import EntityField, EntitySchema
from generators.schema.global_schema import GlobalSchema
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.global_template_value_factory import GlobalTemplateValueFactory
from constants.rust_types import RustTypes

class MigrationTemplateValueFactory(TemplateValueFactory):

    def __init__(self, global_schema: GlobalSchema):
        self._global_schema = global_schema

    def keyvals(self) -> list[TemplateKeyValPair]:
        return [
            *GlobalTemplateValueFactory(self._global_schema).keyvals(),
            TemplateKeyValPair("table_create_calls", self._generate_table_create_calls()),
            TemplateKeyValPair("table_drop_methods", self._generate_table_drop_methods()),
            TemplateKeyValPair("entity_enums", self._generate_entity_enums()),
        ]
    
    def _generate_table_create_calls(self) -> str:
        column_calls = ""

        for entity_schema in self._global_schema.entities():
            column_methods = self._generate_table_column_methods(entity_schema)
            column_calls += f"""
        manager
            .create_table(
                Table::create()
                    .table({entity_schema.name()}::Table)
                    .if_not_exists()
{column_methods}
                    .to_owned(),
            )
            .await?;
    """
            
        return column_calls.rstrip("\n")
    
    def _generate_table_column_methods(self, entity_schema: EntitySchema) -> str:
        column_methods = ""

        for field in entity_schema.fields():
            column_value_prefix = self._generate_column_value_prefix(field)
            column_value_suffix = self._generate_column_value_suffix(field)
            column_value = f"{entity_schema.name()}::{field.field_name_pascal()}"
            column_methods += f"\t\t\t\t\t.col({column_value_prefix}({column_value}){column_value_suffix})\n"

        return column_methods.rstrip("\n")
    
    def _generate_column_value_prefix(self, field: EntityField) -> str:
        if field.field_name() == "id":
            return "primary_key_col"
        if field.field_name().endswith("_id"):
            return "uuid"
        if RustTypes.is_type_or_optional(field.type_name(), RustTypes.STRING):
            return "string"
        if RustTypes.is_number_type(field.type_name()):
            return "number"
        if RustTypes.is_time_type(field.type_name()):
            return "timestamp_col"
        raise Exception(f"Couldn't find a prefix for field '{field.field_name()}' of type '{field.type_name()}'")
    
    # TODO: May need further refinement for relation specifications
    def _generate_column_value_suffix(self, _field: EntityField) -> str:
        #if field.field_name().endswith("_id"):
        #    return ".unique_key()"
        return ""

    def _generate_table_drop_methods(self) -> str:
        methods = ""

        for entity_schema in self._global_schema.entities():
                methods += f"\t\t\t\t\t.table({entity_schema.name()}::Table)\n"

        return methods.rstrip("\n")
    
    def _generate_entity_enums(self) -> str:
        enums = ""
        
        for entity_schema in self._global_schema.entities():
            variants = self._generate_entity_enum_variants(entity_schema)
            enums += f"""
#[derive(DeriveIden)]
enum {entity_schema.name()} {'{'}
    Table,
{variants}
{'}'}
"""

        return enums
    
    def _generate_entity_enum_variants(self, entity_schema: EntitySchema) -> str:
        variants = ""

        for field in entity_schema.fields():
            variants += f"\t{field.field_name_pascal()},\n"

        return variants.rstrip("\n")
