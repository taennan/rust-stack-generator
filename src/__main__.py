from argparse import ArgumentParser
from generators.schema_template_generator import SchemaTemplateGenerator

def main():
    parser = ArgumentParser(description='Generate Rust stack files')
    parser.add_argument("--schema", help="specify schema filename")
    args = parser.parse_args()
    SchemaTemplateGenerator(args.schema).make_files()

if __name__ == "__main__":
    main()
