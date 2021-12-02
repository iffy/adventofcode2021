import std/os
import std/strutils
import std/strformat

var depth = 0
var forward = 0
var aim = 0

for line in open(paramStr(1)).lines:
  let parts = line.split(" ")
  let amount = parts[1].parseInt()
  case parts[0]
    of "forward":
      forward.inc amount
      depth.inc amount * aim
    of "down": aim.inc amount
    of "up": aim.dec amount
    else: discard

echo &"depth {depth} forward {forward} answer {depth*forward}"
