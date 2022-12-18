#ifndef __AOC_DAY_16__
#define __AOC_DAY_16__

#include "aoc_day.h"

struct Room {
    string id;
    long valve;
    vector<string> next;

};

class AocDay16 : public AocDay
{
    private:
        vector<Room> read_input(string filename);
        unordered_map<string, long> roomIndex;
        vector<long> valve;
        vector<long> states;
        vector<vector<long>> adjMat;
        void permutate(long remainingTime, long currIdx, long netRelease, long state, unordered_map<long ,long>& dp);

    public:
        AocDay16();
        ~AocDay16();
        string part1(string filename, vector<string> extra_args);
        string part2(string filename, vector<string> extra_args);
};


#endif
