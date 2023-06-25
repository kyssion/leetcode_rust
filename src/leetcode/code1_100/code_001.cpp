//
// Created by kyssion on 23-6-25.
//
#include <bits/stdc++.h>
using namespace  std;
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int,int>hashtable;
        for(int i=0;i<nums.size();i++){
            aslouto it=hashtable.find(target-nums[i]);
            if(it!=hashtable.end()){
                return{it->second,i};
            }
            hashtable[nums[i]]=i;
        }
        return {};
    }
};
int main(){
    Solution sl;
    vector<int> v = {1,2,3,4,5};
    vector<int> ant = sl.twoSum(v,3);
    std::for_each(ant.begin(), ant.end(), [](const auto &i){std::cout << i << " "; });
    std::cout << "\n";
    return 0;
}