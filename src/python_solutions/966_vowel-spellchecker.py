from common import *

# start_submission
class Solution(object):
    @staticmethod
    def p1(w):
        return w.lower()

    @staticmethod
    def p2(w):
        vowels = "aeiou"

        for v in vowels:
            w = w.replace(v, "*")

        return w

    def spellchecker(self, wordlist, queries):
        """
        :type wordlist: List[str]
        :type queries: List[str]
        :rtype: List[str]
        """

        d1 = {}
        d2 = {}
        d3 = {}

        for w in wordlist:
            r1 = Solution.p1(w)
            r2 = Solution.p2(r1)

            if w not in d1:
                d1[w] = []
            d1[w].append(w)

            if r1 not in d2:
                d2[r1] = []
            d2[r1].append(w)

            if r2 not in d3:
                d3[r2] = []
            d3[r2].append(w)

        res = []

        for q in queries:
            r1 = Solution.p1(q)
            r2 = Solution.p2(r1)

            if q in d1:
                res.append(d1[q][0])
            elif r1 in d2:
                res.append(d2[r1][0])
            elif r2 in d3:
                res.append(d3[r2][0])
            else:
                res.append("")

        return res


# end_submission
