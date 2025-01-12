from pathlib import Path
from generators.template_value_factory.template_value_factory import TemplateValueFactory

class FileTemplateValueInjector:

    def __init__(self, template_name: str, template_value_factory: TemplateValueFactory):
        self._template_name = template_name
        self._template_value_factory = template_value_factory

    def make_string(self) -> str:
        template_path = Path.cwd() / f"src/templates/{self._template_name}"
        template = template_path.read_text()

        for template_keyval in self._template_value_factory.keyvals():
            key = "{" + template_keyval.key() + "}"
            template = template.replace(key, template_keyval.val())

        return template
