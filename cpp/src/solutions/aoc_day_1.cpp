#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <cstdlib>
#include <algorithm>

#include "aoc_day_1.h"
#include "file_utils.h"

using namespace std;

AocDay1::AocDay1():AocDay(0)
{
}

AocDay1::~AocDay1()
{
}

vector<long> AocDay1::read_input(string filename)
{
    FileUtils fileutils;
    vector<string> raw_lines;
    vector<long> data;
    if (!fileutils.read_as_list_of_strings(filename, raw_lines))
    {
        cerr << "Error reading in the data from " << filename << endl;
        return data;
    }
    long bucket = 0;
    for (vector<string>::iterator iter = raw_lines.begin(); iter != raw_lines.end(); ++iter)
    {
        string to_convert = *iter;
        if (!to_convert.empty())
        {
            bucket += strtol(to_convert.c_str(), NULL, 10);
        } else {
            data.push_back(bucket);
            bucket = 0;
        }
    }
    data.push_back(bucket);
    return data;
}

string AocDay1::part1(string filename, vector<string> extra_args)
{
    vector<long> data = read_input(filename);
    long sum = *max_element(data.begin(), data.end());
    ostringstream out;
    out << sum;
    return out.str();
}

string AocDay1::part2(string filename, vector<string> extra_args)
{
    if (extra_args.size() > 0)
    {
        cout << "There are " << extra_args.size() << " extra arguments given:" << endl;
        for (vector<string>::iterator iter = extra_args.begin(); iter != extra_args.end(); ++iter)
        {
            cout << "[" << *iter << "]" << endl;
        }
    }
    
    vector<long> data = read_input(filename);
    sort(data.begin(), data.end(), greater<long>());
    long sum = data[0] + data[1] + data[2];
    ostringstream out;
    out << sum;
    return out.str();
}
