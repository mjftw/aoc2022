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

echo -n "Day 7 part 1 answer: "
find -type d -exec bash -c 'dirsize "$0"' {} \; | \
    awk -F' ' '{if($2<=100000)print$2}' | \
    awk '{s+=$1} END {print s}'

DISK_SIZE=70000000
DISK_NEEDED=30000000
TOTAL_DISK_USED=$(dirsize $ROOT_DIR | awk '{s+=$2} END {print s}')
EXTRA_SIZE_NEEDED=$(($DISK_NEEDED - ($DISK_SIZE - $TOTAL_DISK_USED)))

echo -n "Day 7 part 2 answer: "
find -type d -exec bash -c 'dirsize "$0"' {} \; | \
    awk -F' ' "{if(\$2>=$EXTRA_SIZE_NEEDED)print \$0}" | \
    sort -k2 -n | \
    head -n 1 | \
    cut -d" " -f2

rm -r $ROOT_DIR
