#!/bin/bash

E_BADARGS=85
min_year=2015
max_year=2023

if [ ! -n "$1" ]
then
      echo "Usage: `basename $0` year"
      exit $E_BADARGS
fi

year=$1
if !(( year >= min_year && year <= max_year ))
then
    echo "invalid year: $min_year <= year <= $max_year"
fi

url="https://adventofcode.com/$year/day/{DAY}/input"
input="./input/day{DAY}.in"

for i in {1..25}
do
    day=$(printf "%02d" $i)
    input_url=${url/\{DAY\}/$i}
    input_file=${input/\{DAY\}/$day}
    echo "Downloading day $i input..."
    curl --cookie "session=$AOC_TOKEN" "$input_url" -o "$input_file"
    echo "saved: $input_file"
    echo
done
