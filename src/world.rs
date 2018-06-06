// Copyright (C) 2018 Jakob L. Kreuze, All Rights Reserved.
//
// This file is part of rebuild.
//
// rebuild is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// rebuild is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// rebuild. If not, see <http://www.gnu.org/licenses/>.

// FIXME: Signedness of 'char' is ambiguous from documentation.

extern crate byteorder;
extern crate simple_error;

use std::error::Error;
use std::io::Cursor;

use self::byteorder::{LE, ReadBytesExt};

/// Maintains the current state of the game world - the map geometry and
/// everything contained within it.
#[derive(Debug)]
pub struct World {
    sectors: Vec<Sector>,
    walls: Vec<Wall>,
}

impl World {
    // TODO: Document this.
    // FIXME: Doesn't do any sort of sanity checks on length.
    pub fn from_map(data: &[u8]) -> Result<World, Box<Error>> {
        let mut data = Cursor::new(data);

        // From BUILDINF.TXT
        //
        // Here is Ken's documentation on the COMPLETE BUILD map format:
        // BUILD engine and editor programmed completely by Ken Silverman
        //
        // Here's how you should read a BUILD map file:
        // {
        //      fil = open(???);
        //
        //      // Load map version number (current version is 7L)
        //      read(fil,&mapversion,4);
        //
        //      // Load starting position
        //      read(fil,posx,4);
        //      read(fil,posy,4);
        //      read(fil,posz,4);    // Note: Z coordinates are all shifted up 4
        //      read(fil,ang,2);     // All angles are from 0-2047, clockwise
        //      read(fil,cursectnum,2); // Sector of starting point
        //
        //      // Load all sectors (see sector structure described below)
        //      read(fil,&numsectors,2);
        //      read(fil,&sector[0],sizeof(sectortype)*numsectors);
        //
        //      // Load all walls (see wall structure described below)
        //      read(fil,&numwalls,2);
        //      read(fil,&wall[0],sizeof(walltype)*numwalls);
        //
        //      // Load all sprites (see sprite structure described below)
        //      read(fil,&numsprites,2);
        //      read(fil,&sprite[0],sizeof(spritetype)*numsprites);
        //
        //      close(fil);
        // }

        let version = data.read_u32::<LE>()?;

        if version != 7 {
            bail!("Unsupported MAP version.");
        }

        // TODO: Use this to position the world's player (?)
        let _start_x = data.read_i32::<LE>()?;
        let _start_y = data.read_i32::<LE>()?;
        let _start_z = data.read_i32::<LE>()?;
        let _start_angle = data.read_i16::<LE>()? & 0x7ff;
        let _start_sector = data.read_i16::<LE>()?;

        // From BUILDINF.TXT:
        //
        // -------------------------------------------------------------
        // | @@@@@@@ @@@@@@@ @@@@@@@ @@@@@@@@ @@@@@@@ @@@@@@@  @@@@@@@ |
        // | @@      @@      @@         @@    @@   @@ @@   @@@ @@      |
        // | @@@@@@@ @@@@@   @@         @@    @@   @@ @@@@@@@  @@@@@@@ |
        // |      @@ @@      @@         @@    @@   @@ @@   @@@      @@ |
        // | @@@@@@@ @@@@@@@ @@@@@@@    @@    @@@@@@@ @@    @@ @@@@@@@ |
        // -------------------------------------------------------------
        //
        // // sizeof(sectortype) = 40
        // typedef struct
        // {
        //      short wallptr, wallnum;
        //      long ceilingz, floorz;
        //      short ceilingstat, floorstat;
        //      short ceilingpicnum, ceilingheinum;
        //      signed char ceilingshade;
        //      char ceilingpal, ceilingxpanning, ceilingypanning;
        //      short floorpicnum, floorheinum;
        //      signed char floorshade;
        //      char floorpal, floorxpanning, floorypanning;
        //      char visibility, filler;
        //      short lotag, hitag, extra;
        // } sectortype;
        // sectortype sector[1024];
        //
        // wallptr - index to first wall of sector
        // wallnum - number of walls in sector
        // z's - z coordinate (height) at first point of sector
        //
        // stat's
        //      bit 0: 1 = parallaxing, 0 = not                              "P"
        //      bit 1: 1 = sloped, 0 = not
        //      bit 2: 1 = swap x&y, 0 = not                                 "F"
        //      bit 3: 1 = double smooshiness                                "E"
        //      bit 4: 1 = x-flip                                            "F"
        //      bit 5: 1 = y-flip                                            "F"
        //      bit 6: 1 = Align texture to first wall of sector             "R"
        //      bits 7-15: reserved
        //
        // picnum's - texture index into art file
        // heinum's - slope value (0-parallel to floor, 4096-45 degrees)
        // shade's - shade offset of ceiling/floor
        // pal's - palette lookup table number (0 - use standard colors)
        // panning's - used to align textures or to do texture panning
        // visibility - determines how area changes shade relative to distance
        // filler - useless byte to make structure aligned
        // lotag, hitag, extra - These variables used by the programmer only

        let mut sectors = Vec::new();
        let sector_count = data.read_u16::<LE>()?;

        for _ in 0..sector_count {
            let first_wall = data.read_i16::<LE>()?;
            let wall_count = data.read_i16::<LE>()?;
            let ceiling_height = data.read_i32::<LE>()?;
            let floor_height = data.read_i32::<LE>()?;
            let ceiling_status = data.read_i16::<LE>()?;
            let floor_status = data.read_i16::<LE>()?;
            let ceiling_bitmap = data.read_i16::<LE>()?;
            let ceiling_slope = data.read_i16::<LE>()?;
            let ceiling_shade = data.read_i8()?;
            let ceiling_palette = data.read_u8()?;
            let ceiling_panning_x = data.read_u8()?;
            let ceiling_panning_y = data.read_u8()?;
            let floor_bitmap = data.read_i16::<LE>()?;
            let floor_slope = data.read_i16::<LE>()?;
            let floor_shade = data.read_i8()?;
            let floor_palette = data.read_u8()?;
            let floor_panning_x = data.read_u8()?;
            let floor_panning_y = data.read_u8()?;
            let visibility = data.read_u8()?;
            let _padding = data.read_u8()?;
            let lotag = data.read_i16::<LE>()?;
            let hitag = data.read_i16::<LE>()?;
            let extra = data.read_i16::<LE>()?;

            sectors.push(Sector {
                first_wall,
                wall_count,
                visibility,
                tags: (lotag, hitag, extra),

                ceiling_height,
                ceiling_slope,
                ceiling_status,
                ceiling_bitmap,
                ceiling_shade,
                ceiling_palette,
                ceiling_panning: (ceiling_panning_x, ceiling_panning_y),

                floor_height,
                floor_slope,
                floor_status,
                floor_bitmap,
                floor_shade,
                floor_palette,
                floor_panning: (floor_panning_x, floor_panning_y),
            });
        }

        // From BUILDINF.TXT:
        //
        // -----------------------------------------------
        // | @@      @@ @@@@@@@@ @@      @@      @@@@@@@ |
        // | @@      @@ @@    @@ @@      @@      @@      |
        // | @@  @@  @@ @@@@@@@@ @@      @@      @@@@@@@ |
        // | @@ @@@@ @@ @@    @@ @@      @@           @@ |
        // |  @@@ @@@@  @@    @@ @@@@@@@ @@@@@@@ @@@@@@@ |
        // ----------------------------------------------|
        //
        // // sizeof(walltype) = 32
        // typedef struct
        // {
        //      long x, y;
        //      short point2, nextwall, nextsector, cstat;
        //      short picnum, overpicnum;
        //      signed char shade;
        //      char pal, xrepeat, yrepeat, xpanning, ypanning;
        //      short lotag, hitag, extra;
        // } walltype;
        // walltype wall[8192];
        //
        // x, y: Coordinate of left side of wall
        // point2: Index to next wall on the right (in same sector)
        // nextwall: Index to wall on other side (-1 if there is no sector)
        // nextsector: Index to sector on other side (-1 if there is no sector)
        // cstat:
        //      bit 0: 1 = Blocking wall (use with clipmove, getzrange)      "B"
        //      bit 1: 1 = bottoms of invisible walls swapped, 0 = not       "2"
        //      bit 2: 1 = align picture on bottom (for doors), 0 = top      "O"
        //      bit 3: 1 = x-flipped, 0 = normal                             "F"
        //      bit 4: 1 = masking wall, 0 = not                             "M"
        //      bit 5: 1 = 1-way wall, 0 = not                               "1"
        //      bit 6: 1 = Blocking wall (use with hitscan / cliptype 1)     "H"
        //      bit 7: 1 = Transluscence, 0 = not                            "T"
        //      bit 8: 1 = y-flipped, 0 = normal                             "F"
        //      bit 9: 1 = Transluscence reversing, 0 = normal               "T"
        //      bits 10-15: reserved
        // picnum - texture index into art file
        // overpicnum - texture index into art file for masked / 1-way walls
        // shade - shade offset of wall
        // pal - palette lookup table number (0 - use standard colors)
        // repeat's - used to change the size of pixels (stretch textures)
        // pannings - used to align textures or to do texture panning
        // lotag, hitag, extra - These variables used by the programmer only

        let mut walls = Vec::new();
        let wall_count = data.read_u16::<LE>()?;

        for _ in 0..wall_count {
            let position_x = data.read_i32::<LE>()?;
            let position_y = data.read_i32::<LE>()?;
            let adjacent_wall_index = data.read_i16::<LE>()?;
            let opposite_wall_index = data.read_i16::<LE>()?;
            let into_sector_index = data.read_i16::<LE>()?;
            let status = data.read_i16::<LE>()?;
            let bitmap = data.read_i16::<LE>()?;
            let bitmap_overlay = data.read_i16::<LE>()?;
            let shade = data.read_i8()?;
            let palette = data.read_u8()?;
            let stretch_x = data.read_u8()?;
            let stretch_y = data.read_u8()?;
            let panning_x = data.read_u8()?;
            let panning_y = data.read_u8()?;
            let lotag = data.read_i16::<LE>()?;
            let hitag = data.read_i16::<LE>()?;
            let extra = data.read_i16::<LE>()?;

            walls.push(Wall {
                position: (position_x, position_y),
                adjacent_wall_index,
                opposite_wall_index,
                into_sector_index,

                bitmap,
                bitmap_overlay,
                shade,
                palette,
                stretch: (stretch_x, stretch_y),
                panning: (panning_x, panning_y),

                status,
                tags: (lotag, hitag, extra),
            });
        }

        // From BUILDINF.TXT:
        //
        // -------------------------------------------------------------
        // | @@@@@@@ @@@@@@@ @@@@@@@   @@@@@@ @@@@@@@@ @@@@@@@ @@@@@@@ |
        // | @@      @@   @@ @@   @@@    @@      @@    @@      @@      |
        // | @@@@@@@ @@@@@@@ @@@@@@@     @@      @@    @@@@@   @@@@@@@ |
        // |      @@ @@      @@    @@    @@      @@    @@           @@ |
        // | @@@@@@@ @@      @@    @@  @@@@@@    @@    @@@@@@@ @@@@@@@ |
        // -------------------------------------------------------------
        //
        // // sizeof(spritetype) = 44
        // typedef struct
        // {
        //         long x, y, z;
        //         short cstat, picnum;
        //         signed char shade;
        //         char pal, clipdist, filler;
        //         unsigned char xrepeat, yrepeat;
        //         signed char xoffset, yoffset;
        //         short sectnum, statnum;
        //         short ang, owner, xvel, yvel, zvel;
        //         short lotag, hitag, extra;
        // } spritetype;
        // spritetype sprite[4096];
        //
        // x, y, z - position of sprite - can be defined at center bottom or center
        // cstat:
        //         bit 0: 1 = Blocking sprite (use with clipmove, getzrange)       "B"
        //         bit 1: 1 = transluscence, 0 = normal                            "T"
        //         bit 2: 1 = x-flipped, 0 = normal                                "F"
        //         bit 3: 1 = y-flipped, 0 = normal                                "F"
        //         bits 5-4: 00 = FACE sprite (default)                            "R"
        //                                  01 = WALL sprite (like masked walls)
        //                                  10 = FLOOR sprite (parallel to ceilings&floors)
        //         bit 6: 1 = 1-sided sprite, 0 = normal                           "1"
        //         bit 7: 1 = Real centered centering, 0 = foot center             "C"
        //         bit 8: 1 = Blocking sprite (use with hitscan / cliptype 1)      "H"
        //         bit 9: 1 = Transluscence reversing, 0 = normal                  "T"
        //         bits 10-14: reserved
        //         bit 15: 1 = Invisible sprite, 0 = not invisible
        // picnum - texture index into art file
        // shade - shade offset of sprite
        // pal - palette lookup table number (0 - use standard colors)
        // clipdist - the size of the movement clipping square (face sprites only)
        // filler - useless byte to make structure aligned
        // repeat's - used to change the size of pixels (stretch textures)
        // offset's - used to center the animation of sprites
        // sectnum - current sector of sprite
        // statnum - current status of sprite (inactive/monster/bullet, etc.)
        //
        // ang - angle the sprite is facing
        // owner, xvel, yvel, zvel, lotag, hitag, extra - These variables used by the game programmer only

        let mut sprites = Vec::new();
        let sprite_count = data.read_u16::<LE>()?;

        for _ in 0..sprite_count {
            let position_x = data.read_i32::<LE>()?;
            let position_y = data.read_i32::<LE>()?;
            let position_z = data.read_i32::<LE>()?;
            let sprite_status = data.read_i16::<LE>()?;
            let bitmap = data.read_i16::<LE>()?;
            let shade = data.read_i8()?;
            let palette = data.read_u8()?;
            let clip_distance = data.read_u8()?;
            let _filler = data.read_u8()?;
            let stretch_x = data.read_u8()?;
            let stretch_y = data.read_u8()?;
            let panning_x = data.read_i8()?;
            let panning_y = data.read_i8()?;
            let sector_index = data.read_i16::<LE>()?;
            let entity_status = data.read_i16::<LE>()?;
            let angle = data.read_i16::<LE>()?;
            let owner = data.read_i16::<LE>()?;
            let velocity_x = data.read_i16::<LE>()?;
            let velocity_y = data.read_i16::<LE>()?;
            let velocity_z = data.read_i16::<LE>()?;
            let lotag = data.read_i16::<LE>()?;
            let hitag = data.read_i16::<LE>()?;
            let extra = data.read_i16::<LE>()?;

            sprites.push(Sprite {
                position: (position_x, position_y, position_z),
                velocity: (velocity_x, velocity_y, velocity_z),
                angle,

                sector_index,

                bitmap,
                clip_distance,
                shade,
                palette,
                stretch: (stretch_x, stretch_y),
                panning: (panning_x, panning_y),

                sprite_status,
                entity_status,

                owner,
                tags: (lotag, hitag, extra),
            });
        }

        Ok(World { sectors, walls })
    }
}

#[derive(Debug)]
struct Sector {
    first_wall: i16,
    wall_count: i16,
    visibility: u8,
    tags: (i16, i16, i16),

    ceiling_height: i32,
    ceiling_slope: i16,
    ceiling_status: i16,
    ceiling_bitmap: i16,
    ceiling_shade: i8,
    ceiling_palette: u8,
    ceiling_panning: (u8, u8),

    floor_height: i32,
    floor_slope: i16,
    floor_status: i16,
    floor_bitmap: i16,
    floor_shade: i8,
    floor_palette: u8,
    floor_panning: (u8, u8),
}

#[derive(Debug)]
struct Wall {
    position: (i32, i32),

    adjacent_wall_index: i16,
    opposite_wall_index: i16,
    into_sector_index: i16,

    bitmap: i16,
    bitmap_overlay: i16,
    shade: i8,
    palette: u8,
    stretch: (u8, u8),
    panning: (u8, u8),

    status: i16,
    tags: (i16, i16, i16),
}

#[derive(Debug)]
struct Sprite {
    position: (i32, i32, i32),
    velocity: (i16, i16, i16),
    angle: i16,

    sector_index: i16,

    bitmap: i16,
    clip_distance: u8,
    shade: i8,
    palette: u8,
    stretch: (u8, u8),
    panning: (i8, i8),

    sprite_status: i16,
    entity_status: i16,

    owner: i16,
    tags: (i16, i16, i16),
}
