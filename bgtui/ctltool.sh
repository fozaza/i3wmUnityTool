#!/bin/bash

arg=$1

if [[ "$arg" == "h" || "$arg" == "help" ]]; then
  echo "help"
  echo -e "\t h help: this command help for u"
  echo -e "\t Ext. : ctltool.sh i & ctltool.sh ib\n"
  echo -e "\t i install : install tui app"
  echo -e "\t ib installBgrunner : install bgrunner app"
  echo -e "\t u uninstall : uninstall tui app"

elif [[ "$arg" == "v" || "$arg" == "version" ]]; then
  echo "version : v0.1"

elif [[ "$arg" == "i" || "$arg" == "install" ]]; then
  echo "install bgtui"
  cargo b -r

  mv ./target/release/bgtui ./
  sudo mv ./bgtui /usr/local/bin/bgtui
  cargo clean

elif [[ "$arg" == "ib" || "$arg" == "installBgrunner" ]]; then
  echo "install bgrunner"
  mkdir ~/.config -p
  cp -r ./bgrunner/ ~/.config/

elif [[ "$arg" == "u" || "$arg" == "uninstall" ]]; then
  echo "uninstall bgtui"
  sudo rm /usr/local/bin/bgtui

else
  echo "not found arg "$arg "  ctltool h : can help you"
  exit 1

fi
