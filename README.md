Rebuild is an attempt at reimplementing Ken Silverman's Build engine, with the
goal of being modular enough to host modern source ports of any Build engine
game. The motivation comes from the fact that [BloodGDX][1], the current
recommended source port for Blood, is nonfree and written in Java. Additionally,
other source ports such as [EDuke32][2] are built upon Ken Silverman's codebase,
which is full of DOS-era optimizations like self-modifying code and a generous
amount of globally-shared state. They're also subject to the restrictions of the
BUILD license.

Also, it's come to my attention that "Rebuild" also happens to be the name of
the toolkit included in Transfusion for working with the BUILD file formats.
This is an unrelated project, and I apologize for any confusion caused by my
lack of foresight.


# Thanks

- To the 3D Realms teams for releasing the source code for Duke Nukem 3D.
- To Fabien Sanglard for his [detailed analysis of the Duke Nukem 3D codebase][3], and his work on Chocolate Duke3D.
- To Richard Gobeille et al. for their work on EDuke32.
- To Mathieu Olivier et al. for their work on Transfusion - specifically their open-source parsing code for the BUILD file formats.


# Roadmap

- [ ] Implement support for GRP archives.
  - [x] GRP parser.
  - [x] Caching system.
  - [ ] Support for 'grpinfo' files and GRP dependency chains.
  - [ ] Support for the 'autoload' directory.
  - [ ] Support for game mods.
  - [ ] Internal support for official add-ons.
  - [ ] Support for CRC32 identification.
  - [ ] Resolution of group files based on the current game.
  - [ ] Proper path resolution.
    - This means removing certain paths when the necessary groups are loaded.
  - [ ] Tests.
- [ ] Implement support for Build's MAPs.
  - [x] MAP parser.
  - [ ] Tests.
- [ ] Implement a timer system.
  - Needs to expose some sort of 'totalclock'.
- [ ] Implement support for ART bitmaps.
  - [x] PALETTE.DAT parser.
  - [x] ART parser.
  - [x] Efficient ART-to-bitmap conversion.
  - [ ] Advanced ART features such as translucency and shading.
  - [ ] Tests.
- [ ] Implement support for the CON language.
  - [ ] Write a compiler.
  - [ ] Implement the virtual machine.
  - [ ] Implement a debugger.
- [ ] Implement support for DEF files.
- [ ] Implement support for MACT scripts. (Apparently quite similar to INI)
- [ ] Implement support for RTS.
  - Appears to be a Doom iwad, though I'm not sure what its purpose is.
- [ ] Implement graphical output.
  - [ ] Write something using the kiss3d library to hold me over while I'm trying to learn the linear algebra required to write an actual renderer.
  - [ ] Implement something equivalent to the "classic" software renderer.
  - [ ] Implement a more powerful glium renderer, taking from POLYMER's architecture.
- [ ] Implement the OSD shell.
  - [ ] Gamevars and cvars.
  - [ ] Loading of 'autoexec.cfg', 'settings.cfg'.
- [ ] Implement the CONTROL input system.
  - EDuke32's controls feel very fluid for me, so I'll steal the calculations and make the API less archaic.
- [ ] A welcoming main menu.
- [ ] Support for save files.
- [ ] Support for ANM cinematics.
- [ ] Multiplayer networking!
  - Doesn't necessarily have to be compatible with ENet, but that would be a plus.


[1]: https://blood-wiki.org/index.php/BloodGDX
[2]: http://eduke32.com/
[3]: http://fabiensanglard.net/duke3d/index.php
