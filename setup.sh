#!/bin/bash

# Ensure script exits on errors
set -e

# Function to show usage
usage() {
    echo "Usage: $0 -n <day number> <problem name>"
    exit 1
}

# Parse options
while getopts "n:" opt; do
    case $opt in
        n) day_number=$OPTARG ;;
        *) usage ;;
    esac
done
shift $((OPTIND -1))

# Check arguments
if [ -z "$day_number" ] || [ -z "$1" ]; then
    usage
fi

problem_name="$1"
day_folder="src/day_$day_number"
day_name="Day $day_number: $problem_name"

# Create folder structure
mkdir -p "$day_folder"

cat <<EOL > "$day_folder/explanation.md"
# $day_name

## Input Bounds

## Part A

### Abridged Problem Statement

### Solution

### Code Complexity

**Time Complexity:** \$O(N)\$

**Additional Space Complexity:** \$O(N)\$

**Final answer:**

## Part B

### Abridged Problem Statement

### Solution

### Code Complexity

**Time Complexity:** \$O(N)\$

**Additional Space Complexity:** \$O(N)\$

**Final answer:**
EOL

# Part a setup
mkdir "$day_folder/day_${day_number}a/"
cat <<EOF > "$day_folder/day_${day_number}a/${day_number}a.rs"
// Implementation for Day ${day_number}, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    // let processed_input: Vec<Vec<i32>> = process_input(input, "i32");
    
    return input.join("\n");
}
EOF

# Part b setup
mkdir "$day_folder/day_${day_number}b/"
cat <<EOF > "$day_folder/day_${day_number}b/${day_number}b.rs"
// Implementation for Day ${day_number}, Part B
use crate::common::{get_input, process_input};
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    // let processed_input: Vec<Vec<i32>> = process_input(input, "i32");
    
    return input.join("\n");
}
EOF

cat <<EOF > "$day_folder/day_${day_number}a/test_cases.toml"
[[test_cases]]
input = "5"
output = "5"

[[test_cases]]
input = """
multi
line
test
"""
output = """multi
line
test"""
EOF

touch "$day_folder/day_${day_number}a/input.txt"

cp "$day_folder/day_${day_number}a/test_cases.toml" "$day_folder/day_${day_number}b/test_cases.toml"
touch "$day_folder/day_${day_number}b/input.txt"

# Ensure import.rs exists and modify it
import_rs="src/import.rs"

# If import.rs doesn't exist, create it
if [ ! -f "$import_rs" ]; then
    echo "// Auto-generated import.rs file" > "$import_rs"
fi

# Ensure module imports are at the top of the import.rs file

day_prev=$((day_number - 1))

if grep -q "mod day_${day_prev}b;" "$import_rs"; then
    sed -i "/mod day_${day_prev}b;/a \
#[path = \"day_${day_number}/day_${day_number}a/${day_number}a.rs\"]\n\
mod day_${day_number}a;\n\
#[path = \"day_${day_number}/day_${day_number}b/${day_number}b.rs\"]\n\
mod day_${day_number}b;" "$import_rs"
fi

if ! grep -q "mod day_${day_number}a;" "$import_rs"; then
    echo "#[path = \"day_${day_number}/day_${day_number}a/${day_number}a.rs\"]" >> "$import_rs"
    echo "mod day_${day_number}a;" >> "$import_rs"
fi

if ! grep -q "mod day_${day_number}b;" "$import_rs"; then
    echo "#[path = \"day_${day_number}/day_${day_number}b/${day_number}b.rs\"]" >> "$import_rs"
    echo "mod day_${day_number}b;" >> "$import_rs"
fi

# Now modify the function in import.rs to update the hashmap
hashmap_fn_name="get_day_solvers"
hashmap_fn_signature="fn ${hashmap_fn_name}() -> std::collections::HashMap<String, fn() -> String> {"

# Check if the function already exists
if grep -q "$hashmap_fn_signature" "$import_rs"; then
    # Append the new entries for day_a and day_b in the existing function
        sed -i "/map.insert(\"${day_prev}b\".to_string(), day_${day_prev}b::solve as fn() -> String);/a \
    \    map.insert(\"${day_number}a\".to_string(), day_${day_number}a::solve as fn() -> String);\n\
    map.insert(\"${day_number}b\".to_string(), day_${day_number}b::solve as fn() -> String);" "$import_rs"
else
    # If the function doesn't exist, create it
    cat <<EOF >> "$import_rs"

pub fn ${hashmap_fn_name}() -> std::collections::HashMap<String, fn()> {
    let mut map = std::collections::HashMap::new();
    map.insert("${day_number}a".to_string(), day_${day_number}a::solve as fn() -> String);
    map.insert("${day_number}b".to_string(), day_${day_number}b::solve as fn() -> String);
    map
}
EOF
fi

echo "Setup completed for $day_name!"
