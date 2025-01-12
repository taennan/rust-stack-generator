from pathlib import Path

def try_append_path_suffix(path, suffix: str) -> Path:
    path = str(path)
    suffix = f".{suffix}" if not suffix.startswith(".") else suffix

    if not path.endswith(suffix):
        return Path(f"{path}{suffix}")
    return Path(path)
