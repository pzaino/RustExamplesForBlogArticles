#!/bin/bash

# BUilds code to ASM
cargo rustc --release -- --emit asm
