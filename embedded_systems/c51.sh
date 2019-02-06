#!/bin/bash
sdcc $1.c && packihx $1.ihx > $1.hex
