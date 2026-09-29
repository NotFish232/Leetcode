#include <vector>

using namespace std;

 // start_submission
 class Solution {
 public:
     int findMin(vector<int> &nums) {
         int l = 0, r = nums.size() - 1;
 
         int a = nums.front(), b = nums.back();
 
         int ans = a < b ? 0: (nums.size() - 1);
 
         while (l <= r) {
             int m = l + (r - l) / 2;
 
             if (nums[m] >= b) {
                 l = m + 1;
             } else {
                 r = m - 1;
                 ans = m;
             }
         }
 
         return nums[ans];
     }
 };
 // end_submission
