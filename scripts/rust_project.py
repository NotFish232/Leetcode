#!/bin/python3

from pathlib import Path
import json
from natsort import natsorted


crates = []


for file in natsorted(Path(__file__).parents[1].glob("*.rs")):
    crates.append({
            "root_module": f"./{file.name}",
            "edition": "2021",
            "deps": []
    })

rust_project_json = {
    "sysroot_src": "!/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library",
    "crates": crates
}

with open("./rust-project.json", "w+") as f:
    json.dump(rust_project_json, f, indent=4)