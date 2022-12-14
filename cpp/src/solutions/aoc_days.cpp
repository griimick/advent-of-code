#include <map>

#include "aoc_days.h"
#include "aoc_day.h"
// includes for each day will go here
#include "aoc_day_0.h"
#include "aoc_day_1.h"
#include "aoc_day_16.h"
#include "aoc_day_18.h"

using namespace std;

AocDays::AocDays()
{
    // adding each member to the map goes here
    m_days[0]=new AocDay0();
    m_days[1]=new AocDay1();
    m_days[16]=new AocDay16();
    m_days[18]=new AocDay18();
}

AocDays::~AocDays()
{
    // delete all of the days solutions
    for (map<int, AocDay *>::iterator days_iter = m_days.begin(); days_iter != m_days.end(); ++days_iter)
    {
        delete days_iter->second;
    }
    m_days.clear();
}

AocDay * AocDays::get_day(int day)
{
    return m_days[day];
}

