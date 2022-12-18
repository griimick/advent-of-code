#include <string>
#include <vector>
#include <array>
#include <iostream>
#include <sstream>
#include <cstdlib>
#include <unordered_set>
#include <climits>
#include <stack>

#include "aoc_day_18.h"
#include "file_utils.h"

using namespace std;

AocDay18::AocDay18() : AocDay(0)
{
}

AocDay18::~AocDay18()
{
}

vector<string> AocDay18::read_input(string filename)
{
    FileUtils fileutils;
    vector<string> raw_lines;
    vector<string> data;
    if (!fileutils.read_as_list_of_strings(filename, raw_lines))
    {
        cerr << "Error reading in the data from " << filename << endl;
        return data;
    }
    for (vector<string>::iterator iter = raw_lines.begin(); iter != raw_lines.end(); ++iter)
    {
        data.push_back(*iter);
    }
    return data;
}

const vector<array<int, 3>> directions = {
    {1, 0, 0},
    {0, 1, 0},
    {0, 0, 1},
    {-1, 0, 0},
    {0, -1, 0},
    {0, 0, -1},
};

string AocDay18::part1(string filename, vector<string> extra_args)
{
    vector<string> data = read_input(filename);
    unordered_set<string> cubes(data.begin(), data.end());
    long sides = 0;
    for (auto &cube : cubes)
    {
        int connected = 0;
        stringstream ss(cube);
        int x, y, z;
        while (ss.good())
        {
            string substr;
            getline(ss, substr, ',');
            x = stol(substr);
            getline(ss, substr, ',');
            y = stol(substr);
            getline(ss, substr, ',');
            z = stol(substr);
        }
        for (auto &dir : directions)
        {
            if (cubes.find(to_string(x + dir[0]) + ',' + to_string(y + dir[1]) + ',' + to_string(z + dir[2])) != cubes.end())
            {
                connected++;
            }
        }
        sides += 6 - connected;
    }
    ostringstream out;
    out << sides;
    return out.str();
}

string AocDay18::part2(string filename, vector<string> extra_args)
{
    if (extra_args.size() > 0)
    {
        cout << "There are " << extra_args.size() << " extra arguments given:" << endl;
        for (vector<string>::iterator iter = extra_args.begin(); iter != extra_args.end(); ++iter)
        {
            cout << "[" << *iter << "]" << endl;
        }
    }
    vector<string> data = read_input(filename);
    unordered_set<string> cubes(data.begin(), data.end());
    /*
        Idea is to create a bounding box and to do dfs
        bounding box can be (max_x, max_y, max_z)
        similar to how water/vapour would envolope the the droplet
        and cannot reach closed air pockets
    */
    long xmax = LONG_MIN, ymax = LONG_MIN, zmax = LONG_MIN;
    long xmin = LONG_MAX, ymin = LONG_MAX, zmin = LONG_MAX;
    for (auto &cube : cubes)
    {
        stringstream ss(cube);
        long x, y, z;
        while (ss.good())
        {
            string substr;
            getline(ss, substr, ',');
            x = stol(substr);
            getline(ss, substr, ',');
            y = stol(substr);
            getline(ss, substr, ',');
            z = stol(substr);
        }
        xmax = max(xmax, x);
        ymax = max(ymax, y);
        zmax = max(zmax, z);
        xmin = min(xmin, x);
        ymin = min(ymin, y);
        zmin = min(zmin, z);
    }
    /* Following are the outer bounds */
    xmax += 1;
    ymax += 1;
    zmax += 1;
    xmin -= 1;
    ymin -= 1;
    zmin -= 1;

    long sides = 0;
    stack<array<long, 3>> stk;
    unordered_set<string> watered;
    stk.push({xmax, ymax, zmax});

    while (!stk.empty())
    {
        auto curr = stk.top();
        stk.pop();
        string pos = to_string(curr[0]) + ',' + to_string(curr[1]) + ',' + to_string(curr[2]);

        if (watered.find(pos) != watered.end())
        {
            continue;
        }

        watered.insert(pos);

        for (auto &dir : directions)
        {
            long x = curr[0] + dir[0];
            long y = curr[1] + dir[1];
            long z = curr[2] + dir[2];
            string pos = to_string(x) + ',' + to_string(y) + ',' + to_string(z);

            // out of bounds
            if (x > xmax || y > ymax || z > zmax || x < xmin || y < ymin || z < zmin)
            {
                continue;
            }

            // adjecent lava cube
            if (cubes.find(pos) != cubes.end())
            {
                sides++;
                //  not lava but not watered
            }
            else
            {
                stk.push({x, y, z});
            }
        }
    }
    ostringstream out;
    out << sides;
    return out.str();
}