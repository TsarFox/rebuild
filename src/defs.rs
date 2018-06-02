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

// Straight ripped from DUKE3D.H

pub struct UserDefs {
    god: bool,
    warp_on: bool,
    cashman: bool,
    eog: bool,
    showallmap: bool,
    show_help: bool,
    scrollmode: bool,
    clipping: bool,
    overhead_on: bool,
    last_overhead: bool,
    showweapons: bool,

    // Vec is MAXPLAYERS in size, names are 32 characters.
    user_name: Vec<String>,

    // Vec is 10 in size, strings are 40 characters.
    ridecule: Vec<String>,

    // Vec is 10 in size, strings are 22 characters.
    savegame: Vec<String>,

    // Constrained to 128 characters.
    pwlockout: String,

    // Constrained to 128 characters.
    rtsname: String,

    pause_on: i16, // Maybe a bool?
    from_bonus: i16, // ??
    camerasprite: i16, //??
    last_camsprite: i16, // ??
    last_level: i16, // ??
    secretlevel: i16,

    // These names are pretty shitty.
    const_visibility: i32,
    uw_framerate: i32,
    camera_time: i32,
    folfvel: i32,
    folavel: i32,
    folx: i32,
    foly: i32,
    fola: i32,
    reccnt: i32,

    entered_name: i32,
    screen_tilting: i32,
    shadows: i32,
    fta_on: i32,
    executions: i32,
    auto_run: i32,

    coords: i32,
    tickrate: i32,
    m_coop: i32,
    coop: i32,
    screen_size: i32,
    lockout: i32,
    crosshair: i32,

    // [MAXPLAYERS][MAX_WEAPONS]
    wchoice: Vec<Vec<i32>>,
    playerai: i32,

    respawn_monsters: i32,
    respawn_items: i32,
    respawn_inventory: i32,
    recstat: i32,
    monsters_off: i32,
    brightness: i32,

    m_respawn_items: i32,
    m_respawn_monsters: i32,
    m_respawn_inventory: i32,
    m_recstat: i32,
    m_monsters_off: i32,
    detail: i32,

    m_ffire: i32,
    ffire: i32,
    m_player_skill: i32,
    m_level_number: i32,
    m_volume_number: i32,
    multimode: i32,

    player_skill: i32,
    level_number: i32,
    volume_number: i32,
    m_marker: i32,
    marker: i32,
    mouseflip: i32,
}
