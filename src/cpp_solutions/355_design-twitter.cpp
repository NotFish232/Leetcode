#include <iterator>
#include <queue>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

// start_submission
class Twitter {
private:
    unordered_map<int, int> tweet_timestamps_;
    unordered_map<int, unordered_set<int>> user_followers_;
    unordered_map<int, vector<int>> user_tweets_;

    int timestamp_;

public:
    Twitter() : timestamp_{0} {
    }

    void postTweet(int userId, int tweetId) {
        tweet_timestamps_[tweetId] = ++timestamp_;

        user_tweets_[userId].push_back(tweetId);

        user_followers_[userId].insert(userId);
    }

    vector<int> getNewsFeed(int userId) {
        using iter = vector<int>::iterator;

        auto comp = [this](const tuple<iter, iter> &a, const tuple<iter, iter> &b) {
            return tweet_timestamps_[*get<0>(a)] < tweet_timestamps_[*get<0>(b)];
        };

        priority_queue<tuple<iter, iter>, vector<tuple<iter, iter>>, decltype(comp)> pq(comp);

        for (const int &user : user_followers_[userId]) {
            if (user_tweets_[user].size() > 0) {
                pq.push({std::prev(user_tweets_[user].end()), user_tweets_[user].begin()});
            }
        }

        vector<int> ans;

        while (pq.size() > 0 && ans.size() < 10) {
            auto [it, end] = pq.top();

            ans.push_back(*it);

            pq.pop();

            if (it != end) {
                pq.push({std::prev(it), end});
            }
        }

        return ans;
    }

    void follow(int followerId, int followeeId) {
        if (followerId != followeeId) {
            user_followers_[followerId].insert(followeeId);
        }
    }

    void unfollow(int followerId, int followeeId) {
        if (followerId != followeeId) {
            user_followers_[followerId].erase(followeeId);
        }
    }
};

/**
 * Your Twitter object will be instantiated and called as such:
 * Twitter* obj = new Twitter();
 * obj->postTweet(userId,tweetId);
 * vector<int> param_2 = obj->getNewsFeed(userId);
 * obj->follow(followerId,followeeId);
 * obj->unfollow(followerId,followeeId);
 */
// end_submission
