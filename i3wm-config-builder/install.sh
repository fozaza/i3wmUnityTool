#!/bin/bash

cargo b -r &&
  sudo mv ./target/release/i3-config-builder /usr/bin/ &&
  cargo clean
