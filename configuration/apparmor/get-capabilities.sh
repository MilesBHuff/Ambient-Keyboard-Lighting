#!/bin/sh
exec capsh --print | grep Bounding | sed 's/^Bounding set *= */,/' | sed 's/,cap_/\ncapability /g' | grep capability | sort | sed 's/$/,/gm'
