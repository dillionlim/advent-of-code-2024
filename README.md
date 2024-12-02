# Advent of Code 2024 Automated Directory Setup (Rust)

## Rust Installation

Rust should be installed on your system already. Also, make sure that you have a C linker installed, usually doing 
```
sudo apt-get update
sudo apt install build-essential
sudo apt install gcc
```
will work. Then, check that it is installed with
```
cc --version
```

## Running the directory setup

Enable the script to be run by running:
```
chmod +x setup.sh
```

Then, create a new directory for the problems in day $x$ by doing 
```
./setup.sh -n <x>
```

In general, I like to use the convention $x$ a for the first star problem, and $x$ b for the second star problem. So, the two problems on day 1 would be set up with $1a$ and $1b$.