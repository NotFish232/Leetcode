#include <algorithm>
#include <bits/stdc++.h>
#include <limits>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

// start_submission
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
    ListNode *helper(ListNode *cur, int &mx) {
        if (cur == nullptr) {
            return nullptr;
        }

        ListNode *nxt = helper(cur->next, mx);

        mx = max(mx, cur->val);

        if (cur->val < mx) {
            return nxt;
        }

        cur->next = nxt;

        return cur;
    }

public:
    ListNode *removeNodes(ListNode *head) {
        int mx = numeric_limits<int>::min();

        return helper(head, mx);
    }
};
// end_submission
