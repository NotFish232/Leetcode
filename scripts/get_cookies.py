#!/usr/bin/env python3

import browser_cookie3  # type: ignore
import os
import re


LEETCODE_URL = "https://leetcode.com"

CSRFTOKEN_KEY = "csrftoken"
LEETCODE_SESSION_KEY = "LEETCODE_SESSION"


CONFIG_LOCATION = "~/.leetcode/leetcode.toml"
CONFIG_LOCATION = os.path.expanduser(CONFIG_LOCATION)

CSRF_CONFIG_REGEX = re.compile(r"^csrf = \".*\"$", re.MULTILINE)
SESSION_CONFIG_REGEX = re.compile(r"^session = \".*\"$", re.MULTILINE)


def main() -> None:
    cj = browser_cookie3.chrome(domain_name="leetcode.com")
    cookies = {c.name: c.value for c in cj}

    csrftoken = cookies[CSRFTOKEN_KEY]
    leetcode_session = cookies[LEETCODE_SESSION_KEY]

    with open(CONFIG_LOCATION, "r") as f:
        content = f.read()

    content = CSRF_CONFIG_REGEX.sub(f'csrf = "{csrftoken}"', content)
    content = SESSION_CONFIG_REGEX.sub(f'session = "{leetcode_session}"', content)

    with open(CONFIG_LOCATION, "w") as f:
        f.write(content)


if __name__ == "__main__":
    main()
