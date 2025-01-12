from generators.template_value_factory.template_key_val_pair import TemplateKeyValPair

class TemplateValueFactory:

    def keyvals(self) -> list[TemplateKeyValPair]:
        raise Exception("This must be overridden in subclasses")
