import shutil
from pathlib import Path
from generators.schema.entity_schema import EntitySchema
from generators.file_generator.file_template_generator import FileTemplateGenerator
from generators.template_value_factory.global_template_value_factory import GlobalTemplateValueFactory
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
from generators.template_value_factory.gql_db_template_value_factory import GQLDBTemplateValueFactory
from generators.template_value_factory.gql_endpoints_mod_template_value_factory import GQLEndpointsModTemplateValueFactory
from generators.template_value_factory.gql_endpoints_root_template_value_factory import GQLEndpointsRootTemplateValueFactory
from generators.schema.global_schema import GlobalSchema
from caseconverter import camelcase

class SchemaTemplateGenerator:

    def __init__(self, schema_filename: str):
        self._schema_filename = schema_filename

    def make_files(self):
        self._make_outdir()
        error_dir = self._mkdir(self._outdir() / "error")
        migration_dir = self._mkdir(self._outdir() / "migration")

        gql_dir = self._mkdir(self._outdir() / "graphql")
        gql_context_dir = self._mkdir(gql_dir / "context")
        gql_endpoints_dir = self._mkdir(gql_dir / "schema")

        common_models_dir = self._mkdir(self._outdir() / "common_models")

        db_interface_dir = self._mkdir(self._outdir()/ "db_interface")
        db_models_dir = self._mkdir(self._outdir() / "db_models")
        db_impl_dir = self._mkdir(self._outdir() / "db_impl")
        db_utils_dir = self._mkdir(db_impl_dir / "utils")
        db_test_utils_dir = self._mkdir(db_utils_dir / "client_tests")

        service_interface_dir = self._mkdir(self._outdir() / "service_interface")
        service_impl_dir = self._mkdir(self._outdir() / "service_impl")

        global_schema = GlobalSchema(self._schema_filename)
        global_value_factory = GlobalTemplateValueFactory(global_schema)

        for entity_schema in global_schema.entities():
            entity_filename = f"{entity_schema.name_lower()}.rs"

            entity_db_dir = self._mkdir(db_impl_dir / entity_schema.name_lower())
            entity_db_utils_dir = self._mkdir(entity_db_dir / "utils")
            
            entity_service_interface_dir = self._mkdir(service_interface_dir / entity_schema.name_lower())
            entity_service_dir = self._mkdir(service_impl_dir / entity_schema.name_lower())
            entity_service_utils_dir = self._mkdir(entity_service_dir / "utils")

            entity_name_template_value_factory = EntityNameTemplateValueFactory(global_schema, entity_schema.name())

            self._gen_bru_file("count", global_schema, entity_schema)
            self._gen_bru_file("create", global_schema, entity_schema)
            self._gen_bru_file("getById", global_schema, entity_schema)
            self._gen_bru_file("getMany", global_schema, entity_schema)
            self._gen_bru_file("getOne", global_schema, entity_schema)
            self._gen_bru_file("update", global_schema, entity_schema)

            self._gen_file("gql/schema/endpoint.rs", gql_endpoints_dir / entity_filename, entity_name_template_value_factory)

            self._gen_file("db/interface/mod.rs", db_interface_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("db/interface/trait.rs", db_interface_dir / entity_filename, entity_name_template_value_factory)

            self._gen_file("db/interface/models.rs", db_models_dir / entity_filename, entity_name_template_value_factory)

            self._gen_file("db/impl/database.rs", entity_db_dir / "database.rs", entity_name_template_value_factory)
            self._gen_file("db/impl/entity.rs", entity_db_dir / "entity.rs", DBEntityTemplateValueFactory(global_schema, entity_schema))
            self._gen_file("db/impl/mod.rs", entity_db_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("db/impl/tests.rs", entity_db_dir / "tests.rs", entity_name_template_value_factory)

            self._gen_file("db/impl/utils/create_input_converter.rs", entity_db_utils_dir / "create_input_converter.rs", DBCreateInputConverterTemplateValueFactory(global_schema, entity_schema))
            self._gen_file("db/impl/utils/model_converter.rs", entity_db_utils_dir / "model_converter.rs", ModelConverterTemplateValueFactory.db_model_converter(global_schema, entity_schema))
            self._gen_file("db/impl/utils/search_input_converter.rs", entity_db_utils_dir / "search_input_converter.rs", DBSearchInputConverterTemplateValueFactory(global_schema, entity_schema))
            self._gen_file("db/impl/utils/update_input_converter.rs", entity_db_utils_dir / "update_input_converter.rs", DBUpdateInputConverterTemplateValueFactory(global_schema, entity_schema))
            self._gen_file("db/impl/utils/mod.rs", entity_db_utils_dir / "mod.rs", entity_name_template_value_factory)

            self._gen_file("service/interface/mod.rs", entity_service_interface_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("service/interface/models.rs", entity_service_interface_dir / "models.rs", entity_name_template_value_factory)
            self._gen_file("service/interface/trait.rs", entity_service_interface_dir / "service.rs", entity_name_template_value_factory)

            self._gen_file("service/impl/mod.rs", entity_service_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("service/impl/service.rs", entity_service_dir / "service.rs", entity_name_template_value_factory)
            self._gen_file("service/impl/tests.rs", entity_service_dir / "tests.rs", entity_name_template_value_factory)

            self._gen_file("service/impl/utils/mod.rs", entity_service_utils_dir / "mod.rs", entity_name_template_value_factory)
            self._gen_file("service/impl/utils/create_input_converter.rs", entity_service_utils_dir / "create_input_converter.rs", ModelConverterTemplateValueFactory.service_input_converter(global_schema, entity_schema))
            self._gen_file("service/impl/utils/model_converter.rs", entity_service_utils_dir / "model_converter.rs", ModelConverterTemplateValueFactory(global_schema, entity_schema, ignored_fields=["org_id"]))
            self._gen_file("service/impl/utils/search_input_converter.rs", entity_service_utils_dir / "search_input_converter.rs", ModelConverterTemplateValueFactory.service_input_converter(global_schema, entity_schema))
            self._gen_file("service/impl/utils/search_many_input_converter.rs", entity_service_utils_dir / "search_many_input_converter.rs", entity_name_template_value_factory)
            self._gen_file("service/impl/utils/update_input_converter.rs", entity_service_utils_dir / "update_input_converter.rs", ModelConverterTemplateValueFactory.service_input_converter(global_schema, entity_schema))

        self._gen_rust_file("gql/schema/mod", gql_endpoints_dir, GQLEndpointsModTemplateValueFactory(global_schema))
        self._gen_rust_file("gql/schema/root", gql_endpoints_dir, GQLEndpointsRootTemplateValueFactory(global_schema))
        self._gen_rust_file("gql/context/context_wrapper", gql_context_dir, GQLContextTemplateValueFactory(global_schema))
        self._gen_rust_file("gql/context/db_factory", gql_context_dir, GQLDBTemplateValueFactory(global_schema))
        self._gen_rust_file("gql/context/app_data", gql_context_dir, global_value_factory)
        self._gen_rust_file("gql/context/mod", gql_context_dir, global_value_factory)
        self._gen_rust_file("gql/routes", gql_dir, global_value_factory)
        self._gen_rust_file("gql/lib", gql_dir, global_value_factory)

        self._gen_file("common_models/create.rs", common_models_dir / "create.rs", global_value_factory)
        self._gen_file("common_models/delete.rs", common_models_dir / "delete.rs", global_value_factory)
        self._gen_file("common_models/update.rs", common_models_dir / "update.rs", global_value_factory)
        self._copy_dir("common_models/search", common_models_dir / "search")

        self._gen_rust_file("error/actix_web_integration", error_dir, global_value_factory)
        self._gen_rust_file("error/async_graphql_integration", error_dir, global_value_factory)
        self._gen_rust_file("error/sea_orm_integration", error_dir, global_value_factory)
        self._gen_rust_file("error/error_data", error_dir, global_value_factory)
        self._gen_rust_file("error/error", error_dir, global_value_factory)
        self._gen_rust_file("error/lib", error_dir, global_value_factory)

        self._gen_rust_file("db/impl/global_utils/mod", db_utils_dir, global_value_factory)
        self._gen_rust_file("db/impl/global_utils/client_impl", db_utils_dir, global_value_factory)
        self._gen_rust_file("db/impl/global_utils/database_connector", db_utils_dir, global_value_factory)
        self._gen_rust_file("db/impl/global_utils/client_tests/mod", db_test_utils_dir, global_value_factory)
        self._gen_rust_file("db/impl/global_utils/client_tests/mock_connection", db_test_utils_dir, global_value_factory)
        self._gen_rust_file("db/impl/global_utils/client_tests/test_modules", db_test_utils_dir, global_value_factory)

        self._copy_dir("service/impl/global_utils", service_impl_dir / "utils")
        
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

    def _gen_bru_file(self, template_name: str, global_schema: GlobalSchema, entity_schema: EntitySchema):
        root_bruno_dir = self._mkdir(self._outdir() / "bruno")
        bruno_dir = self._mkdir(root_bruno_dir / camelcase(entity_schema.name()))
        value_factory = ModelTemplateValueFactory(global_schema, entity_schema, camelize_fields=True, ignored_fields=["org_id"])
        filename = f"{template_name}.bru"

        self._gen_file(f"bruno/{filename}", bruno_dir / filename, value_factory)

    def _gen_rust_file(self, template_path: str, outdir: Path, template_value_factory: TemplateValueFactory):
        path = Path(template_path)
        filename = path.name + ".rs"
        self._gen_file(path.parent / filename, outdir / filename, template_value_factory)

    def _gen_file(
        self,
        template_name: str,
        output_path: Path,
        template_value_factory: TemplateValueFactory
    ):
        FileTemplateGenerator(template_name, output_path, template_value_factory).make_file()

    def _copy_dir(self, input: Path, output: Path):
        input = Path("src/templates") / input
        output = Path("src/generated") / output
        shutil.copytree(str(input), str(output))
