#!/usr/bin/env python3

import browser_cookie3  # type: ignore
import os
import re
import json
import typer


LEETCODE_DOMAIN_NAME = "leetcode.com"

CSRFTOKEN_KEY = "csrftoken"
LEETCODE_SESSION_KEY = "LEETCODE_SESSION"


INITIAL_CONFIG_LOCATION = "./cli_config.toml"
TARGET_CONFIG_LOCATION = "~/.leetcode/leetcode.toml"
TARGET_CONFIG_LOCATION = os.path.expanduser(TARGET_CONFIG_LOCATION)

STUBS_LOCATION = "./stubs/"
CODE_LOCATION = "~/code/rust/leetcode/src"
CODE_LOCATION = os.path.expanduser(CODE_LOCATION)

CSRF_CONFIG_REGEX = re.compile(r"^csrf = \"<CSRFTOKEN>\"$", re.MULTILINE)
SESSION_CONFIG_REGEX = re.compile(r"^session = \"<LEETCODE_SESSION>\"$", re.MULTILINE)
LANGUAGE_CONFIG_REGEX = re.compile(r"^lang = \"<LANGUAGE>\"$", re.MULTILINE)
INJECT_BEFORE_CONFIG_REGEX = re.compile(
    r"^inject_before = \"<INJECT_BEFORE>\"$", re.MULTILINE
)
CODE_LOCATION_CONFIG_REGEX = re.compile(r"^code = \"<CODE_LOCATION>\"$", re.MULTILINE)

COMMENT_CONFIG_REGEX = re.compile(r"<COMMENT>")


def main(language: str = "rust") -> None:
    cj = browser_cookie3.chrome(domain_name=LEETCODE_DOMAIN_NAME)
    cookies = {c.name: c.value for c in cj}

    csrftoken = cookies[CSRFTOKEN_KEY]
    leetcode_session = cookies[LEETCODE_SESSION_KEY]

    language_stubs = []
    if os.path.exists(f"{STUBS_LOCATION}/{language}.txt"):
        language_stubs = [
            l.strip() for l in open(f"{STUBS_LOCATION}/{language}.txt").readlines()
        ]

    code_location = f"{CODE_LOCATION}/{language}_solutions"

    with open(INITIAL_CONFIG_LOCATION, "r") as f:
        content = f.read()

    content = CSRF_CONFIG_REGEX.sub(f'csrf = "{csrftoken}"', content)
    content = SESSION_CONFIG_REGEX.sub(f'session = "{leetcode_session}"', content)
    content = LANGUAGE_CONFIG_REGEX.sub(f'lang = "{language}"', content)
    content = INJECT_BEFORE_CONFIG_REGEX.sub(
        f"inject_before = {json.dumps(language_stubs)}", content
    )
    content = CODE_LOCATION_CONFIG_REGEX.sub(f'code = "{code_location}"', content)

    content = COMMENT_CONFIG_REGEX.sub("#" if language == "python" else "//", content)

    with open(TARGET_CONFIG_LOCATION, "w") as f:
        f.write(content)


if __name__ == "__main__":
    typer.run(main)
