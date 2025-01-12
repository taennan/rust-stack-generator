from pathlib import Path
from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.file_generator.file_template_value_injector import FileTemplateValueInjector

class FileTemplateGenerator:

    def __init__(
        self,
        template_name: str,
        output_path: Path,
        template_value_factory: TemplateValueFactory
    ):
        self._template_name = template_name
        self._output_path = output_path
        self._template_value_factory = template_value_factory

    def make_file(self):
        filled_template = FileTemplateValueInjector(self._template_name, self._template_value_factory).make_string()
        self._output_path.write_text(filled_template)
