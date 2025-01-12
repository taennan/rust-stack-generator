class TemplateKeyValPair:
    def __init__(self, key: str, val: str):
        self._key = key
        self._val = val

    def key(self) -> str:
        return self._key

    def val(self) -> str:
        return self._val
