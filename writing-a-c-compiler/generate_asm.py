import os
import subprocess


def generate_asm_files(dir_path: str):
    for file_name in os.listdir(dir_path):
        if file_name.endswith(".c"):
            base_name = os.path.splitext(file_name)[0]
            out = os.path.join(dir_path, f"{base_name}.s")

            subprocess.run(
                f"cargo run {dir_path}/{file_name} {out}", shell=True, check=True
            )


generate_asm_files("files/04")
