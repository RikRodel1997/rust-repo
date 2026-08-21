import os


def to_int32(val: int) -> int:
    val = val & 0xFFFFFFFF
    if val >= 0x80000000:
        return val - 0x100000000
    return val


def define_cases(path: str, tests: dict[str, int]) -> list[tuple[str, int]]:
    return [
        (f, tests[f"{path}/{f}"])
        for f in os.listdir(path)
        if f.endswith(".c") and f"{path}/{f}" in tests
    ]
