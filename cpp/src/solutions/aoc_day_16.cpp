#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <cstdlib>
#include <algorithm>
#include <regex>
#include <climits>
#include <iomanip>

#include "aoc_day_16.h"
#include "file_utils.h"

using namespace std;

const regex input_regex("^Valve ([A-Z]+) has flow rate=([0-9]+); (tunnel leads|tunnels lead) to (valves|valve) (.*)$");
string trim(string s) {
    regex e("^\\s+|\\s+$");   // remove leading and trailing spaces
    return regex_replace(s, e, "");
}


AocDay16::AocDay16():AocDay(0)
{
}

AocDay16::~AocDay16()
{
}

vector<Room> AocDay16::read_input(string filename)
{
    FileUtils fileutils;
    vector<string> raw_lines;
    vector<Room> data;
    if (!fileutils.read_as_list_of_strings(filename, raw_lines))
    {
        cerr << "Error reading in the data from " << filename << endl;
        return data;
    }
    for (vector<string>::iterator iter = raw_lines.begin(); iter != raw_lines.end(); ++iter)
    {
        string line = *iter;
        smatch m;
        Room t;
        if (regex_match(line, m, input_regex) > 0) {
            // current room
            t.id = m[1];

            // room valve pressure release value
            t.valve = stol(m[2]);

            // parse the comma separated agjecent rooms
            stringstream next_substr(m[5]);
            vector<string> next;
            string str;

            while (next_substr.good()) {
                getline(next_substr, str, ',');
                next.push_back(trim(str));
            }

            // vector of pressure release valves
            t.next = next;
        }
        
        data.push_back(t);
    }
    return data;
}

// Idea to get all distances and optimise remaining time with pressure release
// Floyd Warshall for distance b/w all rooms
void allDistances(vector<vector<long>>& matrix) {
    int size = matrix.size();
    for (int k = 0; k < size; k++)
    {
        for (int i = 0; i < size; i++)
        {
            for (int j = 0; j < size; j++)
            {
                    matrix[i][j] = min(matrix[i][j], matrix[i][k] + matrix[k][j]);
            }
        }
    }
}

string AocDay16::part1(string filename, vector<string> extra_args)
{
    long result = 0;
    vector<Room> rooms = read_input(filename);

    long roomsCount = 0;

    for (auto& room: rooms) {
        roomIndex[room.id] = roomsCount++;
        states.push_back(1<<roomIndex[room.id]);
        valve.push_back(room.valve);
    }

    adjMat = vector<vector<long>>(roomsCount, vector<long>(roomsCount, 9999999999));

    for (auto& room: rooms) {
        for (auto& n: room.next) {
            cout << room.id << ' ' << n << endl;
            adjMat[roomIndex[room.id]][roomIndex[n]] = 1;
        }
    }

    cout << roomsCount << " : roomsCount" << endl;

    // for (auto& t : adjMat) {
    //     for (auto& g : t) {
    //         cout << setw(5) << setfill('0') << g << ' ';
    //     }
    //     cout << endl;
    // }
    // cout << "allDistances" << endl;

    allDistances(adjMat);
    // for (auto& t : adjMat) {
    //     for (auto& g : t) {
    //         cout << setw(5) << setfill('0') << g << ' ';
    //     }
    //     cout << endl;
    // }


    unordered_map<long,long> dp;
    permutate(30, 0, 0, 0, dp);

    for (auto& d : dp) {
        result = max(result, d.second);        
    } 

    ostringstream out;
    out << result;
    return out.str();
}

// we go to all the rooms and keep track of best netRelease we get in a certain set of visited rooms
// visited rooms are represented in state format, basically using bits each roomIndex is if visited sets a bit
//  long state should give us room for 64 rooms or we can go for bitset 
void AocDay16::permutate(long remainingTime, long currIdx, long netRelease, long state, unordered_map<long, long>& dp)
{
    dp[state] =  max(dp[state], netRelease);

    for (auto& v : roomIndex) {
        int newRemainingTime = remainingTime - adjMat[currIdx][v.second] - 1; // -1 for taking an action

        // 
        if ((states[v.second] & state) or newRemainingTime <= 0) {
            continue;
        }

        AocDay16::permutate(newRemainingTime, v.second, netRelease + newRemainingTime * valve[v.second], state | states[v.second], dp);
    }
}

string AocDay16::part2(string filename, vector<string> extra_args)
{
    if (extra_args.size() > 0)
    {
        cout << "There are " << extra_args.size() << " extra arguments given:" << endl;
        for (vector<string>::iterator iter = extra_args.begin(); iter != extra_args.end(); ++iter)
        {
            cout << "[" << *iter << "]" << endl;
        }
    }
    
    long sum;
    ostringstream out;
    out << sum;
    return out.str();
}
