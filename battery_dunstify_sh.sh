#!/bin/bash

#dunstify "title" "body"

alert_first=${1:-50}
alert_two=${2:-20}
time=${3:-60}

check_battery_config() {
  num=$1
  std_config=$2
  if [ $num -ge 100 ] || [ $num -le 1 ]; then
    echo $std_config
  else
    echo $num
  fi
}
oalert=$(check_battery_config $alert_first 50)
talert=$(check_battery_config $alert_two 20)
#oalert=100
#talert=100

if [ $time -le 0 ]; then
  time=60
fi

battery_alert1_action=0
battery_alert2_action=0
battery_life=0
while true; do
  battery_life=$(cat /sys/class/power_supply/BAT0/capacity)
  echo $battery_life

  if [ $battery_life -le $oalert ] && [ $battery_alert1_action == 0 ]; then
    dunstify "Battert alert" "battery low then $oalert" -u normal
    battery_alert1_action=1
  elif [ $battery_life -le $talert ] && [ $battery_alert2_action == 0 ]; then
    dunstify "Battert alert" "battery low then $talert" -u critical
    battery_alert2_action=1
  fi
  sleep $time
done
