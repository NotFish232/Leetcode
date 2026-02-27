from common import *


# start_submission
class Solution:
    def simplifyPath(self, path: str) -> str:
        parts = path.split("/")

        stk = []

        for p in parts:
            if p in "/.":
                continue
            elif p == "..":
                if len(stk) > 0:
                    stk.pop()
            else:
                stk.append(p)

        return "/" if len(stk) == 0 else "".join("/" + p for p in stk)


# end_submission
