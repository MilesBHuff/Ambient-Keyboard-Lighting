#!/bin/sh
## Figures out what packages an executable depends on.
if [ ! -f "$1" ]; then
    echo "'$1' not found!" >&2
    exit 1
fi
pacman -Qo `
    ldd "$1" |\
    grep '/usr/' |\
    sed -r 's/^.+?(\/usr\/.+?) .+?$/\1/gm' |\
    xargs
` |\
    sed -r 's/^.+ ([^ ]+?) ([^ ]+?)$/\1/gm' |\
    sort |\
    uniq |\
    xargs |\
    sed -r 's/([^ ]+)/'\''\1'\''/g'
exit 0
