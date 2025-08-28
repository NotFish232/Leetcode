#!/usr/bin/env python3

import browser_cookie3  # type: ignore
import os
import re


LEETCODE_DOMAIN_NAME = "eetcode.com"

CSRFTOKEN_KEY = "csrftoken"
LEETCODE_SESSION_KEY = "LEETCODE_SESSION"


INITIAL_CONFIG_LOCATION = "./cli_config.toml"
TARGET_CONFIG_LOCATION = "~/.leetcode/leetcode.toml"
TARGET_CONFIG_LOCATION = os.path.expanduser(TARGET_CONFIG_LOCATION)

CSRF_CONFIG_REGEX = re.compile(r"^csrf = \"<csrftoken>\"$", re.MULTILINE)
SESSION_CONFIG_REGEX = re.compile(r"^session = \"<LEETCODE_SESSION>\"$", re.MULTILINE)


def main() -> None:
    cj = browser_cookie3.chrome(domain_name=LEETCODE_DOMAIN_NAME)
    cookies = {c.name: c.value for c in cj}

    csrftoken = cookies[CSRFTOKEN_KEY]
    leetcode_session = cookies[LEETCODE_SESSION_KEY]

    with open(INITIAL_CONFIG_LOCATION, "r") as f:
        content = f.read()

    content = CSRF_CONFIG_REGEX.sub(f'csrf = "{csrftoken}"', content)
    content = SESSION_CONFIG_REGEX.sub(f'session = "{leetcode_session}"', content)

    with open(TARGET_CONFIG_LOCATION, "w") as f:
        f.write(content)


if __name__ == "__main__":
    main()
