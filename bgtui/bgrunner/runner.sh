#!/bin/bash

imagePick=$(cat $HOME/.config/bgrunner/imagePick)
echo $imagePick

pkill feh
feh --bg-scale $HOME/.config/bgrunner/$imagePick

# if condition; then
#   command ...
# fijk
