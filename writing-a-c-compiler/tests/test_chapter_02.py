import pytest
from runner import TestRunner

runner = TestRunner("files/02")

execution_tests = {
    "bitwise_int_min.c": 2147483646,
    "bitwise_zero.c": -1,
    "bitwise.c": -13,
    "neg_zero.c": -0,
    "neg.c": -5,
    "negate_int_max.c": -2147483647,
    "nested_ops_2.c": 1,
    "nested_ops.c": 2,
    "parens_2.c": -3,
    "parens_3.c": 4,
    "parens.c": -2,
    "redundant_parens.c": -10,
}


@pytest.mark.parametrize("input,exit_code", [test for test in execution_tests.items()])
def test_chapter_02(input: str, exit_code: int):
    runner.test_asm_execution(input, exit_code)
