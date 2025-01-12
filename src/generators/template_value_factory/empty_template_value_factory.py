from generators.template_value_factory.template_value_factory import TemplateValueFactory
from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair

class EmptyTemplateValueFactory(TemplateValueFactory):

    def keyvals(self) -> list[TemplateKeyValPair]:
        return []
