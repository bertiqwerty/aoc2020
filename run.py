import os
import subprocess
import numpy
import random


def _obfuscate(line):
    def obfuscate_str(number: str, obf_char="#"):
        pos = random.randint(0, len(number) - 1)
        res = "".join([c if i != pos else obf_char for i, c in enumerate(number)])
        return res

    return ", ".join([obfuscate_str(line.strip()) for line in line.split(",")])


def main():
    intro_file = "intro.md"
    readme_file = "README.md"
    cargo_cmd = "cargo run --release"

    with open(intro_file, "r") as f:
        intro = f.read()
    rust_out = subprocess.check_output(cargo_cmd).decode("utf-8")
    print(rust_out)
    rust_out_lines_obfuscated = [
        _obfuscate(line.strip()) if i % 5 == 0 else line
        for i, line in enumerate(rust_out.split("\n")[3:])
    ]
    rust_out_obfuscated = "\n".join(rust_out_lines_obfuscated)
    with open(readme_file, "w") as f:
        f.write(f"{intro}\n```\n{rust_out_obfuscated}```\n")


if __name__ == "__main__":
    main()
