# Some Notes for Part 2

```text
 aaaa
b    c
b    c
 dddd
e    f
e    f
 gggg
```

- Signal of length 2 maps to digit 1, but can't which signal maps to segment c
  and which to segment d
- Signal of length 3 maps to digit 7. The wire unique between 7 and 1 yields the
  wire driving segment a.
- Signal of length 4 maps to digit 4.
- Signal of length 5 maps to digits 2, 3, or 5.
- Signal of length 5 that contains the signal for digit 1, maps to digit 3.
  After removing the signal for digit 1, the common wire between signals 4 and 5
  drives segment d.
- Segment b is driven by the wire contained in signal for 4 that does not drive
  segment d nor the wires for signal 1
- Signal of length 5 that contains the wires driving segment a, b, and d maps to
  digit 5. The intersection of the signals for digit 5 and digit 1 gives the
  wire driving segment f. The remaining wire in the signal for digit 5 drives
  segment g.
- Resolving the wire driving segment f means the wire driving segment c can be
  determined from the signal mapping to digit 1.
- The wire driving segment e is the one wire remaining.
