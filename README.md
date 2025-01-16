# Rust Stack Generator

A simple cli tool to quickly generate a complete Rust GraphQL api, Services and Postgres database with custom entities

## Setup

Run the `setup.sh` script in the root of the project to create a virtual environment and install dependancies

## Usage

The repo comes with an example schema located at `src/schema/example-schema.yml`

Here is a brief explanaition of the configurations available:

- __project__: The name of the project
- __project_prefix__: (Optional) A prefix to be added to certain struct names found throughout a generated project
- __entities__: A list of objects which determine the database tables, services and endpoints to generate
- __entities.name__: The name of an entity
- __entities.fields__: A list of key value pairs which determine the name and type of each field to be generated for the entity

Run the entrypoint script and supply the name of the schema file (without file extension) to generate a complete Rust stack
 
```sh
# Using the example schema...
./main.sh --schema example-schema
```

The stack will be generated under the `src/generated` directory, you may then copy-paste the generated code into a new cargo workspace

### Notes:

- Schema files must be `yaml` fiels and must end with the `.yml` extension
- Schema files must be located under the `src/schema` directory
