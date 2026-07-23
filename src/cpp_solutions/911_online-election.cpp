#include <algorithm>
#include <iterator>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

 // start_submission
class TopVotedCandidate {
private:
    vector<tuple<int, int>> results_;
    
public:
    TopVotedCandidate(vector<int>& persons, vector<int>& times) {
        unordered_map<int, int> num_votes;
        
        int cur;
        int max_votes = 0;

        for (int i = 0; i < times.size(); ++i) {
            int &votes = num_votes[persons[i]];
            ++votes;

            if (votes >= max_votes) {
                cur = persons[i];
                max_votes = votes;
            }

            results_.push_back({times[i], cur});
        }
    }
    
    int q(int t) {
        auto it = upper_bound(results_.begin(), results_.end(), t, [](const int &tgt, const tuple<int, int> &el) {
            return tgt < get<0>(el);
        });

        return get<1>(*prev(it));
    }
};

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * TopVotedCandidate* obj = new TopVotedCandidate(persons, times);
 * int param_1 = obj->q(t);
 */
 // end_submission
