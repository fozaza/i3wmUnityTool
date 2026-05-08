#!/bin/bash

time=${1:-5}
btCh_status=""
btCh_status_old=""

while true; do
  btCh_status=$(cat /sys/class/power_supply/BAT0/status | xargs)
  if [ "$btCh_status_old" != "$btCh_status" ]; then
    btCh_status_old=$btCh_status
    dunstify "battey charger state" $btCh_status_old
  fi
  sleep $time
done
