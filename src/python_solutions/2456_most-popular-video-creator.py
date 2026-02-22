from common import *


# start_submission
class Solution:
    def mostPopularCreator(
        self, creators: List[str], ids: List[str], views: List[int]
    ) -> List[List[str]]:
        n = len(creators)

        creator_popularity = {}
        creator_videos = {}

        for c, v, id in zip(creators, views, ids):
            if c not in creator_popularity:
                creator_popularity[c] = 0
            creator_popularity[c] += v

            if c not in creator_videos:
                creator_videos[c] = []
            creator_videos[c].append((-v, id))

        max_views = max(creator_popularity.values())

        return [
            [k, min(creator_videos[k])[1]]
            for k, v in creator_popularity.items()
            if v == max_views
        ]


# end_submission
