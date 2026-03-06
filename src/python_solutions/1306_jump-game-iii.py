from common import *


# start_submission
class Solution:
    def canReach(self, arr: List[int], start: int) -> bool:
        n = len(arr)

        q = deque()
        visited = set()

        q.append(start)
        visited.add(start)

        while len(q) > 0:
            i = q.popleft()

            if arr[i] == 0:
                return True

            if i - arr[i] >= 0 and i - arr[i] not in visited:
                q.append(i - arr[i])
                visited.add(i - arr[i])

            if i + arr[i] < n and i + arr[i] not in visited:
                q.append(i + arr[i])
                visited.add(i + arr[i])

        return False


# end_submission
