#!/bin/bash
#
# load cyw43 wireless firmware w/o bluetooth firmware 
# using this versus flashwb.sh can save ~16 KiB of ROM space if BT support is not needed
# update the const values in main.rs if you change these locations.
#
set -e


# 0x100000000 = ROM base
# Load Wifi firmware @ 1MiB mark, 256KiB hole...firmware is generally around 226KiB
probe-rs download ./include/cyw43-firmware/43439A0.bin \
  --binary-format bin --chip RP2040 --base-address 0x10100000

# load CLM firmware @ 1MiB + 256KiB, 984bytes ~ 1KiB
probe-rs download ./include/cyw43-firmware/43439A0_clm.bin \
  --binary-format bin --chip RP2040 --base-address 0x10140000

echo "Firmware flashed successfully!"
