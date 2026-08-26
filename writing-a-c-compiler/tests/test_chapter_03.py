import pytest
from runner import Runner

runner = Runner("files/03")

execution_tests = {
    "add.c": 3,
    "associativity.c": -4,
    "associativity_2.c": 1,
    "associativity_3.c": 8,
    "associativity_and_precedence.c": 10,
    "bitwise_and.c": 1,
    "bitwise_or.c": 3,
    "bitwise_precedence.c": 21,
    "bitwise_shiftl.c": 140,
    "bitwise_shiftr.c": 62,
    "bitwise_shiftr_negative.c": 3,
    "bitwise_shift_associativity.c": 132,
    "bitwise_shift_associativity_2.c": 16,
    "bitwise_shift_precedence.c": 1310720,
    "bitwise_variable_shift_count.c": 76,
    "bitwise_xor.c": 6,
    "div.c": 2,
    "div_neg.c": -2,
    "mod.c": 0,
    "mult.c": 6,
    "parens.c": 14,
    "precedence.c": 14,
    "sub.c": -1,
    "sub_neg.c": 3,
    "unop_add.c": 0,
    "unop_parens.c": -3,
}


@pytest.mark.parametrize("input,exit_code", [test for test in execution_tests.items()])
def test_chapter_03(input: str, exit_code: int):
    runner.test_asm_execution(input, exit_code)
