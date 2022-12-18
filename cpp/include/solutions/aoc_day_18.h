#ifndef __AOC_DAY_18__
#define __AOC_DAY_18__

#include "aoc_day.h"

struct Cube {
    long x;
    long y;
    long z;
};

class AocDay18 : public AocDay
{
    private:
        vector<string> read_input(string filename);
    public:
        AocDay18();
        ~AocDay18();
        string part1(string filename, vector<string> extra_args);
        string part2(string filename, vector<string> extra_args);
};


#endif
