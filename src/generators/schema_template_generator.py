import shutil
from pathlib import Path
from generators.schema.entity_schema import EntitySchema
from generators.file_generator.file_template_generator import FileTemplateGenerator
from generators.template_value_factory.entity_modules_template_value_factory import EntityModulesTemplateValueFactory
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

class CrateDirectories:
    def __init__(self, root: Path, src: Path):
        self.root = root
        self.src= src

class SchemaTemplateGenerator:

    def __init__(self, schema_filename: str):
        self._schema_filename = schema_filename

    def make_files(self):
        global_schema = GlobalSchema(self._schema_filename)
        global_value_factory = GlobalTemplateValueFactory(global_schema)
        entity_modules_facotry = EntityModulesTemplateValueFactory(global_schema)

        project_prefix = global_schema.project_kebab()

        self._make_outdir()
        self._mkdir(self._outdir() / "api")
        self._mkdir(self._outdir() / "api/apps")
        self._mkdir(self._outdir() / "api/endpoints")
        self._mkdir(self._outdir() / "db")
        self._mkdir(self._outdir() / "services")
        self._mkdir(self._outdir() / "models")

        admin_app_crate = self._make_crate(f"api/apps/{project_prefix}-admin")
        core_app_crate = self._make_crate(f"api/apps/{project_prefix}-core")
        app_base_crate = self._make_crate(f"api/apps/{project_prefix}-base")

        error_crate = self._make_crate(f"models/{project_prefix}-error")
        migration_dir = self._mkdir(self._outdir() / f"db/{project_prefix}-db-migrations")

        gql_crate = self._make_crate(f"api/endpoints/{project_prefix}-graphql")
        gql_context_dir = self._mkdir(gql_crate.src / "context")
        gql_endpoints_dir = self._mkdir(gql_crate.src / "schema")

        common_models_crate = self._make_crate(f"models/{project_prefix}-common-models")
        common_models_entity_root = self._mkdir(common_models_crate.src / "entities")

        db_interface_crate = self._make_crate(f"db/{project_prefix}-db-interface")
        db_impl_crate = self._make_crate(f"db/{project_prefix}-db-postgres")
        db_utils_dir = self._mkdir(db_impl_crate.src / "utils")
        db_test_utils_dir = self._mkdir(db_utils_dir / "client_tests")

        service_interface_crate = self._make_crate(f"services/{project_prefix}-services-interface")
        service_impl_crate = self._make_crate(f"services/{project_prefix}-services")

        for entity_schema in global_schema.entities():
            entity_filename = f"{entity_schema.name_lower()}.rs"

            entity_db_interface_dir = self._mkdir(db_interface_crate.src / entity_schema.name_lower())
            entity_db_dir = self._mkdir(db_impl_crate.src / "clients" / entity_schema.name_lower())
            entity_db_utils_dir = self._mkdir(entity_db_dir / "utils")
            
            entity_service_interface_dir = self._mkdir(service_interface_crate.src / entity_schema.name_lower())
            entity_service_dir = self._mkdir(service_impl_crate.src / entity_schema.name_lower())
            entity_service_utils_dir = self._mkdir(entity_service_dir / "utils")

            entity_name_template_value_factory = EntityNameTemplateValueFactory(global_schema, entity_schema.name())

            self._gen_bru_file("count", global_schema, entity_schema)
            self._gen_bru_file("create", global_schema, entity_schema)
            self._gen_bru_file("getById", global_schema, entity_schema)
            self._gen_bru_file("getMany", global_schema, entity_schema)
            self._gen_bru_file("getOne", global_schema, entity_schema)
            self._gen_bru_file("update", global_schema, entity_schema)

            self._gen_file("api/gql/src/schema/endpoint.rs", gql_endpoints_dir / entity_filename, entity_name_template_value_factory)

            self._gen_file("common_models/src/entities/entity.rs", common_models_entity_root / entity_filename, ModelTemplateValueFactory(global_schema, entity_schema, ignored_fields=["org_id"]))

            self._gen_rust_file("db/interface/src/database", entity_db_interface_dir, entity_name_template_value_factory)
            self._gen_rust_file("db/interface/src/models", entity_db_interface_dir, entity_name_template_value_factory)
            self._gen_rust_file("db/interface/src/mod", entity_db_interface_dir, entity_name_template_value_factory)

            db_client_dir = "db/impl/src/client"
            db_client_utils_dir = f"{db_client_dir}/utils"
            self._gen_rust_file(f"{db_client_dir}/database", entity_db_dir, entity_name_template_value_factory)
            self._gen_rust_file(f"{db_client_dir}/entity", entity_db_dir, DBEntityTemplateValueFactory(global_schema, entity_schema))
            self._gen_rust_file(f"{db_client_dir}/mod", entity_db_dir, entity_name_template_value_factory)
            self._gen_rust_file(f"{db_client_dir}/tests", entity_db_dir, entity_name_template_value_factory)

            self._gen_rust_file(f"{db_client_utils_dir}/create_input_converter", entity_db_utils_dir, DBCreateInputConverterTemplateValueFactory(global_schema, entity_schema))
            self._gen_rust_file(f"{db_client_utils_dir}/model_converter", entity_db_utils_dir, ModelConverterTemplateValueFactory.db_model_converter(global_schema, entity_schema))
            self._gen_rust_file(f"{db_client_utils_dir}/search_input_converter", entity_db_utils_dir, DBSearchInputConverterTemplateValueFactory(global_schema, entity_schema))
            self._gen_rust_file(f"{db_client_utils_dir}/update_input_converter", entity_db_utils_dir, DBUpdateInputConverterTemplateValueFactory(global_schema, entity_schema))
            self._gen_rust_file(f"{db_client_utils_dir}/mod", entity_db_utils_dir, entity_name_template_value_factory)

            self._gen_rust_file("service/interface/src/mod", entity_service_interface_dir, entity_name_template_value_factory)
            self._gen_rust_file("service/interface/src/models", entity_service_interface_dir, entity_name_template_value_factory)
            self._gen_rust_file("service/interface/src/service", entity_service_interface_dir, entity_name_template_value_factory)

            service_dir = "service/impl/src/service"
            service_utils_dir = f"{service_dir}/utils"
            self._gen_rust_file(f"{service_dir}/mod", entity_service_dir, entity_name_template_value_factory)
            self._gen_rust_file(f"{service_dir}/service", entity_service_dir, entity_name_template_value_factory)
            self._gen_rust_file(f"{service_dir}/tests", entity_service_dir, entity_name_template_value_factory)

            self._gen_rust_file(f"{service_utils_dir}/mod", entity_service_utils_dir, entity_name_template_value_factory)
            self._gen_rust_file(f"{service_utils_dir}/create_input_converter", entity_service_utils_dir, ModelConverterTemplateValueFactory.service_input_converter(global_schema, entity_schema))
            self._gen_rust_file(f"{service_utils_dir}/model_converter", entity_service_utils_dir, ModelConverterTemplateValueFactory(global_schema, entity_schema, ignored_fields=["org_id"]))
            self._gen_rust_file(f"{service_utils_dir}/search_input_converter", entity_service_utils_dir, ModelConverterTemplateValueFactory.service_input_converter(global_schema, entity_schema))
            self._gen_rust_file(f"{service_utils_dir}/search_many_input_converter", entity_service_utils_dir, entity_name_template_value_factory)
            self._gen_rust_file(f"{service_utils_dir}/update_input_converter", entity_service_utils_dir, ModelConverterTemplateValueFactory.service_input_converter(global_schema, entity_schema))

        self._gen_rust_file("api/gql/src/schema/mod", gql_endpoints_dir, GQLEndpointsModTemplateValueFactory(global_schema))
        self._gen_rust_file("api/gql/src/schema/root", gql_endpoints_dir, GQLEndpointsRootTemplateValueFactory(global_schema))
        self._gen_rust_file("api/gql/src/context/context_wrapper", gql_context_dir, GQLContextTemplateValueFactory(global_schema))
        self._gen_rust_file("api/gql/src/context/db_factory", gql_context_dir, GQLDBTemplateValueFactory(global_schema))
        self._gen_rust_file("api/gql/src/context/app_data", gql_context_dir, global_value_factory)
        self._gen_rust_file("api/gql/src/context/mod", gql_context_dir, global_value_factory)
        self._gen_rust_file("api/gql/src/routes", gql_crate.src, global_value_factory)
        self._gen_rust_file("api/gql/src/lib", gql_crate.src, global_value_factory)
        self._gen_cargo_file("api/gql", gql_crate.root, global_value_factory)

        self._gen_rust_file("common_models/src/create", common_models_crate.src, global_value_factory)
        self._gen_rust_file("common_models/src/delete", common_models_crate.src, global_value_factory)
        self._gen_rust_file("common_models/src/update", common_models_crate.src, global_value_factory)
        self._copy_dir("common_models/src/search", common_models_crate.src / "search")
        self._gen_rust_file("common_models/src/entities/mod", common_models_entity_root, entity_modules_facotry)
        self._gen_cargo_file("common_models", common_models_crate.root, global_value_factory)

        self._gen_rust_file("error/src/actix_web_integration", error_crate.src, global_value_factory)
        self._gen_rust_file("error/src/async_graphql_integration", error_crate.src, global_value_factory)
        self._gen_rust_file("error/src/sea_orm_integration", error_crate.src, global_value_factory)
        self._gen_rust_file("error/src/error_data", error_crate.src, global_value_factory)
        self._gen_rust_file("error/src/error", error_crate.src, global_value_factory)
        self._gen_rust_file("error/src/lib", error_crate.src, global_value_factory)
        self._gen_cargo_file("error", error_crate.root, global_value_factory)

        db_global_utils_dir = "db/impl/src/utils"
        self._gen_rust_file(f"db/impl/src/lib", db_impl_crate.src, global_value_factory)
        self._gen_file("db/impl/src/clients_mod.rs", db_impl_crate.src / "clients/mod.rs", entity_modules_facotry)
        self._gen_rust_file(f"{db_global_utils_dir}/mod", db_utils_dir, global_value_factory)
        self._gen_rust_file(f"{db_global_utils_dir}/client_impl", db_utils_dir, global_value_factory)
        self._gen_rust_file(f"{db_global_utils_dir}/database_connector", db_utils_dir, global_value_factory)
        self._gen_rust_file(f"{db_global_utils_dir}/search_field_converter", db_utils_dir, global_value_factory)
        self._gen_rust_file(f"{db_global_utils_dir}/test_utils/mod", db_test_utils_dir, global_value_factory)
        self._gen_rust_file(f"{db_global_utils_dir}/test_utils/mock_connection", db_test_utils_dir, global_value_factory)
        self._gen_rust_file(f"{db_global_utils_dir}/test_utils/test_macros", db_test_utils_dir, global_value_factory)
        self._gen_cargo_file("db/impl", db_impl_crate.root, global_value_factory)

        self._gen_rust_file("db/interface/src/lib", db_interface_crate.src, entity_modules_facotry)
        self._gen_cargo_file("db/interface", db_interface_crate.root, global_value_factory)

        self._gen_rust_file("service/impl/src/lib", service_impl_crate.src, entity_modules_facotry)
        self._copy_dir("service/impl/src/utils", service_impl_crate.src / "utils")
        self._gen_cargo_file("service/impl", service_impl_crate.root, global_value_factory)

        self._gen_rust_file("service/interface/src/lib", service_interface_crate.src, entity_modules_facotry)
        self._gen_cargo_file("service/interface", service_interface_crate.root, global_value_factory)
        
        self._gen_rust_file("migration/init_migration", migration_dir, MigrationTemplateValueFactory(global_schema))

        self._gen_rust_file("api/app-base/src/app_data", app_base_crate.src, global_value_factory)
        self._gen_rust_file("api/app-base/src/lib", app_base_crate.src, global_value_factory)
        self._gen_rust_file("api/app-base/src/macros", app_base_crate.src, global_value_factory)
        self._gen_rust_file("api/app-base/src/main_fn", app_base_crate.src, global_value_factory)
        self._gen_cargo_file("api/app-base", app_base_crate.root, global_value_factory)

        for app_crate in [core_app_crate, admin_app_crate]:
            self._gen_rust_file("api/app/src/main", app_crate.src, global_value_factory)
            self._gen_cargo_file("api/app", app_crate.root, global_value_factory)

        self._gen_cargo_file("", self._outdir(), global_value_factory)

    def _make_outdir(self):
        if self._outdir().exists():
            shutil.rmtree(self._outdir())
        self._outdir().mkdir()

    def _outdir(self) -> Path:
        return Path.cwd() / f"src/generated/{self._schema_filename}"

    def _mkdir(self, path: Path) -> Path:
        path.mkdir(parents=True, exist_ok=True)
        return path
    
    def _make_crate(self, path: str) -> CrateDirectories:
        root = self._mkdir(self._outdir() / path)
        src = self._mkdir(root / "src")
        return CrateDirectories(root, src)

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

    def _gen_cargo_file(self, template_dir: Path, outdir: Path, template_value_factory: TemplateValueFactory):
        template_dir = Path(template_dir)
        filename = "Cargo.toml"
        self._gen_file(template_dir / filename, outdir / filename, template_value_factory)

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
