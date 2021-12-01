import std/os
import std/strutils

var
  lastval = 0
  started = false
  incs = 0
for line in open(paramStr(1)).lines:
  var thisval = line.parseInt()
  if started and thisval > lastval:
    inc incs
  if not started:
    started = true
  lastval = thisval
echo incs