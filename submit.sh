#!/usr/bin/env bash
set -euo pipefail

print_usage() {
  echo "submit.sh <part> <answer>"
}

if [ "$#" -ne 2 ]; then
  print_usage
  exit
fi

PART=$1
ANSWER=$2
DAY=$(date "+%-d")
YEAR=$(date "+%Y")
COOKIE=$(tr -d "\n" < cookie.txt)
URL="https://adventofcode.com/${YEAR}/day/${DAY}/answer"

curl "$URL" \
  -H 'content-type: application/x-www-form-urlencoded' \
  -H "cookie: session=${COOKIE}" \
  --data-raw "level=${PART}&answer=${ANSWER}" \
  --compressed
