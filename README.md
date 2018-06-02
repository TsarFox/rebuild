Rebuild is an attempt at reimplementing Ken Silverman's Build engine, with the
goal of being modular enough to host modern source ports of any Build engine
game. The motivation comes from the fact that [BloodGDX][1], the current
recommended source port for Blood, is nonfree and written in Java. Additionally,
other source ports such as [EDuke32][2] are built upon Ken Silverman's codebase,
which is full of DOS-era optimizations like self-modifying code and a generous
amount of globally-shared state. They're also subject to the restrictions of the
BUILD license.


# Roadmap

- [ ] Implement support for GRP archives.
  - [x] GRP parser.
  - [x] Caching system.
  - [ ] Proper path resolution.
- [ ] Implement support for RTS. (?)
  - Appears to be a Doom iwad, though I'm not sure what its purpose is.
- [ ] Implement a timer system.
  - Needs to expose some sort of 'totalclock'.
- [ ] Implement support for ART bitmaps.
  - [ ] PALETTE.DAT parser.
  - [ ] ART parser.
  - [ ] Efficient ART-to-bitmap conversion.
- [ ] Implement support for Build's MAPs.
  - [ ] MAP parser.


[1]: https://blood-wiki.org/index.php/BloodGDX
[2]: http://eduke32.com/
