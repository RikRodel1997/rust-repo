import pytest
from runner import Runner

runner = Runner("files/01")

execution_tests = {
    "multi_digit.c": 100,
    "newlines.c": 0,
    "no_newlines.c": 0,
    "return_0.c": 0,
    "return_2.c": 2,
    "spaces.c": 0,
    "tabs.c": 0,
}


@pytest.mark.parametrize("input,exit_code", [test for test in execution_tests.items()])
def test_chapter_01(input: str, exit_code: int):
    runner.test_asm_execution(input, exit_code)
