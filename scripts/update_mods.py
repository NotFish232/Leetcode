#!/usr/bin/env python3

from pathlib import Path
from natsort import natsorted
import re

SOLUTION_DIR = "src/solutions/"

MOD_PATH = "src/solutions/mod.rs"


MOD_REGEX = re.compile(r"//\s*mod_start.*//\s*mod_end", re.S)


def main() -> None:
    import_statements = []

    for solution_file in natsorted(Path(SOLUTION_DIR).glob("*.rs")):
        solution_file = Path(*solution_file.parts[2:])

        solution_path = solution_file.stem

        if solution_path == "mod":
            continue

        path_statement = f'#[path = "{solution_file}"]'
        mod_statement = f"mod s_{solution_path.replace('-', '_')};"

        import_statements.append(f"{path_statement}\n{mod_statement}\n")

    combined_import_statement = "".join(import_statements)

    with open(MOD_PATH, "r") as f:
        content = f.read()

    content = re.sub(
        MOD_REGEX,
        f"// mod_start\n{combined_import_statement}// mod_end",
        content,
    )

    with open(MOD_PATH, "w") as f:
        f.write(content)


if __name__ == "__main__":
    main()
