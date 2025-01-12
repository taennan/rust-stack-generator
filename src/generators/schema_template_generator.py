import shutil
from pathlib import Path
from generators.schema.entity_schema import EntitySchema
from generators.file_generator.file_template_generator import FileTemplateGenerator
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.entity_name_template_value_factory import EntityNameTemplateValueFactory
from generators.template_value_factory.model_template_value_factory import ModelTemplateValueFactory
from generators.template_value_factory.db_entity_template_value_factory import DBEntityTemplateValueFactory
from generators.template_value_factory.db_create_input_converter_template_value_factory import DBCreateInputConverterTemplateValueFactory
from generators.template_value_factory.db_update_input_converter_template_value_factory import DBUpdateInputConverterTemplateValueFactory
from generators.template_value_factory.db_search_input_converter_template_value_factory import DBSearchInputConverterTemplateValueFactory
from generators.template_value_factory.model_converter_template_value_factory import ModelConverterTemplateValueFactory
from generators.template_value_factory.migration_template_value_factory import MigrationTemplateValueFactory
from generators.template_value_factory.gql_context_template_value_factory import GQLContextTemplateValueFactory
from generators.template_value_factory.gql_endpoints_root_template_value_factory import GQLEndpointsRootTemplateValueFactory
from generators.schema.global_schema import GlobalSchema
from caseconverter import camelcase

class SchemaTemplateGenerator:

    def __init__(self, schema_filename: str):
        self._schema_filename = schema_filename

    def make_files(self):
        self._make_outdir()
        migration_dir = self._mkdir(self._outdir() / "migration")
        gql_dir = self._mkdir(self._outdir() / "graphql")
        gql_endpoints_dir = self._mkdir(gql_dir / "endpoints")

        global_schema = GlobalSchema(self._schema_filename)

        for entity_schema in global_schema.entities():
            entity_dir = self._mkdir(self._outdir() / entity_schema.name_snakecase())
            db_interface_dir = self._mkdir(entity_dir / "db_interface")
            db_impl_dir = self._mkdir(entity_dir / "db_impl")
            db_utils_dir = self._mkdir(db_impl_dir / "utils")
            service_interface_dir = self._mkdir(entity_dir / "service_interface")
            service_impl_dir = self._mkdir(entity_dir / "service_impl")
            service_utils_dir = self._mkdir(service_impl_dir / "utils")

            entity_name_template_value_factory = EntityNameTemplateValueFactory(entity_schema.name())

            self._gen_bru_file("count", entity_schema)
            self._gen_bru_file("create", entity_schema)
            self._gen_bru_file("getById", entity_schema)
            self._gen_bru_file("getMany", entity_schema)
            self._gen_bru_file("getOne", entity_schema)
            self._gen_bru_file("update", entity_schema)

            self._gen_file("graphql/endpoints.rs", gql_endpoints_dir / f"{entity_schema.name_snakecase()}.rs", entity_name_template_value_factory)

            self._gen_file("db/create_input_converter.rs", db_utils_dir / "create_input_converter.rs", DBCreateInputConverterTemplateValueFactory(entity_schema))
            self._gen_file("db/db_impl.rs", db_impl_dir / "database.rs", entity_name_template_value_factory)
            self._gen_file("db/db_mod.rs", db_impl_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("db/db_tests.rs", db_impl_dir / "tests.rs", entity_name_template_value_factory)
            self._gen_file("db/db_trait.rs", db_interface_dir / "database.rs", entity_name_template_value_factory)
            self._gen_file("db/entity.rs", db_impl_dir / "entity.rs", DBEntityTemplateValueFactory(entity_schema))
            self._gen_file("db/interface_mod.rs", db_interface_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("db/model_converter.rs", db_utils_dir / "model_converter.rs", ModelConverterTemplateValueFactory.db_model_converter(entity_schema))
            self._gen_file("db/models.rs", db_interface_dir / "models.rs", ModelTemplateValueFactory(entity_schema))
            self._gen_file("db/search_input_converter.rs", db_utils_dir / "search_input_converter.rs", DBSearchInputConverterTemplateValueFactory(entity_schema))
            self._gen_file("db/update_input_converter.rs", db_utils_dir / "update_input_converter.rs", DBUpdateInputConverterTemplateValueFactory(entity_schema))
            self._gen_file("db/utils_mod.rs", db_utils_dir / "mod.rs", entity_name_template_value_factory)

            self._gen_file("service/create_input_converter.rs", service_utils_dir / "create_input_converter.rs", ModelConverterTemplateValueFactory.service_input_converter(entity_schema))
            self._gen_file("service/interface_mod.rs", service_interface_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("service/model_converter.rs", service_utils_dir / "model_converter.rs", ModelConverterTemplateValueFactory(entity_schema, ignored_fields=["org_id"]))
            self._gen_file("service/models.rs", service_interface_dir / "models.rs", ModelTemplateValueFactory(entity_schema, ignored_fields=["org_id"]))
            self._gen_file("service/search_input_converter.rs", service_utils_dir / "search_input_converter.rs", ModelConverterTemplateValueFactory.service_input_converter(entity_schema))
            self._gen_file("service/search_many_input_converter.rs", service_utils_dir / "search_many_input_converter.rs", entity_name_template_value_factory)
            self._gen_file("service/service_impl.rs", service_impl_dir / "service.rs", entity_name_template_value_factory)
            self._gen_file("service/service_mod.rs", service_impl_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("service/service_tests.rs", service_impl_dir / "tests.rs", entity_name_template_value_factory)
            self._gen_file("service/service_trait.rs", service_interface_dir / "service.rs", entity_name_template_value_factory)
            self._gen_file("service/update_input_converter.rs", service_utils_dir / "update_input_converter.rs", ModelConverterTemplateValueFactory.service_input_converter(entity_schema))
            self._gen_file("service/utils_mod.rs", service_utils_dir / "mod.rs", entity_name_template_value_factory)

        self._gen_file("graphql/gql_context_wrapper.rs", gql_dir / "gql_context_wrapper.rs", GQLContextTemplateValueFactory(global_schema))
        self._gen_file("graphql/endpoints_root.rs", gql_endpoints_dir / "root.rs", GQLEndpointsRootTemplateValueFactory(global_schema))

        self._gen_file("migration/init_migration.rs", migration_dir / "init_migration.rs", MigrationTemplateValueFactory(global_schema))

    def _make_outdir(self):
        if self._outdir().exists():
            shutil.rmtree(self._outdir())
        self._outdir().mkdir()

    def _outdir(self) -> Path:
        return Path.cwd() / f"src/generated/{self._schema_filename}"

    def _mkdir(self, path: Path) -> Path:
        path.mkdir(exist_ok=True)
        return path

    def _gen_bru_file(self, template_name: str, entity_schema: EntitySchema):
        root_bruno_dir = self._mkdir(self._outdir() / "bruno")
        bruno_dir = self._mkdir(root_bruno_dir / camelcase(entity_schema.name()))
        value_factory = ModelTemplateValueFactory(entity_schema, camelize_fields=True, ignored_fields=["org_id"])
        filename = f"{template_name}.bru"

        self._gen_file(f"bruno/{filename}", bruno_dir / filename, value_factory)

    def _gen_file(
        self,
        template_name: str,
        output_path: Path,
        template_value_factory: TemplateValueFactory
    ):
        FileTemplateGenerator(template_name, output_path, template_value_factory).make_file()
