# IsItUp
Automatically get an HTTP response code for a given list of subdomains, useful for validating quickly which targets are live.

# Installation
Simply clone the repository, compile the tool and copy the binary to your bin directory

``` bash
git clone https://github.com/XoanOuteiro/IsItUp
cd ./IsItUp/IsItUp
cargo build
cp ./target/debug/IsItUp /usr/bin/
```

# Usage
Simply pass a file path as an argument

``` bash
IsItUp [filepath]
```
