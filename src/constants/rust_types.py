class RustTypes:
    STRING = "String"
    UUID = "Uuid"

    _NUMBER_TYPES = ["usize", "isize", "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
    _TIME_TYPES = ["NaiveDateTime", "DateTime"]
    _ITERABLE_TYPES = [STRING]

    _EXACT_TYPES = ["Uuid"]
    _RANGED_TYPES = [*_TIME_TYPES, *_NUMBER_TYPES]

    @staticmethod
    def _is_one_of_types_or_optional(type_name: str, types: list[str]) -> bool:
        for compared_type in types:
            if RustTypes.is_type_or_optional(type_name, compared_type):
                return True
        return False
    
    @staticmethod
    def is_optional(type_name: str) -> bool:
        return type_name.startswith("Option<") and type_name.endswith(">")
    
    @staticmethod
    def is_type_or_optional(type_name: str, compared_type: str) -> bool:
        return type_name == compared_type or type_name == f"Option<{compared_type}>"

    @staticmethod
    def is_number_type(type_name: str) -> bool:
        return RustTypes._is_one_of_types_or_optional(type_name, RustTypes._NUMBER_TYPES)
    
    @staticmethod
    def is_time_type(type_name: str) -> bool:
        return RustTypes._is_one_of_types_or_optional(type_name, RustTypes._TIME_TYPES)
    
    @staticmethod
    def is_exact_type(type_name: str) -> bool:
        return RustTypes._is_one_of_types_or_optional(type_name, RustTypes._EXACT_TYPES)

    @staticmethod
    def is_ranged_type(type_name: str) -> bool:
        return RustTypes._is_one_of_types_or_optional(type_name, RustTypes._RANGED_TYPES)

    @staticmethod
    def is_iterable_type(type_name: str) -> bool:
        optional_types = [f"Option<{t}>" for t in RustTypes._ITERABLE_TYPES]
        return type_name in RustTypes._ITERABLE_TYPES or "Vec<" in type_name or type_name in optional_types
