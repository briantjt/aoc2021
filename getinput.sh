#!/usr/bin/env bash
set -euo pipefail

DAY=$(date "+%-d")
YEAR=$(date "+%Y")
URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"
COOKIE=$(tr -d "\n" < cookie.txt)
echo "Getting input for day ${DAY}: ${URL}"
mkdir -p "src/Day${DAY}"

curl -f "https://adventofcode.com/${YEAR}/day/${DAY}/input" \
  -H "accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9" \
  -H "cookie: session=${COOKIE}" \
  --compressed > "src/Day${DAY}/input.txt"

STARTER_CODE=$(cat <<EOF
fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").lines();
    Ok(())
}

EOF
)

echo "Creating new main.rs file"
echo -n "$STARTER_CODE" > "src/Day${DAY}/main.rs"

echo "Adding new bin to Cargo.tml"
NEW_TOML_BIN=$(cat <<EOF

[[bin]]
name = "day${DAY}"
path = "src/Day${DAY}/main.rs"

EOF
)

echo -n "$NEW_TOML_BIN" >> Cargo.toml

echo "Done!"
