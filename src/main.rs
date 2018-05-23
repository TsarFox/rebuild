#[macro_use]
extern crate simple_error;

use std::process;

mod fmt;

// initgroupfile(char *filename)
// {
//      char buf[16];
//      long i, j, k;

//      if (numgroupfiles >= MAXGROUPFILES) return(-1);

//      groupfil[numgroupfiles] = open(filename,O_BINARY|O_RDWR,S_IREAD);
//      if (groupfil[numgroupfiles] != -1)
//      {
//              groupfilpos[numgroupfiles] = 0;
//              read(groupfil[numgroupfiles],buf,16);
//              if ((buf[0] != 'K') || (buf[1] != 'e') || (buf[2] != 'n') ||
//                       (buf[3] != 'S') || (buf[4] != 'i') || (buf[5] != 'l') ||
//                       (buf[6] != 'v') || (buf[7] != 'e') || (buf[8] != 'r') ||
//                       (buf[9] != 'm') || (buf[10] != 'a') || (buf[11] != 'n'))
//              {
//                      close(groupfil[numgroupfiles]);
//                      groupfil[numgroupfiles] = -1;
//                      return(-1);
//              }
//              gnumfiles[numgroupfiles] = *((long *)&buf[12]);

//              if ((gfilelist[numgroupfiles] = (char *)kmalloc(gnumfiles[numgroupfiles]<<4)) == 0)
//                      { printf("Not enough memory for file grouping system\n"); exit(0); }
//              if ((gfileoffs[numgroupfiles] = (long *)kmalloc((gnumfiles[numgroupfiles]+1)<<2)) == 0)
//                      { printf("Not enough memory for file grouping system\n"); exit(0); }

//              read(groupfil[numgroupfiles],gfilelist[numgroupfiles],gnumfiles[numgroupfiles]<<4);

//              j = 0;
//              for(i=0;i<gnumfiles[numgroupfiles];i++)
//              {
//                      k = *((long *)&gfilelist[numgroupfiles][(i<<4)+12]);
//                      gfilelist[numgroupfiles][(i<<4)+12] = 0;
//                      gfileoffs[numgroupfiles][i] = j;
//                      j += k;
//              }
//              gfileoffs[numgroupfiles][gnumfiles[numgroupfiles]] = j;
//      }
//      numgroupfiles++;
//      return(groupfil[numgroupfiles-1]);
// }


// High-level description:
// - Caches group files into an array with a rolling index.
// - Keeps track of group file positions (pointer?) starts as 0.
// - Keeps track of number of files in another array (gnumfiles)
//
// - Check header for "KenSilverman", invalidate entry in cache if bad.
// - kmalloc 16 * number of files (for storing table)
// - kmalloc 4 * (number of files + 1) (for storing individual file offsets)
// - Read in table
// - For every entry, read in the file size, and invalidate the entry.




// -------

// void printstr(short x, short y, char string[81], char attribute)
// {
//         char character;
//         short i, pos;

//         pos = (y*80+x)<<1;
//         i = 0;
//         while (string[i] != 0)
//         {
//                 character = string[i];
//                 printchrasm(0xb8000+(long)pos,1L,((long)attribute<<8)+(long)character);
//                 i++;
//                 pos+=2;
//         }
// }

// static char todd[] = "Duke Nukem 3D(tm) Copyright 1989, 1996 Todd Replogle and 3D Realms Entertainment";
// static char trees[] = "I want to make a game with trees";
// static char sixteen[] = "16 Possible Dukes";

fn main() {
    // #define VERSION "1.4"
    // #define HEAD2  "Duke Nukem 3D v"VERSION" - Atomic Edition"
    // printstr(40-(strlen(HEAD2)>>1),0,HEAD2,79);

    // ud.multimode = 1;
    // printstr(0,1,"                   Copyright (c) 1996 3D Realms Entertainment                   ",79);

    // initgroupfile("duke3d.grp");

    let filename = "DUKE3D.GRP";
    let mut group_manager = fmt::GroupManager::new();

    if let Err(e) = group_manager.load_from_file(filename) {
        println!("Couldn't open {}: {}", filename, e);
        process::exit(1);
    }

    println!("DOGWHINE.VOC: {} bytes", group_manager.get("DOGWHINE.VOC").unwrap().len());

    // checkcommandline(argc,argv);

    println!("You don't have enough free memory to run Duke Nukem 3D.");
    println!("The DOS \"mem\" command should report 6,800K (or 6.8 megs)");
    println!("of \"total memory free\".");
    println!("");
    println!("Duke Nukem 3D requires {} more bytes to run.", 3162000 - 350000);
    process::exit(1);

    // Considering that most of this can be implemented with Drop, I
    // don't think it's necessary.

    // RegisterShutdownFunction( ShutDown );
    // void ShutDown( void )
    // {
    //     SoundShutdown();
    //     MusicShutdown();
    //     uninittimer();
    //     uninitengine();
    //     CONTROL_Shutdown();
    //     CONFIG_WriteSetup();
    //     KB_Shutdown();
    // }

    // Startup();


    // if(numplayers > 1)
   //  {
   //      ud.multimode = numplayers;
   //      sendlogon();
   //  }
   //  else if(boardfilename[0] != 0)
   //  {
   //      ud.m_level_number = 7;
   //      ud.m_volume_number = 0;
   //      ud.warp_on = 1;
   //  }

   //  getnames();

   //  if(ud.multimode > 1)
   //  {
   //      playerswhenstarted = ud.multimode;

   //      if(ud.warp_on == 0)
   //      {
   //          ud.m_monsters_off = 1;
   //          ud.m_player_skill = 0;
   //      }
   //  }

   //  ud.last_level = -1;

   // RTS_Init(ud.rtsname);
   // if(numlumps) printf("Using .RTS file:%s\n",ud.rtsname);

    // if( setgamemode(ScreenMode,ScreenWidth,ScreenHeight) < 0 )
    // {
    //     printf("\nVESA driver for ( %i * %i ) not found/supported!\n",xdim,ydim);
    //     ScreenMode = 2;
    //     ScreenWidth = 320;
    //     ScreenHeight = 200;
    //     setgamemode(ScreenMode,ScreenWidth,ScreenHeight);
    // }

    // // CTW END - MODIFICATION

    // genspriteremaps();

// #ifdef VOLUMEONE
//         if(numplayers > 4 || ud.multimode > 4)
//             gameexit(" The full version of Duke Nukem 3D supports 5 or more players.");
// #endif

    // setbrightness(ud.brightness>>2,&ps[myconnectindex].palette[0]);

    // ESCESCAPE;

    // FX_StopAllSounds();
    // clearsoundlocks();

    // if(ud.warp_on > 1 && ud.multimode < 2)
    // {
    //     clearview(0L);
    //     ps[myconnectindex].palette = palette;
    //     palto(0,0,0,0);
    //     rotatesprite(320<<15,200<<15,65536L,0,LOADSCREEN,0,0,2+8+64,0,0,xdim-1,ydim-1);
    //     menutext(160,105,0,0,"LOADING SAVED GAME...");
    //     nextpage();

    //     j = loadplayer(ud.warp_on-2);
    //     if(j)
    //         ud.warp_on = 0;
    // }

    // //    getpackets();

//     MAIN_LOOP_RESTART:

//     if(ud.warp_on == 0)
//         Logo();
//     else if(ud.warp_on == 1)
//     {
//         newgame(ud.m_volume_number,ud.m_level_number,ud.m_player_skill);
//         enterlevel(MODE_GAME);
//     }
//     else vscrn();

//     tempautorun = ud.auto_run;

//     if( ud.warp_on == 0 && playback() )
//     {
//         FX_StopAllSounds();
//         clearsoundlocks();
//         nomorelogohack = 1;
//         goto MAIN_LOOP_RESTART;
//     }

//     ud.auto_run = tempautorun;

//     ud.warp_on = 0;

//     while ( !(ps[myconnectindex].gm&MODE_END) ) //The whole loop!!!!!!!!!!!!!!!!!!
//     {
//         if( ud.recstat == 2 || ud.multimode > 1 || ( ud.show_help == 0 && (ps[myconnectindex].gm&MODE_MENU) != MODE_MENU ) )
//             if( ps[myconnectindex].gm&MODE_GAME )
//                 if( moveloop() ) continue;

//         if( ps[myconnectindex].gm&MODE_EOL || ps[myconnectindex].gm&MODE_RESTART )
//         {
//             if( ps[myconnectindex].gm&MODE_EOL )
//             {
// #ifdef ONELEVELDEMO
//                 gameexit(" ");
// #endif
//                 closedemowrite();

//                 ready2send = 0;

//                 i = ud.screen_size;
//                 ud.screen_size = 0;
//                 vscrn();
//                 ud.screen_size = i;
//                 dobonus(0);

//                 if(ud.eog)
//                 {
//                     ud.eog = 0;
//                     if(ud.multimode < 2)
//                     {
// #ifndef VOLUMEALL
//                         doorders();
// #endif
//                         ps[myconnectindex].gm = MODE_MENU;
//                         cmenu(0);
//                         probey = 0;
//                         goto MAIN_LOOP_RESTART;
//                     }
//                     else
//                     {
//                         ud.m_level_number = 0;
//                         ud.level_number = 0;
//                     }
//                 }
//             }

//             ready2send = 0;
//             if(numplayers > 1) ps[myconnectindex].gm = MODE_GAME;
//             enterlevel(ps[myconnectindex].gm);
//             continue;
//         }

//         cheats();
//         nonsharedkeys();

//         if( (ud.show_help == 0 && ud.multimode < 2 && !(ps[myconnectindex].gm&MODE_MENU) ) || ud.multimode > 1 || ud.recstat == 2)
//             i = min(max((totalclock-ototalclock)*(65536L/TICSPERFRAME),0),65536);
//         else
//             i = 65536;

//         displayrooms(screenpeek,i);
//         displayrest(i);

// //        if( KB_KeyPressed(sc_F) )
// //        {
// //            KB_ClearKeyDown(sc_F);
// //            addplayer();
// //        }

//         if(ps[myconnectindex].gm&MODE_DEMO)
//             goto MAIN_LOOP_RESTART;

//         if(debug_on) caches();

//         checksync();

// #ifdef VOLUMEONE
//         if(ud.show_help == 0 && show_shareware > 0 && (ps[myconnectindex].gm&MODE_MENU) == 0 )
//             rotatesprite((320-50)<<16,9<<16,65536L,0,BETAVERSION,0,0,2+8+16+128,0,0,xdim-1,ydim-1);
// #endif
//         nextpage();
//     }

//     gameexit(" ");
}
