#!/bin/bash
# USAGE: ADVENT_TOKEN=<sesion> get_input.sh year day

echo $ADVENT_TOKEN

currentYear=$(date +%Y)
year=$1
if [[ "$year" -lt "2015" ]] || [[ "$year" -gt "$currentYear" ]]; then
    echo "Invalid year ($year). Years must be between 2015 and $currentYear" >&2
    exit 1
fi

day=$2
if [[ "$day" -lt "1" ]] || [[ "$day" -gt "25" ]]; then
    echo "Invalid day ($day). Days must be between 1 and 25" >&2
    exit 1
fi

urlPath="https://adventofcode.com/$year/day/$day/input"
echo "Downloading from $urlPath" >&2
curl --cookie "session=$ADVENT_TOKEN" "$urlPath"
