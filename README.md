An attempt at reimplementing Ken Silverman's Build engine, with the goal of
being modular enough to host a modern Blood source port. The motivation comes
from the fact that [BloodGDX][1], the current source port being recommended by
many, is nonfree, and written in Java.

Additionally, existing source ports such as [EDuke32][2] which are built upon
Ken Silverman's codebase are filled with legacy cruft and are subject to the
restrictions of the BUILD license.

No, I'm not on a zealous [RIIR][3] crusade. I'm mostly using this as an
opportunity to learn about Rust and its ecosystem. Even if you have reservations
about Rust, I'm willing to bet that you'd still prefer this to something written
in Java.


# Roadmap

- [ ] Implement usable parsers for the various Build engine formats.
  - [x] Implement a GRP loader.
  - [ ] Implement a MAP parser.
  - [ ] Implement an ART parser.
  - [ ] Implement a VOX parser.
  - [ ] Implement a PALETTE.DAT parser.
  - [ ] Implement a TABLES.DAT parser.
- [ ] Get Jon St. John to insult me in person (in Duke's voice).


[1]: https://blood-wiki.org/index.php/BloodGDX
[2]: http://eduke32.com/
[3]: https://github.com/ansuz/RIIR
