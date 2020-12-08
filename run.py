import os
import subprocess


def main():
    intro_file = "intro.md"
    readme_file = "README.md"
    cargo_cmd = "cargo run --release"
    
    with open(intro_file, "r") as f:
        intro = f.read()
    cargo_out = subprocess.check_output(cargo_cmd).decode("utf-8") 
    with open(readme_file, "w") as f:
        f.write(f"{intro}\n```\n{cargo_out}```\n")


if __name__ == "__main__":
    main()
