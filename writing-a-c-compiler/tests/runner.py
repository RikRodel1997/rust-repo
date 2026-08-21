import os
import subprocess
from utils import to_int32


class TestRunner:
    path: str

    def __init__(self, path: str):
        self.path = path

    def test_asm_execution(self, input: str, exit_code: int):
        base_name = input.split(".")[0]
        input_path = f"{self.path}/{base_name}.s"
        executable = f"{self.path}/{base_name}.out"

        subprocess.run(f"gcc {input_path} -o {executable}", shell=True, check=True)

        try:
            win_executable = os.path.normpath(executable)

            program_result = subprocess.run(win_executable, shell=True)

            returncode = to_int32(program_result.returncode)

            assert returncode == exit_code
        finally:
            if os.path.exists(executable):
                os.remove(executable)
