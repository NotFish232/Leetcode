#include <bits/stdc++.h>

using namespace std;

 // start_submission
 class Solution {
 public:
     string predictPartyVictory(string senate) {
         int n = senate.size();
 
         int r_cnt = count(senate.begin(), senate.end(), 'R');
         int d_cnt = n - r_cnt;
 
         queue<char> q(senate.begin(), senate.end());
 
         int acc = 0;
 
         while (r_cnt > 0 && d_cnt > 0) {
             char c = q.front();
 
             if (acc == 0) {
                 q.pop();
 
                 acc = c == 'R' ? 1 : -1;
 
                 q.push(c);
             } else {
                 if (acc > 0) {
                     if (c == 'R') {
                         q.push(q.front());
                         q.pop();
                         ++acc;
                     } else {
                         --acc;
                         q.pop();
                         --d_cnt;
                     }
                 } else {
                     if (c == 'D') {
                         q.push(q.front());
                         q.pop();
                         --acc;
                     } else {
                         ++acc;
                         q.pop();
                         --r_cnt;
                     }
                 }
             }
         }
 
         return r_cnt != 0 ? "Radiant" : "Dire";
     }
 };
 // end_submission
