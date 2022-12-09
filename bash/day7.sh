#!/bin/bash
if [ -z "$1" ]; then
    echo "Usage: $0 <day7_input_path>"
    exit 1
fi
INPUT_PATH=$1

ROOT_DIR=$(mktemp -d)
cd $ROOT_DIR

cat $INPUT_PATH | \
    sed "s|^\$ cd /$|\$ cd $ROOT_DIR|g" | \
    grep -vE '^\$ ls' | \
    sed -E 's|^\$ cd (.+)$|mkdir -p \1; cd \1|g' | \
    sed -E 's|^dir (.+)$|mkdir -p \1|g' | \
    sed -E 's|^([0-9]+) (.+)$|echo \1 > \2|g' | \
    bash

dirsize() {
    echo "$1 $(for f in $(find $1 -type f); do cat $f; done | awk '{s+=$1} END {print s}')"
}
export -f dirsize

THRESHOLD=100000

find -type d -exec bash -c 'dirsize "$0"' {} \; | \
    awk -F" " "{if(\$2<=$THRESHOLD)print\$2}" | \
    awk '{s+=$1} END {print s}'

rm -r $ROOT_DIR