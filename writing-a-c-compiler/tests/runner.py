import os
import subprocess
from utils import to_int32


class Runner:
    path: str

    def __init__(self, path: str):
        self.path = path

    def test_asm_execution(self, file: str, exit_code: int):

        base_name = file.split(".")[0]
        asm = f"{self.path}/{base_name}.s"
        executable = f"{self.path}/{base_name}.out"

        subprocess.run(f"cargo run {self.path}/{file} {asm}", shell=True, check=True)
        subprocess.run(f"gcc {asm} -o {executable}", shell=True, check=True)

        try:
            win_executable = os.path.normpath(executable)

            program_result = subprocess.run(win_executable, shell=True)

            returncode = to_int32(program_result.returncode)

            assert returncode == exit_code, (
                f"Expected exit code {exit_code}, got {returncode}.\n"
                f"STDOUT:\n{program_result.stdout}\n"
                f"STDERR:\n{program_result.stderr}"
            )
        finally:
            if os.path.exists(executable):
                os.remove(executable)
