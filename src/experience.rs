use num_enum::FromPrimitive;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq, Debug)]
#[repr(i64)]
pub enum ExperienceType {
    Kill_Player = 1,
    Kill_Player_Assist = 2,
    Kill_Player_Spawn_Assist = 3,
    Heal_Player = 4,
    Heal_Assist = 5,
    MAX_Repair = 6,
    Revive = 7,
    Kill_Streak = 8,
    Domination_Kill = 10,
    Revenge_Kill = 11,
    Control_Point_Defend = 15,
    Control_Point_Attack = 16,
    Facility_Captured_Not_Used = 19,
    Facility_Defense = 20,
    Destroy_Secondary_Objective = 21,
    Destroy_SecondaryObjectiveAssist = 22,
    Vehicle_Destruction_Flash = 24,
    Multiple_Kill = 25,
    Vehicle_RoadKill = 26,
    Squad_Repair_Flash = 28,
    Kill_Player_Class_MAX = 29,
    Transport_Assist = 30,
    Vehicle_Repair_Flash = 31,
    Nemesis_Kill = 32,
    Resupply_Player = 34,
    Spot_Kill = 36,
    Headshot = 37,
    Stop_Kill_Streak = 38,
    Meta_Game_Event_reward = 47,
    Squad_Heal = 51,
    Squad_Revive = 53,
    Squad_Spot_Kill = 54,
    Squad_Resupply = 55,
    Squad_Spawn = 56,
    Destroy_Engineer_Turret = 57,
    Vehicle_Destruction_Phalanx = 58,
    Vehicle_Destruction_Drop_Pod = 59,
    Vehicle_Destruction_Galaxy = 60,
    Vehicle_Destruction_Liberator = 61,
    Vehicle_Destruction_Lightning = 62,
    Vehicle_Destruction_Magrider = 63,
    Vehicle_Destruction_Mosquito = 64,
    Vehicle_Destruction_Prowler = 65,
    Vehicle_Destruction_Reaver = 66,
    Vehicle_Destruction_Scythe = 67,
    Vehicle_Destruction_Sunderer = 68,
    Vehicle_Destruction_Vanguard = 69,
    Vehicle_Ram_Bonus = 72,
    Vehicle_Ram_Kill_Engi_Turret = 73,
    Vehicle_Ram_Kill_Phalanx = 74,
    Vehicle_Ram_Kill_Drop_Pod = 75,
    Vehicle_Ram_Kill_Galaxy = 76,
    Vehicle_Ram_Kill_Liberator = 77,
    Vehicle_Ram_Kill_Lightning = 78,
    Vehicle_Ram_Kill_Magrider = 79,
    Vehicle_Ram_Kill_Mosquito = 80,
    Vehicle_Ram_Kill_Prowler = 81,
    Vehicle_Ram_Kill_Reaver = 82,
    Vehicle_Ram_Kill_Scythe = 83,
    Vehicle_Ram_Kill_Sunderer = 84,
    Vehicle_Ram_Kill_Vanguard = 85,
    Explosive_Destruction = 86,
    Secondary_Facility_Object_Repair = 87,
    Vehicle_Repair_Engi_Turret = 88,
    Vehicle_Repair_Phalanx = 89,
    Vehicle_Repair_Drop_Pod = 90,
    Vehicle_Repair_Galaxy = 91,
    Vehicle_Repair_Liberator = 92,
    Vehicle_Repair_Lightning = 93,
    Vehicle_Repair_Magrider = 94,
    Vehicle_Repair_Mosquito = 95,
    Vehicle_Repair_Prowler = 96,
    Vehicle_Repair_Reaver = 97,
    Vehicle_Repair_Scythe = 98,
    Vehicle_Repair_Sunderer = 99,
    Vehicle_Repair_Vanguard = 100,
    Kill_Assist_Flash = 101,
    Kill_Assist_Engi_Turret = 102,
    Kill_Assist_Phalanx = 103,
    Kill_Assist_Drop_Pod = 104,
    Kill_Assist_Galaxy = 105,
    Kill_Assist_Liberator = 106,
    Kill_Assist_Lightning = 107,
    Kill_Assist_Magrider = 108,
    Kill_Assist_Mosquito = 109,
    Kill_Assist_Prowler = 110,
    Kill_Assist_Reaver = 111,
    Kill_Assist_Scythe = 112,
    Kill_Assist_Sunderer = 113,
    Kill_Assist_Vanguard = 114,
    Squad_Repair_Engi_Turret = 129,
    Squad_Repair_Phalanx = 130,
    Squad_Repair_Drop_Pod = 131,
    Squad_Repair_Galaxy = 132,
    Squad_Repair_Liberator = 133,
    Squad_Repair_Lightning = 134,
    Squad_Repair_Magrider = 135,
    Squad_Repair_Mosquito = 136,
    Squad_Repair_Prowler = 137,
    Squad_Repair_Reaver = 138,
    Squad_Repair_Scythe = 139,
    Squad_Repair_Sunderer = 140,
    Squad_Repair_Vanguard = 141,
    Squad_MAX_Repair = 142,
    Drop_Pod_Kill = 143,
    Player_Kill_by_Sunderer_Gunner = 146,
    Player_Kill_by_Magrider_Gunner = 148,
    Player_Kill_by_Vanguard_Gunner = 149,
    Player_Kill_by_Prowler_Gunner = 150,
    Player_Kill_by_Liberator_Gunner = 154,
    Player_Kill_by_Galaxy_Gunner = 155,
    Flash_Kill_by_Sunderer_Gunner = 159,
    Sunderer_Kill_by_Sunderer_Gunner = 160,
    Lightning_Kill_by_Sunderer_Gunner = 161,
    Magrider_Kill_by_Sunderer_Gunner = 162,
    Vanguard_Kill_by_Sunderer_Gunner = 163,
    Prowler_Kill_by_Sunderer_Gunner = 164,
    Scythe_Kill_by_Sunderer_Gunner = 165,
    Reaver_Kill_by_Sunderer_Gunner = 166,
    Mosquito_Kill_by_Sunderer_Gunner = 167,
    Lib_Kill_by_Sunderer = 168,
    Galaxy_Kill_by_Sunderer_Gunner = 169,
    Flash_Kill_by_Magrider_Gunner = 181,
    Sunderer_Kill_by_Magrider_Gunner = 182,
    Lightning_Kill_by_Magrider_Gunner = 183,
    Vanguard_Kill_by_Magrider_Gunner = 184,
    Prowler_Kill_by_Magrider_Gunner = 185,
    Reaver_Kill_by_Magrider_Gunner = 186,
    Mosquito_Kill_by_Magrider_Gunner = 187,
    Lib_Kill_by_Magrider = 188,
    Galaxy_Kill_by_Magrider_Gunner = 189,
    Flash_Kill_by_Vanguard_Gunner = 190,
    Sunderer_Kill_by_Vanguard_Gunner = 191,
    Lightning_Kill_by_Vanguard_Gunner = 192,
    Magrider_Kill_by_Vanguard_Gunner = 193,
    Prowler_Kill_by_Vanguard_Gunner = 195,
    Scythe_Kill_by_Vanguard_Gunner = 196,
    Mosquito_Kill_by_Vanguard_Gunner = 197,
    Lib_Kill_by_Vanguard = 198,
    Galaxy_Kill_by_Vanguard_Gunner = 199,
    Flash_Kill_by_Prowler_Gunner = 200,
    Galaxy_Spawn_Bonus = 201,
    Sunderer_Kill_by_Prowler_Gunner = 202,
    Lightning_Kill_by_Prowler_Gunner = 203,
    Magrider_Kill_by_Prowler_Gunner = 204,
    Vanguard_Kill_by_Prowler_Gunner = 205,
    Scythe_Kill_by_Prowler_Gunner = 207,
    Reaver_Kill_by_Prowler_Gunner = 208,
    Liberator_Kill_by_Prowler_Gunner = 209,
    Galaxy_Kill_by_Prowler_Gunner = 210,
    Flash_Kill_by_Liberator_Gunner = 211,
    Sunderer_Kill_by_Lib_Gunner = 212,
    Lightning_Kill_by_Liberator_Gunner = 213,
    Magrider_Kill_by_Lib_Gunner = 214,
    Vanguard_Kill_by_Lib_Gunner = 215,
    Prowler_Kill_by_Liberator_Gunner = 216,
    Scythe_Kill_by_Liberator_Gunner = 217,
    Reaver_Kill_by_Liberator_Gunner = 218,
    Mosquito_Kill_by_Lib_Gunner = 219,
    Lib_Kill_by_Liberator = 220,
    Galaxy_Kill_by_Liberator_Gunner = 221,
    Flash_Kill_by_Galaxy_Gunner = 222,
    Sunderer_Kill_by_Galaxy_Gunner = 223,
    Lightning_Kill_by_Galaxy_Gunner = 224,
    Magrider_Kill_by_Galaxy_Gunner = 225,
    Vanguard_Kill_by_Galaxy_Gunner = 226,
    Prowler_Kill_by_Galaxy_Gunner = 227,
    Scythe_Kill_by_Galaxy_Gunner = 228,
    Reaver_Kill_by_Galaxy_Gunner = 229,
    Mosquito_Kill_by_Galaxy_Gunner = 230,
    LibKill_by_Galaxy_Gunner = 231,
    Galaxy_Kill_by_Galaxy_Gunner = 232,
    Sunderer_Spawn_Bonus = 233,
    Facility_placed_bomb = 234,
    Facility_defused_bomb = 235,
    Facility_Terminal_Hack = 236,
    Facility_Turret_Hack = 237,
    Vehicle_Resupply = 240,
    Squad_Vehicle_Resupply = 241,
    Spot_Kill_Flash = 242,
    Spot_Kill_Engi_Turret = 243,
    Spot_Kill_Phalanx = 244,
    Spot_Kill_Drop_Pod = 245,
    Spot_Kill_Galaxy = 246,
    Spot_Kill_Liberator = 247,
    Spot_Kill_Lightning = 248,
    Spot_Kill_Magrider = 249,
    Spot_Kill_Mosquito = 250,
    Spot_Kill_Prowler = 251,
    Spot_Kill_Reaver = 252,
    Spot_Kill_Scythe = 253,
    Spot_Kill_Sunderer = 254,
    Spot_Kill_Vanguard = 255,
    Squad_Spot_Kill_Flash = 256,
    Squad_Spot_Kill_Engi_Turret = 257,
    Squad_Spot_Kill_Phalanx = 258,
    Squad_Spot_Kill_Drop_Pod = 259,
    Squad_Spot_Kill_Galaxy = 260,
    Squad_Spot_Kill_Liberator = 261,
    Squad_Spot_Kill_Lightning = 262,
    Squad_Spot_Kill_Magrider = 263,
    Squad_Spot_Kill_Mosquito = 264,
    Squad_Spot_Kill_Prowler = 265,
    Squad_Spot_Kill_Reaver = 266,
    Squad_Spot_Kill_Scythe = 267,
    Squad_Spot_Kill_Sunderer = 268,
    Squad_Spot_Kill_Vanguard = 269,
    Squad_Spawn_Beacon_Kill = 270,
    Convert_Capture_Point = 272,
    Terminal_Kill = 275,
    Terminal_Repair = 276,
    Spawn_Kill = 277,
    Priority_Kill = 278,
    High_Priority_Kill = 279,
    Lightning_Damage_Infantry_vs_Vehicle = 280,
    Prowler_Damage_Infantry_vs_Vehicle = 281,
    Galaxy_Damage = 283,
    Liberator_Damage = 284,
    Magrider_Damage_Infantry_vs_Vehicle = 285,
    Mosquito_Damage = 286,
    Reaver_Damage = 287,
    Scythe_Damage = 288,
    Sunderer_Damage_Infantry_vs_Vehicle = 289,
    Vanguard_Damage_Infantry_vs_Vehicle = 290,
    Ribbon_Experience = 291,
    Motion_Detect = 293,
    Squad_Motion_Spot = 294,
    Vehicle_Ram_Kill_Harasser = 300,
    Vehicle_Destruction_Harasser = 301,
    Squad_Repair_Harasser = 302,
    Vehicle_Repair_Harasser = 303,
    Kill_Assist_Harasser = 304,
    Spot_Kill_Harasser = 306,
    Squad_Spot_Kill_Harasser = 307,
    Harasser_Kill_by_Sunderer_Gunner = 308,
    Harasser_Kill_by_Magrider_Gunner = 309,
    Harasser_Kill_by_Vanguard_Gunner = 310,
    Harasser_Kill_by_Prowler_Gunner = 311,
    Harasser_Kill_by_Liberator_Gunner = 312,
    Harasser_Kill_by_Galaxy_Gunner = 313,
    Player_Kill_by_Harasser_Gunner = 314,
    Flash_Kill_by_Harasser_Gunner = 315,
    Sunderer_Kill_by_Harasser_Gunner = 316,
    Lightning_Kill_by_Harasser_Gunner = 317,
    Vanguard_Kill_by_Harasser_Gunner = 318,
    Prowler_Kill_by_Harasser_Gunner = 319,
    Reaver_Kill_by_Harasser_Gunner = 320,
    Mosquito_Kill_by_Harasser_Gunner = 321,
    Lib_Kill_by_Harasser = 322,
    Galaxy_Kill_by_Harasser_Gunner = 323,
    Harasser_Kill_by_Harasser_Gunner = 324,
    Magrider_Kill_by_Harasser_Gunner = 325,
    Scythe_Kill_by_Harasser_Gunner = 326,
    Tank_Mine_Despawn_or_Defusal = 327,
    Alert_Reward = 328,
    Tank_Hunter_Bonus_Prowler_Kill = 329,
    Tank_Hunter_Bonus_Magrider_Kil = 330,
    Dogfighter_Bonus_Mosquito_Kill = 331,
    Dogfighter_Bonus_Reaver_Kill = 332,
    Dogfighter_Bonus_Scythe_Kill = 333,
    Tank_Hunter_Bonus_Vanguard_Kil = 334,
    Savior_Kill_Non_MAX = 335,
    Saved = 336,
    Holiday_Event_NPC_Kill = 337,
    Holiday_Event_NPC_Gold_Kill = 338,
    Snowman_Kill_by_Sunderer_Gunner = 339,
    Snowman_Kill_by_Magrider_Gunner = 340,
    Snowman_Kill_by_Vanguard_Gunner = 341,
    Snowman_Kill_by_Prowler_Gunner = 342,
    Snowman_Kill_by_Liberator_Gunner = 343,
    Snowman_Kill_by_Galaxy_Gunner = 344,
    Snowman_Kill_by_Harasser_Gunner = 345,
    GSnowman_Kill_by_Sunder_Gunner = 346,
    GSnowman_Kill_by_Mag_Gunner = 347,
    GSnowman_Kill_by_Vang_Gunner = 348,
    GSnowman_Kill_by_Prow_Gunner = 349,
    GSnowman_Kill_by_Lib_Gunner = 350,
    GSnowman_Kill_by_Gal_Gunner = 351,
    GSnowman_Kill_by_Haras_Gunner = 352,
    Scout_Radar_Detect = 353,
    Squad_Scout_Radar_Detect = 354,
    Squad_Vehicle_Spawn_Bonus = 355,
    Vehicle_Ram_Kill_R_Drone = 356,
    Vehicle_Destruction_R_Drone = 357,
    Squad_Repair_R_Drone = 358,
    Vehicle_Repair_R_Drone = 359,
    Kill_Assist_R_Drone = 360,
    R_Drone_Kill_by_Harasser_Gunner = 362,
    R_Drone_Kill_by_Sunderer_Gunner = 363,
    R_Drone_Kill_by_Magrider_Gunner = 364,
    R_Drone_Kill_by_Prowler_Gunner = 365,
    R_Drone_Kill_by_Lib_Gunner = 366,
    R_Drone_Kill_by_Galaxy_Gunner = 367,
    Spot_Kill_R_Drone = 368,
    Squad_Spot_Kill_R_Drone = 369,
    Motion_Sensor_Spotter_Kill = 370,
    Kill_Player_Priority_Assist = 371,
    Kill_Player_High_Priority_Assist = 372,
    Gunner_Kill_Share_Player = 373,
    Gunner_Kill_Share_Flash = 374,
    Gunner_Kill_Share_Mana_Turret = 375,
    Gunner_Kill_Share_Phalanx = 376,
    Gunner_Kill_Share_Galaxy = 378, //Same time as kill assist on galaxy
    Gunner_Kill_Share_Liberator = 379,
    Gunner_Kill_Share_Lightning = 380,
    Gunner_Kill_Share_Magrider = 381,
    Gunner_Kill_Share_Mosquito = 382,
    Gunner_Kill_Share_Prowler = 383,
    Gunner_Kill_Share_Reaver = 384,
    Gunner_Kill_Share_Scythe = 385,
    Gunner_Kill_Share_Sunderer = 386,
    Gunner_Kill_Share_Vanguard = 387,
    Gunner_Kill_Share_Harasser = 388,
    Gunner_Kill_Assist_Share_Player = 389,
    Gunner_Kill_Assist_Share_Flash = 390,
    Gunner_Kill_Assist_Share_Mana_Turret = 391,
    Gunner_Kill_Assist_Share_Phalanx = 392,
    Gunner_Kill_Assist_Share_Drop_Pod = 393,
    Gunner_Kill_Assist_Share_Galaxy = 394,
    Gunner_Kill_Assist_Share_Lib = 395,
    Gunner_Kill_Assist_Share_Lightning = 396,
    Gunner_Kill_Assist_Share_Magrider = 397,
    Gunner_Kill_Assist_Share_Mosquito = 398,
    Gunner_Kill_Assist_Share_Prowler = 399,
    Gunner_Kill_Assist_Share_Reaver = 400,
    Gunner_Kill_Assist_Share_Scythe = 401,
    Gunner_Kill_Assist_Share_Sunderer = 402,
    Gunner_Kill_Assist_Share_Vanguard = 403,
    Gunner_Kill_Assist_Share_Harasser = 404,
    Passenger_Kill_Share_Player = 405,
    Passenger_Kill_Share_MANA = 407,
    Passenger_Kill_Share_Galaxy = 410,
    Passenger_Kill_Share_Reaver = 416,
    Passenger_Kill_Share_Scythe = 417,
    Passenger_Kill_Share_Sunderer = 418,
    Gunner_Kill_Assist_Share_Infantry_To_Pilot = 421,
    Gunner_Kill_Assist_Share_Phalanx_To_Pilot = 424,
    Passenger_Kill_Assist_Share_Galaxy = 426,
    Gunner_Kill_Assist_Share_Lightning_To_Pilot = 428,
    Gunner_Kill_Assist_Share_Reaver_To_Pilot = 432,
    Gunner_Kill_Assist_Share_Scythe_To_Pilot = 433,
    Gunner_Kill_Assist_Share_Sunderer_To_Pilot = 434,
    Shield_Regen_Tool_Kill = 437,
    Shield_Repair = 438,
    Squad_Shield_Repair = 439,
    Chain_Expl_Assist_Infantry = 440,
    Chain_Expl_Assist_Flash = 441,
    Vehicle_Destruction_Valkyrie = 501,
    Vehicle_Ram_Kill_Valkyrie = 502,
    Vehicle_Repair_Valkyrie = 503,
    Kill_Assist_Valkyrie = 504,
    Squad_Repair_Valkyrie = 505,
    Spot_Kill_Valkyrie = 506,
    Squad_Spot_Kill_Valkyrie = 507,
    Valkyrie_Damage = 508,
    Valkyrie_Kill_by_Sunderer_Gunner = 509,
    Valkyrie_Kill_by_Magrider_Gunner = 510,
    Valkyrie_Kill_by_Vanguard_Gunner = 511,
    Valkyrie_Kill_by_Prowler_Gunner = 512,
    Valkyrie_Kill_by_Liberator_Gunner = 513,
    Valkyrie_Kill_by_Galaxy_Gunner = 514,
    Player_Kill_by_Valkyrie_Gunner = 515,
    Gunner_Kill_Share_Valkyrie = 516,
    Gunner_Kill_Assist_Share_Valkyrie = 517, //probably a gunner kill assist / kill assist share / kill share XP of some flavor
    Passenger_Kill_Share_Valkyrie = 518,
    Gunner_Kill_Assit_Share_To_Pilot_Valkyrie = 519,
    Flash_Kill_by_Valkyrie_Gunner = 520,
    Sunderer_Kill_by_Valkyrie_Gunner = 521,
    Lightning_Kill_by_Valkyrie_Gunner = 522,
    Vanguard_Kill_by_Valkyrie_Gunner = 523,
    Prowler_Kill_by_Valkyrie_Gunner = 524,
    Reaver_Kill_by_Valkyrie_Gunner = 525,
    Mosquito_Kill_by_Valkyrie_Gunner = 526,
    Lib_Kill_by_Valkyrie = 527,
    Galaxy_Kill_by_Valkyrie_Gunner = 528,
    Magrider_Kill_by_Valkyrie_Gunner = 529,
    Scythe_Kill_by_Valkyrie_Gunner = 530,
    Snowman_Kill_by_Valkyrie_Gunner = 531,
    R_Drone_Kill_by_Valkyrie_Gunner = 532,
    Valkyrie_Kill_by_Valkyrie_Gunner = 533,
    Chain_Expl_Assist_Phalanx = 534,
    Chain_Expl_Assist_Drop_Pod = 535,
    Chain_Expl_Assist_Galaxy = 536,
    Chain_Expl_Assist_Liberator = 537,
    Chain_Expl_Assist_Lightning = 538,
    Chain_Expl_Assist_Magrider = 539,
    Chain_Expl_Assist_Mosquito = 540,
    Chain_Expl_Assist_Prowler = 541,
    Chain_Expl_Assist_Reaver = 542,
    Chain_Expl_Assist_Scythe = 543,
    Chain_Expl_Assist_Sunderer = 544,
    Chain_Expl_Assist_Vanguard = 545,
    Chain_Expl_Assist_Harasser = 546,
    Chain_Expl_Assist_R_Drone = 547,
    Chain_Expl_Assist_Valkyrie = 548,
    Concussion_Grenade_Assist = 550,
    Concussion_Grenade_Squad_Assist = 551,
    EMP_Grenade_Assist = 552,
    EMP_Grenade_Squad_Assist = 553,
    Flashbang_Assist = 554,
    Flashbang_Squad_Assist = 555,
    Objective_Pulse_Defend = 556,
    Objective_Pulse_Capture = 557,
    Halloween_Event_NPC_GreatP_Kill = 558,
    Pumpkin_Kill_by_Valkyrie_Gunner = 559,
    Pumpkin_Kill_by_Sunderer_Gunner = 560,
    Pumpkin_Kill_by_Magrider_Gunner = 561,
    Pumpkin_Kill_by_Vanguard_Gunner = 562,
    Pumpkin_Kill_by_Prowler_Gunner = 563,
    Pumpkin_Kill_by_Liberator_Gunner = 564,
    Pumpkin_Kill_by_Galaxy_Gunner = 565,
    Pumpkin_Kill_by_Harasser_Gunner = 566,
    GPumpkin_Kill_by_Sunder_Gunner = 567,
    GPumpkin_Kill_by_Mag_Gunner = 568,
    GPumpkin_Kill_by_Vang_Gunner = 569,
    GPumpkin_Kill_by_Prow_Gunner = 570,
    GPumpkin_Kill_by_Lib_Gunner = 571,
    GPumpkin_Kill_by_Gal_Gunner = 572,
    GPumpkin_Kill_by_Haras_Gunner = 573,
    Halloween_Event_NPC_Kill = 574,
    Harasser_Kill_by_Valkyrie_Gunner = 575,
    Valkyrie_Kill_by_Harasser_Gunner = 576,
    Snowman_kill_by_squad_member = 577,
    Gsnowman_kill_by_squad_member = 578,
    Destroy_Spitfire_Turret = 579,
    Vehicle_Ram_Kill_Spitfire_Turret = 580,
    Vehicle_Repair_Spitfire_Turret = 581,
    Kill_Assist_Spitfire_Turret = 582,
    Squad_Repair_Spitfire_Turret = 584,
    Spot_Kill_Spitfire_Turret = 585,
    Squad_Spot_Kill_Spitfire_Turret = 586,
    Gunner_Kill_Share_Spitfire_Turret = 587,
    Gunner_Kill_Assist_Share_Spitfire_Turret = 588,
    Kill_Dummy_NPC = 591,
    Savior_Kill_MAX = 592,
    Bounty_Kill_Bonus = 593,
    Bounty_Kill_Cashed_In = 594,
    Bounty_Kill_Streak = 595,
    Membership_bonus_xp = 596,
    Victory_Point_XP = 597,
    Continent_Capture_XP = 598,
    Victory_Point_XP25_percent_XP = 599,
    Victory_Point_XP50_percent_XP = 600,
    Victory_Point_XP75_percent_XP = 601,
    Victory_Point_XP_plus_50_percent_XP = 602,
    Vehicle_Ram_Kill_Construction_Med = 603,
    Kill_or_Destroy_Construction_Med = 604,
    Squad_Repair_Construction_Med = 605,
    Repair_Construction_Med = 606,
    KillAssist_Construction_Med = 607,
    Spot_Kill_Construction_Med = 609,
    Squad_Spot_Kill_Construction_Med = 610,
    Gunner_Kill_Share_Construction_Med = 611,
    Gunner_Kill_Assist_Share_Construction_Med = 612,
    Gunner_Kill_Assist_Share_To_Pilot_Construction_Med = 614,
    Vehicle_Ram_Kill_Construction_Small = 615,
    Kill_or_Destroy_Construction_Small = 616,
    Squad_Repair_Construction_Small = 617,
    Repair_Construction_Small = 618,
    KillAssist_Construction_Small = 619,
    Spot_Kill_Construction_Small = 621,
    Squad_Spot_Kill_Construction_Small = 622,
    Gunner_Kill_Share_Construction_Small = 623,
    Gunner_Kill_Assist_Share_Construction_Small = 624,
    Passenger_Kill_Share_Construction_Small = 625,
    Gunner_Kill_Assist_Share_To_Pilot_Construction_Small = 626,
    Vehicle_Ram_Kill_Construction_Large = 627,
    Kill_or_Destroy_Construction_Large = 628,
    Squad_Repair_Construction_Large = 629,
    Repair_Construction_Large = 630,
    KillAssist_Construction_Large = 631,
    Spot_Kill_Construction_Large = 633,
    Squad_Spot_Kill_Construction_Large = 634,
    Gunner_Kill_Share_Construction_Large = 635,
    Gunner_Kill_Assist_Share_Construction_Large = 636,
    Passenger_Kill_Share_Construction_Large = 637,
    Gunner_KIll_Assist_Share_To_Pilot_Construction_large = 638,
    Vehicle_Ram_Kill_Construction_Core = 639,
    Kill_or_Destroy_Construction_Core = 640,
    Squad_Repair_Construction_Core = 641,
    Repair_Construction_Core = 642,
    KillAssist_Construction_Core = 643,
    Spot_Kill_Construction_Core = 645,
    Squad_Spot_Kill_Construction_Core = 646,
    Vehicle_Destruction_ANT = 651,
    Vehicle_Ram_Kill_ANT = 652,
    Vehicle_Repair_ANT = 653,
    Kill_Assist_ANT = 654,
    Squad_Repair_ANT = 656,
    ANT_Kill_by_ANT_Gunner = 657,
    ANT_Kill_by_Magrider_Gunner = 658,
    ANT_Kill_by_Vanguard_Gunner = 659,
    ANT_Kill_by_Prowler_Gunner = 660,
    ANT_Kill_by_Lib_Gunner = 661,
    ANT_Kill_by_Galaxy_Gunner = 662,
    Spot_Kill_ANT = 663,
    Squad_Spot_Kill_ANT = 664,
    ANT_Damage_Infantry_vs_Vehicle = 665,
    ANT_Kill_by_Harasser_Gunner = 666,
    Gunner_Kill_Share_Ant = 667,
    Gunner_Kill_Assist_Share_Ant = 668,
    ANT_Kill_by_Valkyrie_Gunner = 671,
    Chain_Expl_Assist_ANT = 672,
    Bounty_Kill_Cashed_In_Alt = 673,
    Cortium_Harvest = 674,
    Cortium_Deposit = 675,
    Flash_Kill_by_ANT_Gunner = 676,
    Galaxy_Kill_by_ANT_Gunner = 677,
    Harasser_Kill_by_ANT_Gunner = 678,
    Magrider_Kill_by_ANT_Gunner = 679,
    Mosquito_Kill_by_ANT_Gunner = 680,
    Player_Kill_by_ANT_Gunner = 681,
    Prowler_Kill_by_ANT_Gunner = 682,
    Pumpkin_Kill_by_ANT_Gunner = 683,
    R_Drone_Kill_by_ANT_Gunner = 684,
    Reaver_Kill_by_ANT_Gunner = 685,
    Scythe_Kill_by_ANT_Gunner = 686,
    Snowman_Kill_by_ANT_Gunner = 687,
    Sunderer_Kill_by_ANT_Gunner = 688,
    Valkyrie_Kill_by_ANT_Gunner = 689,
    Vanguard_Kill_by_ANT_Gunner = 690,

    //HIVE mechanics are no longer in the game. The Census API does define the following XP gain ID
    //numbers, but we are very unlikely to encounter these in the future.
    /*
    Kill_Player_HIVE_XP_Source = 693,
    Kill_Player_HIVE_XP_Target = 694,
    Kill_Player_Assist_HIVE_Source = 695,
    Kill_Spawn_Ass_HIVE_Source = 696,
    Heal_Player_HIVE_Source = 697,
    MAX_Repair_HIVE_Source = 698,
    Revive_HIVE_Source = 699,
    Destroy_SecObj_HIVE_Source = 701,
    Destroy_SecAss_HIVE_Source = 702,
    Vehicle_Dest_Flash_HIVE_Source = 703,
    Transport_Assist_HIVE_Source = 705,
    Vehicle_Repair_Flash_HIVE_Source = 706,
    Resupply_Player_HIVE_Source = 707,
    Kill_Player_Assist_HIVE_XP_Target = 708,
    Kill_Spawn_Ass_HIVE_XP_Target = 709,
    Heal_Player_HIVE_XP_Target = 710,
    MAX_Repair_HIVE_XP_Target = 711,
    Revive_HIVE_XP_Target = 712,
    Destroy_SecObj_HIVE_XP_Target = 714,
    Destroy_SecAss_HIVE_XP_Target = 715,
    Vehicle_Dest_Flash_HIVE_XP_Target = 716,
    Transport_Assist_HIVE_XP_Target = 718,
    Vehicle_Repair_Flash_HIVE_XP_Target = 719,
    Resupply_Player_HIVE_XP_Target = 720,
    Destroy_Engineer_Turret_HIVE_XP_Source = 721,
    Vehicle_Destruction_Phalanx_HIVE_XP_Source = 722,
    Vehicle_Destruction_Drop_Pod_HIVE_XP_Source = 723,
    Vehicle_Destruction_Galaxy_HIVE_XP_Source = 724,
    Vehicle_Destruction_Liberator_HIVE_XP_Source = 725,
    Vehicle_Destruction_Lightning_HIVE_XP_Source = 726,
    Vehicle_Destruction_Magrider_HIVE_XP_Source = 727,
    Vehicle_Destruction_Mosquito_HIVE_XP_Source = 728,
    Vehicle_Destruction_Prowler_HIVE_XP_Source = 729,
    Vehicle_Destruction_Reaver_HIVE_XP_Source = 730,
    Vehicle_Destruction_Scythe_HIVE_XP_Source = 731,
    Vehicle_Destruction_Sunderer_HIVE_XP_Source = 732,
    Vehicle_Destruction_Vanguard_HIVE_XP_Source = 733,
    Vehicle_Repair_Engi_Turret_HIVE_XP_Source = 734,
    Vehicle_Repair_Phalanx_HIVE_XP_Source = 735,
    Vehicle_Repair_Drop_Pod_HIVE_XP_Source = 736,
    Vehicle_Repair_Galaxy_HIVE_XP_Source = 737,
    Vehicle_Repair_Liberator_HIVE_XP_Source = 738,
    Vehicle_Repair_Lightning_HIVE_XP_Source = 739,
    Vehicle_Repair_Magrider_HIVE_XP_Source = 740,
    Destroy_Engineer_Turret_HIVE_XP_Target = 741,
    Vehicle_Destruction_Phalanx_HIVE_XP_Target = 742,
    Vehicle_Destruction_Drop_Pod_HIVE_XP_Target = 743,
    Vehicle_Destruction_Galaxy_HIVE_XP_Target = 744,
    Vehicle_Destruction_Liberator_HIVE_XP_Target = 745,
    Vehicle_Destruction_Lightning_HIVE_XP_Target = 746,
    Vehicle_Destruction_Magrider_HIVE_XP_Target = 747,
    Vehicle_Destruction_Mosquito_HIVE_XP_Target = 748,
    Vehicle_Destruction_Prowler_HIVE_XP_Target = 749,
    Vehicle_Destruction_Reaver_HIVE_XP_Target = 750,
    Vehicle_Destruction_Scythe_HIVE_XP_Target = 751,
    Vehicle_Destruction_Sunderer_HIVE_XP_Target = 752,
    Vehicle_Destruction_Vanguard_HIVE_XP_Target = 753,
    Vehicle_Repair_Engi_Turret_HIVE_XP_Target = 754,
    Vehicle_Repair_Phalanx_HIVE_XP_Target = 755,
    Vehicle_Repair_Drop_Pod_HIVE_XP_Target = 756,
    Vehicle_Repair_Galaxy_HIVE_XP_Target = 757,
    Vehicle_Repair_Liberator_HIVE_XP_Target = 758,
    Vehicle_Repair_Lightning_HIVE_XP_Target = 759,
    Vehicle_Repair_Magrider_HIVE_XP_Target = 760,
    Vehicle_Repair_Mosquito_HIVE_XP_Source = 761,
    Vehicle_Repair_Prowler_HIVE_XP_Source = 762,
    Vehicle_Repair_Reaver_HIVE_XP_Source = 763,
    Vehicle_Repair_Scythe_HIVE_XP_Source = 764,
    Vehicle_Repair_Sunderer_HIVE_XP_Source = 765,
    Vehicle_Repair_Vanguard_HIVE_XP_Source = 766,
    Kill_Assist_Flash_HIVE_XP_Source = 767,
    Kill_Assist_Engi_Turret_HIVE_XP_Source = 768,
    Kill_Assist_Phalanx_HIVE_XP_Source = 769,
    Kill_Assist_Drop_Pod_HIVE_XP_Source = 770,
    Kill_Assist_Galaxy_HIVE_XP_Source = 771,
    Kill_Assist_Liberator_HIVE_XP_Source = 772,
    Kill_Assist_Lightning_HIVE_XP_Source = 773,
    Kill_Assist_Magrider_HIVE_XP_Source = 774,
    Kill_Assist_Mosquito_HIVE_XP_Source = 775,
    Kill_Assist_Prowler_HIVE_XP_Source = 776,
    Kill_Assist_Reaver_HIVE_XP_Source = 777,
    Kill_Assist_Scythe_HIVE_XP_Source = 778,
    Kill_Assist_Sunderer_HIVE_XP_Source = 779,
    Kill_Assist_Vanguard_HIVE_XP_Source = 780,
    Vehicle_Repair_Mosquito_HIVE_XP_Target = 781,
    Vehicle_Repair_Prowler_HIVE_XP_Target = 782,
    Vehicle_Repair_Reaver_HIVE_XP_Target = 783,
    Vehicle_Repair_Scythe_HIVE_XP_Target = 784,
    Vehicle_Repair_Sunderer_HIVE_XP_Target = 785,
    Vehicle_Repair_Vanguard_HIVE_XP_Target = 786,
    Kill_Assist_Flash_HIVE_XP_Target = 787,
    Kill_Assist_Engi_Turret_HIVE_XP_Target = 788,
    Kill_Assist_Phalanx_HIVE_XP_Target = 789,
    Kill_Assist_Drop_Pod_HIVE_XP_Target = 790,
    Kill_Assist_Galaxy_HIVE_XP_Target = 791,
    Kill_Assist_Liberator_HIVE_XP_Target = 792,
    Kill_Assist_Lightning_HIVE_XP_Target = 793,
    Kill_Assist_Magrider_HIVE_XP_Target = 794,
    Kill_Assist_Mosquito_HIVE_XP_Target = 795,
    Kill_Assist_Prowler_HIVE_XP_Target = 796,
    Kill_Assist_Reaver_HIVE_XP_Target = 797,
    Kill_Assist_Scythe_HIVE_XP_Target = 798,
    Kill_Assist_Sunderer_HIVE_XP_Target = 799,
    Kill_Assist_Vanguard_HIVE_XP_Target = 800,
    Drop_Pod_Kill_HIVE_XP_Source = 815,
    Player_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 816,
    Player_Kill_by_Magrider_Gunner_HIVE_XP_Source = 817,
    Player_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 818,
    Player_Kill_by_Prowler_Gunner_HIVE_XP_Source = 819,
    Player_Kill_by_Liberator_Gunner_HIVE_XP_Source = 820,
    Drop_Pod_Kill_HIVE_XP_Target = 835,
    Player_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 836,
    Player_Kill_by_Magrider_Gunner_HIVE_XP_Target = 837,
    Player_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 838,
    Player_Kill_by_Prowler_Gunner_HIVE_XP_Target = 839,
    Player_Kill_by_Liberator_Gunner_HIVE_XP_Target = 840,
    Player_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 841,
    Flash_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 842,
    Sunderer_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 843,
    Lightning_Kill_by_Sunderer_Gunne_HIVE_XP_Source = 844,
    Magrider_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 845,
    Vanguard_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 846,
    Prowler_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 847,
    Scythe_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 848,
    Reaver_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 849,
    Mosquito_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 850,
    Lib_Kill_by_Sunderer_HIVE_XP_Source = 851,
    Galaxy_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 852,
    Flash_Kill_by_Magrider_Gunner_HIVE_XP_Source = 853,
    Sunderer_Kill_by_Magrider_Gunner_HIVE_XP_Source = 854,
    Lightning_Kill_by_Magrider_Gunne_HIVE_XP_Source = 855,
    Vanguard_Kill_by_Magrider_Gunner_HIVE_XP_Source = 856,
    Prowler_Kill_by_Magrider_Gunner_HIVE_XP_Source = 857,
    Reaver_Kill_by_Magrider_Gunner_HIVE_XP_Source = 858,
    Mosquito_Kill_by_Magrider_Gunner_HIVE_XP_Source = 859,
    Lib_Kill_by_Magrider_HIVE_XP_Source = 860,
    Player_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 861,
    Flash_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 862,
    Sunderer_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 863,
    Lightning_Kill_by_Sunderer_Gunne_HIVE_XP_Target = 864,
    Magrider_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 865,
    Vanguard_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 866,
    Prowler_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 867,
    Scythe_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 868,
    Reaver_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 869,
    Mosquito_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 870,
    Lib_Kill_by_Sunderer_HIVE_XP_Target = 871,
    Galaxy_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 872,
    Flash_Kill_by_Magrider_Gunner_HIVE_XP_Target = 873,
    Sunderer_Kill_by_Magrider_Gunner_HIVE_XP_Target = 874,
    Lightning_Kill_by_Magrider_Gunne_HIVE_XP_Target = 875,
    Vanguard_Kill_by_Magrider_Gunner_HIVE_XP_Target = 876,
    Prowler_Kill_by_Magrider_Gunner_HIVE_XP_Target = 877,
    Reaver_Kill_by_Magrider_Gunner_HIVE_XP_Target = 878,
    Mosquito_Kill_by_Magrider_Gunner_HIVE_XP_Target = 879,
    Lib_Kill_by_Magrider_HIVE_XP_Target = 880,
    Galaxy_Kill_by_Magrider_Gunner_HIVE_XP_Source = 881,
    Flash_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 882,
    Sunderer_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 883,
    Lightning_Kill_by_Vanguard_Gunne_HIVE_XP_Source = 884,
    Magrider_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 885,
    Prowler_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 886,
    Scythe_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 887,
    Mosquito_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 888,
    Lib_Kill_by_Vanguard_HIVE_XP_Source = 889,
    Galaxy_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 890,
    Flash_Kill_by_Prowler_Gunner_HIVE_XP_Source = 891,
    Sunderer_Kill_by_Prowler_Gunner_HIVE_XP_Source = 892,
    Lightning_Kill_by_Prowler_Gunner_HIVE_XP_Source = 893,
    Magrider_Kill_by_Prowler_Gunner_HIVE_XP_Source = 894,
    Vanguard_Kill_by_Prowler_Gunner_HIVE_XP_Source = 895,
    Scythe_Kill_by_Prowler_Gunner_HIVE_XP_Source = 896,
    Reaver_Kill_by_Prowler_Gunner_HIVE_XP_Source = 897,
    Liberator_Kill_by_Prowler_Gunner_HIVE_XP_Source = 898,
    Galaxy_Kill_by_Prowler_Gunner_HIVE_XP_Source = 899,
    Galaxy_Kill_by_Magrider_Gunner_HIVE_XP_Target = 900,
    Flash_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 901,
    Sunderer_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 902,
    Lightning_Kill_by_Vanguard_Gunne_HIVE_XP_Target = 903,
    Magrider_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 904,
    Prowler_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 905,
    Scythe_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 906,
    Mosquito_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 907,
    Lib_Kill_by_Vanguard_HIVE_XP_Target = 908,
    Galaxy_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 909,
    Flash_Kill_by_Prowler_Gunner_HIVE_XP_Target = 910,
    Sunderer_Kill_by_Prowler_Gunner_HIVE_XP_Target = 911,
    Lightning_Kill_by_Prowler_Gunner_HIVE_XP_Target = 912,
    Magrider_Kill_by_Prowler_Gunner_HIVE_XP_Target = 913,
    Vanguard_Kill_by_Prowler_Gunner_HIVE_XP_Target = 914,
    Scythe_Kill_by_Prowler_Gunner_HIVE_XP_Target = 915,
    Reaver_Kill_by_Prowler_Gunner_HIVE_XP_Target = 916,
    Liberator_Kill_by_Prowler_Gunner_HIVE_XP_Target = 917,
    Galaxy_Kill_by_Prowler_Gunner_HIVE_XP_Target = 918,
    Flash_Kill_by_Liberator_Gunner_HIVE_XP_Source = 919,
    Sunderer_Kill_by_Lib_Gunner_HIVE_XP_Source = 920,
    Lightning_Kill_by_Liberator_Gunn_HIVE_XP_Source = 921,
    Magrider_Kill_by_Lib_Gunner_HIVE_XP_Source = 922,
    Vanguard_Kill_by_Lib_Gunner_HIVE_XP_Source = 923,
    Prowler_Kill_by_Liberator_Gunner_HIVE_XP_Source = 924,
    Scythe_Kill_by_Liberator_Gunner_HIVE_XP_Source = 925,
    Reaver_Kill_by_Liberator_Gunner_HIVE_XP_Source = 926,
    Mosquito_Kill_by_Lib_Gunner_HIVE_XP_Source = 927,
    Lib_Kill_by_Liberator_HIVE_XP_Source = 928,
    Galaxy_Kill_by_Liberator_Gunner_HIVE_XP_Source = 929,
    Flash_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 930,
    Sunderer_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 931,
    Lightning_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 932,
    Magrider_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 933,
    Vanguard_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 934,
    Prowler_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 935,
    Scythe_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 936,
    Reaver_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 937,
    Mosquito_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 938,
    Flash_Kill_by_Liberator_Gunner_HIVE_XP_Target = 939,
    Sunderer_Kill_by_Lib_Gunner_HIVE_XP_Target = 940,
    Lightning_Kill_by_Liberator_Gunn_HIVE_XP_Target = 941,
    Magrider_Kill_by_Lib_Gunner_HIVE_XP_Target = 942,
    Vanguard_Kill_by_Lib_Gunner_HIVE_XP_Target = 943,
    Prowler_Kill_by_Liberator_Gunner_HIVE_XP_Target = 944,
    Scythe_Kill_by_Liberator_Gunner_HIVE_XP_Target = 945,
    Reaver_Kill_by_Liberator_Gunner_HIVE_XP_Target = 946,
    Mosquito_Kill_by_Lib_Gunner_HIVE_XP_Target = 947,
    Lib_Kill_by_Liberator_HIVE_XP_Target = 948,
    Galaxy_Kill_by_Liberator_Gunner_HIVE_XP_Target = 949,
    Flash_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 950,
    Sunderer_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 951,
    Lightning_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 952,
    Magrider_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 953,
    Vanguard_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 954,
    Prowler_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 955,
    Scythe_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 956,
    Reaver_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 957,
    Mosquito_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 958,
    LibKill_by_Galaxy_Gunner_HIVE_XP_Source = 959,
    Galaxy_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 960,
    Terminal_Kill_HIVE_XP_Source = 961,
    Terminal_Repair_HIVE_XP_Source = 962,
    Galaxy_Damage_HIVE_XP_Source = 966,
    Liberator_Damage_HIVE_XP_Source = 967,
    Mosquito_Damage_HIVE_XP_Source = 969,
    Reaver_Damage_HIVE_XP_Source = 970,
    Scythe_Damage_HIVE_XP_Source = 971,
    Vehicle_Repair_Harasser_HIVE_XP_Source = 974,
    Kill_Assist_Harasser_HIVE_XP_Source = 975,
    Spot_Kill_Harasser_HIVE_XP_Source = 977,
    LibKill_by_Galaxy_Gunner_HIVE_XP_Target = 978,
    Galaxy_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 979,
    Terminal_Kill_HIVE_XP_Target = 980,
    Terminal_Repair_HIVE_XP_Target = 981,
    Galaxy_Damage_HIVE_XP_Target = 985,
    Liberator_Damage_HIVE_XP_Target = 986,
    Mosquito_Damage_HIVE_XP_Target = 988,
    Reaver_Damage_HIVE_XP_Target = 989,
    Scythe_Damage_HIVE_XP_Target = 990,
    Vehicle_Repair_Harasser_HIVE_XP_Target = 993,
    Kill_Assist_Harasser_HIVE_XP_Target = 994,
    Spot_Kill_Harasser_HIVE_XP_Target = 996,
    Harasser_Kill_by_Sunderer_Gunne_HIVE_XP_Source = 997,
    Harasser_Kill_by_Magrider_Gunne_HIVE_XP_Source = 998,
    Harasser_Kill_by_Vanguard_Gunne_HIVE_XP_Source = 999,
    Harasser_Kill_by_Prowler_Gunner_HIVE_XP_Source = 1000,
    Harasser_Kill_by_Liberator_Gunn_HIVE_XP_Source = 1001,
    Harasser_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 1002,
    Player_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1003,
    Flash_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1004,
    Sunderer_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1005,
    Lightning_Kill_by_Harasser_Gunne_HIVE_XP_Source = 1006,
    Vanguard_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1007,
    Prowler_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1008,
    Reaver_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1009,
    Mosquito_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1010,
    Lib_Kill_by_Harasser_HIVE_XP_Source = 1011,
    Galaxy_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1012,
    Harasser_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1013,
    Magrider_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1014,
    Scythe_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1015,
    Tank_Mine_Despawn_or_Defusal_HIVE_XP_Source = 1016,
    Harasser_Kill_by_Sunderer_Gunne_HIVE_XP_Target = 1017,
    Harasser_Kill_by_Magrider_Gunne_HIVE_XP_Target = 1018,
    Harasser_Kill_by_Vanguard_Gunne_HIVE_XP_Target = 1019,
    Harasser_Kill_by_Prowler_Gunner_HIVE_XP_Target = 1020,
    Harasser_Kill_by_Liberator_Gunn_HIVE_XP_Target = 1021,
    Harasser_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 1022,
    Player_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1023,
    Flash_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1024,
    Sunderer_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1025,
    Lightning_Kill_by_Harasser_Gunne_HIVE_XP_Target = 1026,
    Vanguard_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1027,
    Prowler_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1028,
    Reaver_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1029,
    Mosquito_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1030,
    Lib_Kill_by_Harasser_HIVE_XP_Target = 1031,
    Galaxy_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1032,
    Harasser_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1033,
    Magrider_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1034,
    Scythe_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1035,
    Tank_Mine_Despawn_or_Defusal_HIVE_XP_Target = 1036,
    Vehicle_Gunner_Kill_Share_Infantry_HIVE_XP_Source = 1037,
    Vehicle_Gunner_Kill_Share_Flash_HIVE_XP_Source = 1038,
    Vehicle_Gunner_Kill_Share_Engineer_Turret_HIVE_XP_Source = 1039,
    Vehicle_Gunner_Kill_Share_Phalanx_HIVE_XP_Source = 1040,
    Vehicle_Gunner_Kill_Share_Drop_Pod_HIVE_XP_Source = 1041,
    Vehicle_Gunner_Kill_Share_Galaxy_HIVE_XP_Source = 1042,
    Vehicle_Gunner_Kill_Share_Liberator_HIVE_XP_Source = 1043,
    Vehicle_Gunner_Kill_Share_Lightning_HIVE_XP_Source = 1044,
    Vehicle_Gunner_Kill_Share_Magrider_HIVE_XP_Source = 1045,
    Vehicle_Gunner_Kill_Share_Mosquito_HIVE_XP_Source = 1046,
    Vehicle_Gunner_Kill_Share_Prowler_HIVE_XP_Source = 1047,
    Vehicle_Gunner_Kill_Share_Reaver_HIVE_XP_Source = 1048,
    Vehicle_Gunner_Kill_Share_Scythe_HIVE_XP_Source = 1049,
    Vehicle_Gunner_Kill_Share_Sunderer_HIVE_XP_Source = 1050,
    Vehicle_Gunner_Kill_Share_Vanguard_HIVE_XP_Source = 1051,
    Vehicle_Gunner_Kill_Share_Harasser_HIVE_XP_Source = 1052,
    Vehicle_Gunner_Kill_Assist_Share_Infantry_HIVE_XP_Source = 1053,
    Vehicle_Gunner_Kill_Assist_Share_Flash_HIVE_XP_Source = 1054,
    Vehicle_Gunner_Kill_Assist_Share_Engi_Turret_HIVE_XP_Source = 1055,
    Vehicle_Gunner_Kill_Assist_Share_Phalanx_HIVE_XP_Source = 1056,
    Vehicle_Gunner_Kill_Share_Infantry_HIVE_XP_Target = 1057,
    Vehicle_Gunner_Kill_Share_Flash_HIVE_XP_Target = 1058,
    Vehicle_Gunner_Kill_Share_Engineer_Turret_HIVE_XP_Target = 1059,
    Vehicle_Gunner_Kill_Share_Phalanx_HIVE_XP_Target = 1060,
    Vehicle_Gunner_Kill_Share_Drop_Pod_HIVE_XP_Target = 1061,
    Vehicle_Gunner_Kill_Share_Galaxy_HIVE_XP_Target = 1062,
    Vehicle_Gunner_Kill_Share_Liberator_HIVE_XP_Target = 1063,
    Vehicle_Gunner_Kill_Share_Lightning_HIVE_XP_Target = 1064,
    Vehicle_Gunner_Kill_Share_Magrider_HIVE_XP_Target = 1065,
    Vehicle_Gunner_Kill_Share_Mosquito_HIVE_XP_Target = 1066,
    Vehicle_Gunner_Kill_Share_Prowler_HIVE_XP_Target = 1067,
    Vehicle_Gunner_Kill_Share_Reaver_HIVE_XP_Target = 1068,
    Vehicle_Gunner_Kill_Share_Scythe_HIVE_XP_Target = 1069,
    Vehicle_Gunner_Kill_Share_Sunderer_HIVE_XP_Target = 1070,
    Vehicle_Gunner_Kill_Share_Vanguard_HIVE_XP_Target = 1071,
    Vehicle_Gunner_Kill_Share_Harasser_HIVE_XP_Target = 1072,
    Vehicle_Gunner_Kill_Assist_Share_Infantry_HIVE_XP_Target = 1073,
    Vehicle_Gunner_Kill_Assist_Share_Flash_HIVE_XP_Target = 1074,
    Vehicle_Gunner_Kill_Assist_Share_Engi_Turret_HIVE_XP_Target = 1075,
    Vehicle_Gunner_Kill_Assist_Share_Phalanx_HIVE_XP_Target = 1076,
    Vehicle_Gunner_Kill_Assist_Share_Drop_Pod_HIVE_XP_Source = 1077,
    Vehicle_Gunner_Kill_Assist_Share_Galaxy_HIVE_XP_Source = 1078,
    Vehicle_Gunner_Kill_Assist_Share_Liberator_HIVE_XP_Source = 1079,
    Vehicle_Gunner_Kill_Assist_Share_Lightning_HIVE_XP_Source = 1080,
    Vehicle_Gunner_Kill_Assist_Share_Magrider_HIVE_XP_Source = 1081,
    Vehicle_Gunner_Kill_Assist_Share_Mosquito_HIVE_XP_Source = 1082,
    Vehicle_Gunner_Kill_Assist_Share_Prowler_HIVE_XP_Source = 1083,
    Vehicle_Gunner_Kill_Assist_Share_Reaver_HIVE_XP_Source = 1084,
    Vehicle_Gunner_Kill_Assist_Share_Scythe_HIVE_XP_Source = 1085,
    Vehicle_Gunner_Kill_Assist_Share_Sunderer_HIVE_XP_Source = 1086,
    Vehicle_Gunner_Kill_Assist_Share_Vanguard_HIVE_XP_Source = 1087,
    Vehicle_Gunner_Kill_Assist_Share_Harasser_HIVE_XP_Source = 1088,
    Vehicle_Passenger_Kill_Share_Infantry_HIVE_XP_Source = 1089,
    Vehicle_Passenger_Kill_Share_Flash_HIVE_XP_Source = 1090,
    Vehicle_Passenger_Kill_Share_Engineer_Turret_HIVE_XP_Source = 1091,
    Vehicle_Passenger_Kill_Share_Phalanx_HIVE_XP_Source = 1092,
    Vehicle_Passenger_Kill_Share_Drop_Pod_HIVE_XP_Source = 1093,
    Vehicle_Passenger_Kill_Share_Galaxy_HIVE_XP_Source = 1094,
    Vehicle_Passenger_Kill_Share_Liberator_HIVE_XP_Source = 1095,
    Vehicle_Passenger_Kill_Share_Lightning_HIVE_XP_Source = 1096,
    Vehicle_Gunner_Kill_Assist_Share_Drop_Pod_HIVE_XP_Target = 1097,
    Vehicle_Gunner_Kill_Assist_Share_Galaxy_HIVE_XP_Target = 1098,
    Vehicle_Gunner_Kill_Assist_Share_Liberator_HIVE_XP_Target = 1099,
    Vehicle_Gunner_Kill_Assist_Share_Lightning_HIVE_XP_Target = 1100,
    Vehicle_Gunner_Kill_Assist_Share_Magrider_HIVE_XP_Target = 1101,
    Vehicle_Gunner_Kill_Assist_Share_Mosquito_HIVE_XP_Target = 1102,
    Vehicle_Gunner_Kill_Assist_Share_Prowler_HIVE_XP_Target = 1103,
    Vehicle_Gunner_Kill_Assist_Share_Reaver_HIVE_XP_Target = 1104,
    Vehicle_Gunner_Kill_Assist_Share_Scythe_HIVE_XP_Target = 1105,
    Vehicle_Gunner_Kill_Assist_Share_Sunderer_HIVE_XP_Target = 1106,
    Vehicle_Gunner_Kill_Assist_Share_Vanguard_HIVE_XP_Target = 1107,
    Vehicle_Gunner_Kill_Assist_Share_Harasser_HIVE_XP_Target = 1108,
    Vehicle_Passenger_Kill_Share_Infantry_HIVE_XP_Target = 1109,
    Vehicle_Passenger_Kill_Share_Flash_HIVE_XP_Target = 1110,
    Vehicle_Passenger_Kill_Share_Engineer_Turret_HIVE_XP_Target = 1111,
    Vehicle_Passenger_Kill_Share_Phalanx_HIVE_XP_Target = 1112,
    Vehicle_Passenger_Kill_Share_Drop_Pod_HIVE_XP_Target = 1113,
    Vehicle_Passenger_Kill_Share_Galaxy_HIVE_XP_Target = 1114,
    Vehicle_Passenger_Kill_Share_Liberator_HIVE_XP_Target = 1115,
    Vehicle_Passenger_Kill_Share_Lightning_HIVE_XP_Target = 1116,
    Vehicle_Passenger_Kill_Share_Magrider_HIVE_XP_Source = 1117,
    Vehicle_Passenger_Kill_Share_Mosquito_HIVE_XP_Source = 1118,
    Vehicle_Passenger_Kill_Share_Prowler_HIVE_XP_Source = 1119,
    Vehicle_Passenger_Kill_Share_Reaver_HIVE_XP_Source = 1120,
    Vehicle_Passenger_Kill_Share_Scythe_HIVE_XP_Source = 1121,
    Vehicle_Passenger_Kill_Share_Sunderer_HIVE_XP_Source = 1122,
    Vehicle_Passenger_Kill_Share_Vanguard_HIVE_XP_Source = 1123,
    Vehicle_Passenger_Kill_Share_Harasser_HIVE_XP_Source = 1124,
    Vehicle_Driver_Kill_Assist_Share_Infantry_HIVE_XP_Source = 1125,
    Vehicle_Driver_Kill_Assist_Share_Flash_HIVE_XP_Source = 1126,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Source = 1127,
    Vehicle_Driver_Kill_Assist_Share_Phalanx_HIVE_XP_Source = 1128,
    Vehicle_Driver_Kill_Assist_Share_Drop_Pod_HIVE_XP_Source = 1129,
    Vehicle_Driver_Kill_Assist_Share_Galaxy_HIVE_XP_Source = 1130,
    Vehicle_Driver_Kill_Assist_Share_Liberator_HIVE_XP_Source = 1131,
    Vehicle_Driver_Kill_Assist_Share_Lightning_HIVE_XP_Source = 1132,
    Vehicle_Driver_Kill_Assist_Share_Magrider_HIVE_XP_Source = 1133,
    Vehicle_Driver_Kill_Assist_Share_Mosquito_HIVE_XP_Source = 1134,
    Vehicle_Driver_Kill_Assist_Share_Prowler_HIVE_XP_Source = 1135,
    Vehicle_Driver_Kill_Assist_Share_Reaver_HIVE_XP_Source = 1136,
    Vehicle_Passenger_Kill_Share_Magrider_HIVE_XP_Target = 1137,
    Vehicle_Passenger_Kill_Share_Mosquito_HIVE_XP_Target = 1138,
    Vehicle_Passenger_Kill_Share_Prowler_HIVE_XP_Target = 1139,
    Vehicle_Passenger_Kill_Share_Reaver_HIVE_XP_Target = 1140,
    Vehicle_Passenger_Kill_Share_Scythe_HIVE_XP_Target = 1141,
    Vehicle_Passenger_Kill_Share_Sunderer_HIVE_XP_Target = 1142,
    Vehicle_Passenger_Kill_Share_Vanguard_HIVE_XP_Target = 1143,
    Vehicle_Passenger_Kill_Share_Harasser_HIVE_XP_Target = 1144,
    Vehicle_Driver_Kill_Assist_Share_Infantry_HIVE_XP_Target = 1145,
    Vehicle_Driver_Kill_Assist_Share_Flash_HIVE_XP_Target = 1146,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Target = 1147,
    Vehicle_Driver_Kill_Assist_Share_Phalanx_HIVE_XP_Target = 1148,
    Vehicle_Driver_Kill_Assist_Share_Drop_Pod_HIVE_XP_Target = 1149,
    Vehicle_Driver_Kill_Assist_Share_Galaxy_HIVE_XP_Target = 1150,
    Vehicle_Driver_Kill_Assist_Share_Liberator_HIVE_XP_Target = 1151,
    Vehicle_Driver_Kill_Assist_Share_Lightning_HIVE_XP_Target = 1152,
    Vehicle_Driver_Kill_Assist_Share_Magrider_HIVE_XP_Target = 1153,
    Vehicle_Driver_Kill_Assist_Share_Mosquito_HIVE_XP_Target = 1154,
    Vehicle_Driver_Kill_Assist_Share_Prowler_HIVE_XP_Target = 1155,
    Vehicle_Driver_Kill_Assist_Share_Reaver_HIVE_XP_Target = 1156,
    Vehicle_Driver_Kill_Assist_Share_Scythe_HIVE_XP_Source = 1157,
    Vehicle_Driver_Kill_Assist_Share_Sunderer_HIVE_XP_Source = 1158,
    Vehicle_Driver_Kill_Assist_Share_Vanguard_HIVE_XP_Source = 1159,
    Vehicle_Driver_Kill_Assist_Share_Harasser_HIVE_XP_Source = 1160,
    Valkyrie_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 1161,
    Valkyrie_Kill_by_Magrider_Gunner_HIVE_XP_Source = 1162,
    Valkyrie_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 1163,
    Valkyrie_Kill_by_Prowler_Gunner_HIVE_XP_Source = 1164,
    Valkyrie_Kill_by_Liberator_Gunner_HIVE_XP_Source = 1165,
    Valkyrie_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 1166,
    Player_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1167,
    Vehicle_Gunner_Kill_Share_Valkyrie_HIVE_XP_Source = 1168,
    Vehicle_Gunner_Kill_Assist_Share_Valkyrie_HIVE_XP_Source = 1169,
    Vehicle_Passenger_Kill_Share_Valkyrie_HIVE_XP_Source = 1170,
    Vehicle_Driver_Kill_Assist_Share_Valkyrie_HIVE_XP_Source = 1171,
    Flash_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1172,
    Sunderer_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1173,
    Lightning_Kill_by_Valkyrie_Gunne_HIVE_XP_Source = 1174,
    Vanguard_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1175,
    Prowler_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1176,
    Vehicle_Driver_Kill_Assist_Share_Scythe_HIVE_XP_Target = 1177,
    Vehicle_Driver_Kill_Assist_Share_Sunderer_HIVE_XP_Target = 1178,
    Vehicle_Driver_Kill_Assist_Share_Vanguard_HIVE_XP_Target = 1179,
    Vehicle_Driver_Kill_Assist_Share_Harasser_HIVE_XP_Target = 1180,
    Valkyrie_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 1181,
    Valkyrie_Kill_by_Magrider_Gunner_HIVE_XP_Target = 1182,
    Valkyrie_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 1183,
    Valkyrie_Kill_by_Prowler_Gunner_HIVE_XP_Target = 1184,
    Valkyrie_Kill_by_Liberator_Gunner_HIVE_XP_Target = 1185,
    Valkyrie_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 1186,
    Player_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1187,
    Vehicle_Gunner_Kill_Share_Valkyrie_HIVE_XP_Target = 1188,
    Vehicle_Gunner_Kill_Assist_Share_Valkyrie_HIVE_XP_Target = 1189,
    Vehicle_Passenger_Kill_Share_Valkyrie_HIVE_XP_Target = 1190,
    Vehicle_Driver_Kill_Assist_Share_Valkyrie_HIVE_XP_Target = 1191,
    Flash_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1192,
    Sunderer_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1193,
    Lightning_Kill_by_Valkyrie_Gunne_HIVE_XP_Target = 1194,
    Vanguard_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1195,
    Prowler_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1196,
    Reaver_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1197,
    Mosquito_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1198,
    Lib_Kill_by_Valkyrie_HIVE_XP_Source = 1199,
    Galaxy_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1200,
    Magrider_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1201,
    Scythe_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1202,
    Snowman_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1203,
    Valkyrie_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1204,
    Harasser_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1205,
    Valkyrie_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1206,
    Destroy_Spitfire_Turret_HIVE_XP_Source = 1207,
    Vehicle_Gunner_Kill_Share_Spitfire_HIVE_XP_Source = 1208,
    Vehicle_Gunner_Kill_Assist_Share_Spitfire_HIVE_XP_Source = 1209,
    Vehicle_Passenger_Kill_Share_Spitfire_HIVE_XP_Source = 1210,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Source = 1211,
    Kill_or_Destroy_Construction_Med_HIVE_XP_Source = 1212,
    Repair_Construction_Med_HIVE_XP_Source = 1214,
    KillAssist_Construction_Med_HIVE_XP_Source = 1215,
    Reaver_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1217,
    Mosquito_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1218,
    Lib_Kill_by_Valkyrie_HIVE_XP_Target = 1219,
    Galaxy_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1220,
    Magrider_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1221,
    Scythe_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1222,
    Snowman_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1223,
    Valkyrie_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1224,
    Harasser_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1225,
    Valkyrie_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1226,
    Destroy_Spitfire_Turret_HIVE_XP_Target = 1227,
    Vehicle_Gunner_Kill_Share_Spitfire_HIVE_XP_Target = 1228,
    Vehicle_Gunner_Kill_Assist_Share_Spitfire_HIVE_XP_Target = 1229,
    Vehicle_Passenger_Kill_Share_Spitfire_HIVE_XP_Target = 1230,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Target = 1231,
    Kill_or_Destroy_Construction_Med_HIVE_XP_Target = 1232,
    Repair_Construction_Med_HIVE_XP_Target = 1234,
    KillAssist_Construction_Med_HIVE_XP_Target = 1235,
    Spot_Kill_Construction_Med__HIVE_XP_Source = 1237,
    Gunner_Kill_Share_Construction_Med_HIVE_XP_Source = 1238,
    Gunner_Kill_Assist_Share_Construction_Med_HIVE_XP_Source = 1239,
    Vehicle_Passenger_Kill_Share_Construction_Med_HIVE_XP_Source = 1240,
    Driver_Kill_Aassist_Share_Construction_Med_HIVE_XP_Source = 1241,
    Vehicle_Ram_Kill_Construction_Small_HIVE_XP_Source = 1242,
    Kill_or_Destroy_Construction_Small_HIVE_XP_Source = 1243,
    Repair_Construction_Small_HIVE_XP_Source = 1245,
    KillAssist_Construction_Small_HIVE_XP_Source = 1246,
    Spot_Kill_Construction_Small__HIVE_XP_Source = 1248,
    Gunner_Kill_Share_Construction_Small_HIVE_XP_Source = 1249,
    Gunner_Kill_Assist_Share_Construction_Small_HIVE_XP_Source = 1250,
    Vehicle_Passenger_Kill_Share_Construction_Small_HIVE_XP_Source = 1251,
    Driver_Kill_Aassist_Share_Construction_Small_HIVE_XP_Source = 1252,
    Vehicle_Ram_Kill_Construction_Large_HIVE_XP_Source = 1253,
    Kill_or_Destroy_Construction_Large_HIVE_XP_Source = 1254,
    Spot_Kill_Construction_Med__HIVE_XP_Target = 1255,
    Gunner_Kill_Share_Construction_Med_HIVE_XP_Target = 1256,
    Gunner_Kill_Assist_Share_Construction_Med_HIVE_XP_Target = 1257,
    Vehicle_Passenger_Kill_Share_Construction_Med_HIVE_XP_Target = 1258,
    Driver_Kill_Aassist_Share_Construction_Med_HIVE_XP_Target = 1259,
    Vehicle_Ram_Kill_Construction_Small_HIVE_XP_Target = 1260,
    Kill_or_Destroy_Construction_Small_HIVE_XP_Target = 1261,
    Repair_Construction_Small_HIVE_XP_Target = 1263,
    KillAssist_Construction_Small_HIVE_XP_Target = 1264,
    Spot_Kill_Construction_Small__HIVE_XP_Target = 1266,
    Gunner_Kill_Share_Construction_Small_HIVE_XP_Target = 1267,
    Gunner_Kill_Assist_Share_Construction_Small_HIVE_XP_Target = 1268,
    Vehicle_Passenger_Kill_Share_Construction_Small_HIVE_XP_Target = 1269,
    Driver_Kill_Aassist_Share_Construction_Small_HIVE_XP_Target = 1270,
    Vehicle_Ram_Kill_Construction_Large_HIVE_XP_Target = 1271,
    Kill_or_Destroy_Construction_Large_HIVE_XP_Target = 1272,
    Repair_Construction_Large_HIVE_XP_Source = 1273,
    KillAssist_Construction_Large_HIVE_XP_Source = 1274,
    Spot_Kill_Construction_Large__HIVE_XP_Source = 1276,
    Gunner_Kill_Share_Construction_Large_HIVE_XP_Source = 1277,
    Gunner_Kill_Assist_Share_Construction_Large_HIVE_XP_Source = 1278,
    Vehicle_Passenger_Kill_Share_Construction_Large_HIVE_XP_Source = 1279,
    Driver_Kill_Aassist_Share_Construction_Large_HIVE_XP_Source = 1280,
    Vehicle_Ram_Kill_Construction_Core_HIVE_XP_Source = 1281,
    Kill_or_Destroy_Construction_Core_HIVE_XP_Source = 1282,
    Repair_Construction_Core_HIVE_XP_Source = 1283,
    KillAssist_Construction_Core_HIVE_XP_Source = 1284,
    Spot_Kill_Construction_Core__HIVE_XP_Source = 1286,
    Gunner_Kill_Share_Construction_Core_HIVE_XP_Source = 1287,
    Gunner_Kill_Assist_Share_Construction_Core_HIVE_XP_Source = 1288,
    Repair_Construction_Large_HIVE_XP_Target = 1289,
    KillAssist_Construction_Large_HIVE_XP_Target = 1290,
    Spot_Kill_Construction_Large__HIVE_XP_Target = 1292,
    Gunner_Kill_Share_Construction_Large_HIVE_XP_Target = 1293,
    Gunner_Kill_Assist_Share_Construction_Large_HIVE_XP_Target = 1294,
    Vehicle_Passenger_Kill_Share_Construction_Large_HIVE_XP_Target = 1295,
    Driver_Kill_Aassist_Share_Construction_Large_HIVE_XP_Target = 1296,
    Vehicle_Ram_Kill_Construction_Core_HIVE_XP_Target = 1297,
    Kill_Destroy_Construction_Core_HIVE_XP_Target = 1298,
    Repair_Construction_Core_HIVE_XP_Target = 1299,
    KillAssist_Construction_Core_HIVE_XP_Target = 1300,
    Spot_Kill_Construction_Core__HIVE_XP_Target = 1302,
    Gunner_Kill_Share_Construction_Core_HIVE_XP_Target = 1303,
    Gunner_Kill_Assist_Share_Construction_Core_HIVE_XP_Target = 1304,
    Vehicle_Passenger_Kill_Share_Construction_Core_HIVE_XP_Source = 1305,
    Driver_Kill_Aassist_Share_Construction_Core_HIVE_XP_Source = 1306,
    Vehicle_Destruction_ANT_HIVE_XP_Source = 1307,
    Vehicle_Ram_Kill_ANT_HIVE_XP_Source = 1308,
    Vehicle_Repair_ANT_HIVE_XP_Source = 1309,
    Kill_Assist_ANT_HIVE_XP_Source = 1310,
    ANT_Kill_by_ANT_Gunner_HIVE_XP_Source = 1312,
    ANT_Kill_by_Magrider_Gunner_HIVE_XP_Source = 1313,
    ANT_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 1314,
    ANT_Kill_by_Prowler_Gunner_HIVE_XP_Source = 1315,
    ANT_Kill_by_Lib_Gunner_HIVE_XP_Source = 1316,
    ANT_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 1317,
    Spot_Kill_ANT_HIVE_XP_Source = 1318,
    ANT_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1320,
    Vehicle_Gunner_Kill_Share_ANT_HIVE_XP_Source = 1321,
    Vehicle_Gunner_Kill_Assist_Share_ANT_HIVE_XP_Source = 1322,
    Vehicle_Passenger_Kill_Share_ANT_HIVE_XP_Source = 1323,
    Vehicle_Driver_Kill_Assist_Share_ANT_HIVE_XP_Source = 1324,
    Vehicle_Passenger_Kill_Share_Construction_Core_HIVE_XP_Target = 1325,
    Driver_Kill_Aassist_Share_Construction_Core_HIVE_XP_Target = 1326,
    Vehicle_Destruction_ANT_HIVE_XP_Target = 1327,
    Vehicle_Ram_Kill_ANT_HIVE_XP_Target = 1328,
    Vehicle_Repair_ANT_HIVE_XP_Target = 1329,
    Kill_Assist_ANT_HIVE_XP_Target = 1330,
    ANT_Kill_by_ANT_Gunner_HIVE_XP_Target = 1332,
    ANT_Kill_by_Magrider_Gunner_HIVE_XP_Target = 1333,
    ANT_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 1334,
    ANT_Kill_by_Prowler_Gunner_HIVE_XP_Target = 1335,
    ANT_Kill_by_Lib_Gunner_HIVE_XP_Target = 1336,
    ANT_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 1337,
    Spot_Kill_ANT_HIVE_XP_Target = 1338,
    ANT_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1340,
    Vehicle_Gunner_Kill_Share_ANT_HIVE_XP_Target = 1341,
    Vehicle_Gunner_Kill_Assist_Share_ANT_HIVE_XP_Target = 1342,
    Vehicle_Passenger_Kill_Share_ANT_HIVE_XP_Target = 1343,
    Vehicle_Driver_Kill_Assist_Share_ANT_HIVE_XP_Target = 1344,
    ANT_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1345,
    Flash_Kill_by_ANT_Gunner_HIVE_XP_Source = 1346,
    Galaxy_Kill_by_ANT_Gunner_HIVE_XP_Source = 1347,
    Harasser_Kill_by_ANT_Gunner_HIVE_XP_Source = 1348,
    Magrider_Kill_by_ANT_Gunner_HIVE_XP_Source = 1349,
    Mosquito_Kill_by_ANT_Gunner_HIVE_XP_Source = 1350,
    Player_Kill_by_ANT_Gunner_HIVE_XP_Source = 1351,
    Prowler_Kill_by_ANT_Gunner_HIVE_XP_Source = 1352,
    Pumpkin_Kill_by_ANT_Gunner_HIVE_XP_Source = 1353,
    Reaver_Kill_by_ANT_Gunner_HIVE_XP_Source = 1354,
    Scythe_Kill_by_ANT_Gunner_HIVE_XP_Source = 1355,
    Sunderer_Kill_by_ANT_Gunner_HIVE_XP_Source = 1356,
    Valkyrie_Kill_by_ANT_Gunner_HIVE_XP_Source = 1357,
    Vanguard_Kill_by_ANT_Gunner_HIVE_XP_Source = 1358,
    ANT_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1359,
    Flash_Kill_by_ANT_Gunner_HIVE_XP_Target = 1360,
    Galaxy_Kill_by_ANT_Gunner_HIVE_XP_Target = 1361,
    Harasser_Kill_by_ANT_Gunner_HIVE_XP_Target = 1362,
    Magrider_Kill_by_ANT_Gunner_HIVE_XP_Target = 1363,
    Mosquito_Kill_by_ANT_Gunner_HIVE_XP_Target = 1364,
    Player_Kill_by_ANT_Gunner_HIVE_XP_Target = 1365,
    Prowler_Kill_by_ANT_Gunner_HIVE_XP_Target = 1366,
    Reaver_Kill_by_ANT_Gunner_HIVE_XP_Target = 1368,
    Scythe_Kill_by_ANT_Gunner_HIVE_XP_Target = 1369,
    Sunderer_Kill_by_ANT_Gunner_HIVE_XP_Target = 1370,
    Valkyrie_Kill_by_ANT_Gunner_HIVE_XP_Target = 1371,
    Vanguard_Kill_by_ANT_Gunner_HIVE_XP_Target = 1372,*/
    Destroy_Hardlight_Barrier = 1373,
    Vehicle_Ram_Kill_Hardlight_Barrier = 1374,
    Vehicle_Repair_Hardlight_Barrier = 1375,
    Kill_Assist_Hardlight_Barrier = 1376,
    Squad_Repair_Hardlight_Barrier = 1378,
    Gunner_Kill_Share_Hardlight_Barrier = 1379,
    /*
    Destroy_Hardlight_Barrier_HIVE_XP_Source = 1383,
    Vehicle_Gunner_Kill_Share_Hardlight_Barrier_HIVE_XP_Source = 1384,
    Vehicle_Gunner_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Source = 1385,
    Vehicle_Passenger_Kill_Share_Hardlight_Barrier_HIVE_XP_Source = 1386,
    Vehicle_Driver_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Source = 1387,
    Destroy_Hardlight_Barrier_HIVE_XP_Target = 1388,
    Vehicle_Gunner_Kill_Share_Hardlight_Barrier_HIVE_XP_Target = 1389,
    Vehicle_Gunner_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Target = 1390,
    Vehicle_Passenger_Kill_Share_Hardlight_Barrier_HIVE_XP_Target = 1391,
    Vehicle_Driver_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Target = 1392,
    */
    Missing_1380 = 1380,
    //Hardlight_Cover_Blocking_Exp_placeholder_until_code_is_done = 1393,
    Draw_Fire_Assist = 1393,
    Draw_Fire_Award = 1394,
    Flash_Damage_Infantry_vs_Vehicle = 1395,
    Harasser_Damage_Infantry_vs_Vehicle = 1396,
    Router_Kill = 1409,
    Generic_Npc_Spawn = 1410,
    Event_Anomaly_Defense_Pulse = 1420,
    EQ20_Collectible = 1430,
    EQ20_Nightshade = 1440,
    Vehicle_Destruction_Colossus = 1449,
    Vehicle_Ram_Kill_Colossus = 1450,
    Vehicle_Repair_Colossus = 1451,
    Squad_Repair_Colossus = 1452,
    Kill_Assist_Colossus = 1453,
    Tank_Hunter_Bonus_Colossus_Kill = 1454,
    Chain_Expl_Assist_Colossus = 1455,
    Squad_Spot_Kill_Colossus = 1456,
    Spot_Kill_Colossus = 1457,
    Colossus_Damage_Infantry_vs_Vehicle = 1458,
    Flash_Kill_by_Colossus_Gunner = 1459,
    Vanguard_Kill_by_Colossus_Gunner = 1460,
    ANT_Kill_by_Colossus_Gunner = 1461,
    Galaxy_Kill_by_Colossus_Gunner = 1462,
    Harasser_Kill_by_Colossus_Gunner = 1463,
    Magrider_Kill_by_Colossus_Gunner = 1464,
    Mosquito_Kill_by_Colossus_Gunner = 1465,
    Player_Kill_by_Colossus_Gunner = 1466,
    Prowler_Kill_by_Colossus_Gunner = 1467,
    Pumpkin_Kill_by_Colossus_Gunner = 1468,
    R_Drone_Kill_by_Colossus_Gunner = 1469,
    Reaver_Kill_by_Colossus_Gunner = 1470,
    Scythe_Kill_by_Colossus_Gunner = 1471,
    Snowman_Kill_by_Colossus_Gunner = 1472,
    Sunderer_Kill_by_Colossus_Gunner = 1473,
    Valkyrie_Kill_by_Colossus_Gunner = 1474,
    Colossus_Kill_by_Colossus_Gunner = 1475,
    Gunner_Kill_Assist_Share_Colossus = 1477,
    Vehicle_Destruction_Javelin = 1480,
    Squad_Repair_Javelin = 1481,
    Vehicle_Repair_Javelin = 1482,
    Vehicle_Ram_Bonus_Javelin = 1483,
    Kill_Assist_Javelin = 1484,
    Javelin_Kill_by_Sunderer_Gunner = 1486,
    Javelin_Kill_by_Magrider_Gunner = 1487,
    Javelin_Kill_by_Vanguard_Gunner = 1488,
    Javelin_Kill_by_Prowler_Gunner = 1489,
    Javelin_Kill_by_Liberator_Gunner = 1490,
    Javelin_Kill_by_Galaxy_Gunner = 1491,
    Spot_Kill_Javelin = 1492,
    Squad_Spot_Kill_Javelin = 1493,
    Javelin_Kill_by_Harasser_Gunner = 1494,
    Gunner_Kill_Assist_Share_Javelin = 1496,
    Chain_Expl_Assist_Javelin = 1499,
    Javelin_Kill_by_Valkyrie_Gunner = 1500,
    Javelin_Kill_by_ANT_Gunner = 1501,
    Javelin_Damage_Infantry_vs_Vehicle = 1502,
    ANT_Kill_by_Bastion_Gunner = 1503,
    Colossus_Kill_by_Bastion_Gunner = 1504,
    Flash_Kill_by_Bastion_Gunner = 1505,
    Galaxy_Kill_by_Bastion_Gunner = 1506,
    Harasser_Kill_by_Bastion_Gunner = 1507,
    Magrider_Kill_by_Bastion_Gunner = 1508,
    Mosquito_Kill_by_Bastion_Gunner = 1509,
    Player_Kill_by_Bastion_Gunner = 1510,
    Prowler_Kill_by_Bastion_Gunner = 1511,
    Pumpkin_Kill_by_Bastion_Gunner = 1512,
    R_Drone_Kill_by_Bastion_Gunner = 1513,
    Reaver_Kill_by_Bastion_Gunner = 1514,
    Scythe_Kill_by_Bastion_Gunner = 1515,
    Snowman_Kill_by_Bastion_Gunner = 1516,
    Sunderer_Kill_by_Bastion_Gunner = 1517,
    Valkyrie_Kill_by_Bastion_Gunner = 1518,
    Vanguard_Kill_by_Bastion_Gunner = 1519,
    Vehicle_Component_Destruction_Bastion = 1520,
    Kill_Assist_Bastion_Component = 1521,
    Gunner_Kill_Assist_Share_Bastion_Component = 1523,
    Bastion_Component_Kill_by_Sunderer_Gunner = 1526,
    Bastion_Component_Kill_by_Vanguard_Gunner = 1527,
    Bastion_Component_Kill_by_Prowler_Gunner = 1528,
    Bastion_Component_Kill_by_Lib_Gunner = 1529,
    Bastion_Component_Kill_by_Galaxy_Gunner = 1530,
    Bastion_Component_Kill_by_Harasser_Gunner = 1531,
    Bastion_Component_Kill_by_Valkyrie_Gunner = 1532,
    Bastion_Component_Kill_by_ANT_Gunner = 1533,
    Bastion_Component_Kill_by_Colossus_Gunner = 1534,
    Bastion_Component_Kill_by_Bastion_Gunner = 1535,
    Bastion_Component_Damage = 1536,
    Destroy_Lightning_Arrester = 1537,
    Lightning_Arrester_Absorb_Xp = 1538,
    Convoy_Periodic_Update = 1539,
    Convoy_Convoy_Complete = 1540,
    Sunderer_Campaign_Spawn_Bonus = 1542,
    Galaxy_Campaign_Spawn_Bonus = 1543,
    tutorial2_grantBr1 = 1544,
    Door_Lock_Repair = 1545,
    Destroy_Shield_Door_Lock = 1546,
    Containment_Site_Gate_Shield_Gen_Destroy = 1547,
    Containment_Site_Gate_Shield_Gen_Destroy_Assist = 1548,
    Containment_Site_Gate_Shield_Gen_Repair = 1549,
    War_Asset_Destruction_Standard = 1553, //Destroy Anvil? Yes!
    War_Asset_Destruction_Valuable = 1554,
    War_Asset_Destruction_Epic = 1555,
    Vehicle_Damage_Chimera = 1557,
    Kill_Assist_Chimera = 1560,

    Missing_1563 = 1563, //Triggered at same time as killing a Chimera from Prowler top gun (swamped out by other xp events?
    Tank_Superiority_Bonus = 1564,
    Vehicle_Destruction_Chimera = 1565,
    Gunner_Kill_Assist_Share_Chimera = 1567,
    Gunner_Kill_Share_Chimera = 1568,
    Vehicle_Repair_Chimera = 1571,

    Vehicle_Destruction_Dervish = 1635,
    Kill_Assist_Dervish = 1636,
    Missing_1646 = 1646, //Triggered after dervish kill from galaxy gun turret
    Surface_To_Air_Dervish = 1647,
    Dervish_Damage = 1657,
    Fighter_Superiority_Bonus = 1649,
    Gunner_Kill_Share_Dervish = 1650,
    Gunner_Kill_Assist_Share_Dervish = 1651,

    Vehicle_Destruction_Corsair = 1989,
    Kill_Assist_Corsair = 1992,
    Gunner_Kill_Assist_Share_Corsair_Alt = 2005,
    Gunner_Kill_Assist_Share_Corsair = 2006,
    Gunner_To_Pilot_Kill_Assist_Share_Corsair = 2008,
    Gunner_Kill_Corsair_Bonus = 2055,
    Gunner_Kill_Bonus = 2074,
    Podium_Defense_Bonus = 2132,
    Conduit_Repository_Hack = 2135,

    #[num_enum(default)]
    Unknown = 0,
}

#[allow(dead_code)]
impl ExperienceType {
    pub fn is_repair(&self) -> bool {
        matches!(
            self,
            ExperienceType::MAX_Repair
                | ExperienceType::Squad_Repair_Flash
                | ExperienceType::Vehicle_Repair_Flash
                | ExperienceType::Secondary_Facility_Object_Repair
                | ExperienceType::Vehicle_Repair_Engi_Turret
                | ExperienceType::Vehicle_Repair_Phalanx
                | ExperienceType::Vehicle_Repair_Drop_Pod
                | ExperienceType::Vehicle_Repair_Galaxy
                | ExperienceType::Vehicle_Repair_Liberator
                | ExperienceType::Vehicle_Repair_Lightning
                | ExperienceType::Vehicle_Repair_Magrider
                | ExperienceType::Vehicle_Repair_Mosquito
                | ExperienceType::Vehicle_Repair_Prowler
                | ExperienceType::Vehicle_Repair_Reaver
                | ExperienceType::Vehicle_Repair_Scythe
                | ExperienceType::Vehicle_Repair_Sunderer
                | ExperienceType::Vehicle_Repair_Vanguard
                | ExperienceType::Squad_Repair_Engi_Turret
                | ExperienceType::Squad_Repair_Phalanx
                | ExperienceType::Squad_Repair_Drop_Pod
                | ExperienceType::Squad_Repair_Galaxy
                | ExperienceType::Squad_Repair_Liberator
                | ExperienceType::Squad_Repair_Lightning
                | ExperienceType::Squad_Repair_Magrider
                | ExperienceType::Squad_Repair_Mosquito
                | ExperienceType::Squad_Repair_Prowler
                | ExperienceType::Squad_Repair_Reaver
                | ExperienceType::Squad_Repair_Scythe
                | ExperienceType::Squad_Repair_Sunderer
                | ExperienceType::Squad_Repair_Vanguard
                | ExperienceType::Terminal_Repair
                | ExperienceType::Squad_MAX_Repair
                | ExperienceType::Squad_Repair_Harasser
                | ExperienceType::Vehicle_Repair_Harasser
                | ExperienceType::Squad_Repair_R_Drone
                | ExperienceType::Vehicle_Repair_R_Drone
                | ExperienceType::Vehicle_Repair_Valkyrie
                | ExperienceType::Squad_Repair_Valkyrie
                | ExperienceType::Vehicle_Repair_Spitfire_Turret
                | ExperienceType::Squad_Repair_Spitfire_Turret
                | ExperienceType::Squad_Repair_Construction_Med
                | ExperienceType::Repair_Construction_Med
                | ExperienceType::Squad_Repair_Construction_Small
                | ExperienceType::Repair_Construction_Small
                | ExperienceType::Squad_Repair_Construction_Large
                | ExperienceType::Repair_Construction_Large
                | ExperienceType::Squad_Repair_Construction_Core
                | ExperienceType::Repair_Construction_Core
                | ExperienceType::Vehicle_Repair_ANT
                | ExperienceType::Squad_Repair_ANT
                | ExperienceType::Vehicle_Repair_Colossus
                | ExperienceType::Squad_Repair_Colossus
                | ExperienceType::Vehicle_Repair_Javelin
                | ExperienceType::Squad_Repair_Javelin
                | ExperienceType::Door_Lock_Repair
                | ExperienceType::Containment_Site_Gate_Shield_Gen_Repair
        )
    }

    pub fn is_missing(&self) -> bool {
        matches!(
            self,
            ExperienceType::Missing_1380
                | ExperienceType::Missing_1563
                | ExperienceType::Missing_1646
        )
    }
}

impl std::fmt::Display for ExperienceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExperienceType::Kill_Player => write!(f, "Kill XP"),
            ExperienceType::Kill_Player_Assist => write!(f, "Kill Assist XP"),
            ExperienceType::Kill_Player_Spawn_Assist => write!(f, "Kill Spawn Assist XP"),
            ExperienceType::Heal_Player => write!(f, "Heal XP"),
            ExperienceType::Heal_Assist => write!(f, "Heal Assist XP"),
            ExperienceType::MAX_Repair => write!(f, "MAX repair XP"),
            ExperienceType::Revive => write!(f, "Revive XP"),
            ExperienceType::Kill_Streak => write!(f, "Kill Streak XP"),
            ExperienceType::Domination_Kill => write!(f, "Domination Kill XP"),
            ExperienceType::Revenge_Kill => write!(f, "Revenge Kill XP"),
            ExperienceType::Control_Point_Defend => write!(f, "Control Point Defend XP"),
            ExperienceType::Control_Point_Attack => write!(f, "Control Point Attack XP"),
            ExperienceType::Facility_Captured_Not_Used => write!(f, "Facility Capture XP"),
            ExperienceType::Facility_Defense => write!(f, "Facility Defense XP"),
            ExperienceType::Destroy_Secondary_Objective => {
                write!(f, "Destroy Secondary Objective XP")
            }
            ExperienceType::Destroy_SecondaryObjectiveAssist => {
                write!(f, "Assist Destroy 2nd Objective XP")
            }
            ExperienceType::Vehicle_Destruction_Flash => write!(f, "Flash Destroy XP"),
            ExperienceType::Multiple_Kill => write!(f, "Multikill XP"),
            ExperienceType::Vehicle_RoadKill => write!(f, "Roadkill XP"),
            ExperienceType::Squad_Repair_Flash => write!(f, "Repair Squad Flash XP"),
            ExperienceType::Kill_Player_Class_MAX => write!(f, "Kill MAX XP"),
            ExperienceType::Transport_Assist => write!(f, "Transport Assist XP"),
            ExperienceType::Vehicle_Repair_Flash => write!(f, "Repair Flash XP"),
            ExperienceType::Nemesis_Kill => write!(f, "Nemesis Kill XP"),
            ExperienceType::Resupply_Player => write!(f, "Resupply XP"),
            ExperienceType::Spot_Kill => write!(f, "Spot Kill XP"),
            ExperienceType::Headshot => write!(f, "Headshot XP"),
            ExperienceType::Stop_Kill_Streak => write!(f, "Ended Kill Streak XP"),
            //ExperienceType::Meta_Game_Event_reward => write!(f, ""),
            ExperienceType::Squad_Heal => write!(f, "Squad Heal XP"),
            ExperienceType::Squad_Revive => write!(f, "Squad Revive XP"),
            ExperienceType::Squad_Spot_Kill => write!(f, "Squad Spot Kill XP"),
            ExperienceType::Squad_Resupply => write!(f, "Squad Resupply XP"),
            ExperienceType::Squad_Spawn => write!(f, "Squad Spawn XP"),
            ExperienceType::Destroy_Engineer_Turret => write!(f, "Destroy Mana Turrent XP"),
            ExperienceType::Vehicle_Destruction_Phalanx => write!(f, "Destroy Phalanx Turret XP"),
            ExperienceType::Vehicle_Destruction_Drop_Pod => write!(f, "Destroy Drop Pod XP"),
            ExperienceType::Vehicle_Destruction_Galaxy => write!(f, "Destroy Galaxy XP"),
            ExperienceType::Vehicle_Destruction_Liberator => write!(f, "Destroy Liberator XP"),
            ExperienceType::Vehicle_Destruction_Lightning => write!(f, "Destroy Lightning XP"),
            ExperienceType::Vehicle_Destruction_Magrider => write!(f, "Destroy Magrider XP"),
            ExperienceType::Vehicle_Destruction_Mosquito => write!(f, "Destroy Mosquito XP"),
            ExperienceType::Vehicle_Destruction_Prowler => write!(f, "Destroy Prowler XP"),
            ExperienceType::Vehicle_Destruction_Reaver => write!(f, "Destroy Reaver XP"),
            ExperienceType::Vehicle_Destruction_Scythe => write!(f, "Destroy Scythe XP"),
            ExperienceType::Vehicle_Destruction_Sunderer => write!(f, "Destroy Sunderer XP"),
            ExperienceType::Vehicle_Destruction_Vanguard => write!(f, "Destroy Vanguard XP"),
            ExperienceType::Vehicle_Ram_Bonus => write!(f, "Vehicle Ram Bonus XP"),
            ExperienceType::Vehicle_Ram_Kill_Engi_Turret => write!(f, "Ram Kill (Mana Turret) XP"),
            ExperienceType::Vehicle_Ram_Kill_Phalanx => write!(f, "Ram Kill (Phalanx Turret) XP"),
            ExperienceType::Vehicle_Ram_Kill_Drop_Pod => write!(f, "Ram Kill (Drop Pod) XP"),
            ExperienceType::Vehicle_Ram_Kill_Galaxy => write!(f, "Ram Kill (Galaxy) XP"),
            ExperienceType::Vehicle_Ram_Kill_Liberator => write!(f, "Ram Kill (Liberator) XP"),
            ExperienceType::Vehicle_Ram_Kill_Lightning => write!(f, "Ram Kill (Lightning) XP"),
            ExperienceType::Vehicle_Ram_Kill_Magrider => write!(f, "Ram Kill (Magrider) XP"),
            ExperienceType::Vehicle_Ram_Kill_Mosquito => write!(f, "Ram Kill (Mosquito) XP"),
            ExperienceType::Vehicle_Ram_Kill_Prowler => write!(f, "Ram Kill (Prowler) XP"),
            ExperienceType::Vehicle_Ram_Kill_Reaver => write!(f, "Ram Kill (Reaver) XP"),
            ExperienceType::Vehicle_Ram_Kill_Scythe => write!(f, "Ram Kill (Scythe) XP"),
            ExperienceType::Vehicle_Ram_Kill_Sunderer => write!(f, "Ram Kill (Sunderer) XP"),
            ExperienceType::Vehicle_Ram_Kill_Vanguard => write!(f, "Ram Kill (Vanguard) XP"),
            ExperienceType::Explosive_Destruction => write!(f, "Explosive Destruction XP"),
            ExperienceType::Secondary_Facility_Object_Repair => {
                write!(f, "2ndary Facility Obj Repair XP")
            }
            ExperienceType::Vehicle_Repair_Engi_Turret => write!(f, "Repair Mana Turret XP"),
            ExperienceType::Vehicle_Repair_Phalanx => write!(f, "Repair Phalanx Turret XP"),
            ExperienceType::Vehicle_Repair_Drop_Pod => write!(f, "Repair Drop Pod XP"),
            ExperienceType::Vehicle_Repair_Galaxy => write!(f, "Repair Galaxy XP"),
            ExperienceType::Vehicle_Repair_Liberator => write!(f, "Repair Liberator XP"),
            ExperienceType::Vehicle_Repair_Lightning => write!(f, "Repair Lightning XP"),
            ExperienceType::Vehicle_Repair_Magrider => write!(f, "Repair Magrider XP"),
            ExperienceType::Vehicle_Repair_Mosquito => write!(f, "Repair Mosquito XP"),
            ExperienceType::Vehicle_Repair_Prowler => write!(f, "Repair Prowler XP"),
            ExperienceType::Vehicle_Repair_Reaver => write!(f, "Repair Reaver XP"),
            ExperienceType::Vehicle_Repair_Scythe => write!(f, "Repair Scythe XP"),
            ExperienceType::Vehicle_Repair_Sunderer => write!(f, "Repair Sunderer XP"),
            ExperienceType::Vehicle_Repair_Vanguard => write!(f, "Repair Vanguard XP"),
            ExperienceType::Kill_Assist_Flash => write!(f, "Kill Assist Flash XP"),
            ExperienceType::Kill_Assist_Engi_Turret => write!(f, "Kill Assist Mana Turret XP"),
            ExperienceType::Kill_Assist_Phalanx => write!(f, "Kill Assist Phalanx XP"),
            ExperienceType::Kill_Assist_Drop_Pod => write!(f, "Kill Assist Drop Pod XP"),
            ExperienceType::Kill_Assist_Galaxy => write!(f, "Kill Assist Galaxy XP"),
            ExperienceType::Kill_Assist_Liberator => write!(f, "Kill Assist Liberator XP"),
            ExperienceType::Kill_Assist_Lightning => write!(f, "Kill Assist Lightning XP"),
            ExperienceType::Kill_Assist_Magrider => write!(f, "Kill Assist Magrider XP"),
            ExperienceType::Kill_Assist_Mosquito => write!(f, "Kill Assist Mosquito XP"),
            ExperienceType::Kill_Assist_Prowler => write!(f, "Kill Assist Prowler XP"),
            ExperienceType::Kill_Assist_Reaver => write!(f, "Kill Assist Reaver XP"),
            ExperienceType::Kill_Assist_Scythe => write!(f, "Kill Assist Scythe XP"),
            ExperienceType::Kill_Assist_Sunderer => write!(f, "Kill Assist Sunderer XP"),
            ExperienceType::Kill_Assist_Vanguard => write!(f, "Kill Assist Vanguard XP"),
            ExperienceType::Squad_Repair_Engi_Turret => write!(f, "Repair Squad Mana Turret XP"),
            ExperienceType::Squad_Repair_Phalanx => write!(f, "Repair Squad Phalanx XP"),
            ExperienceType::Squad_Repair_Drop_Pod => write!(f, "Repair Squad Drop Pod XP"),
            ExperienceType::Squad_Repair_Galaxy => write!(f, "Repair Squad Galaxy XP"),
            ExperienceType::Squad_Repair_Liberator => write!(f, "Repair Squad Liberator XP"),
            ExperienceType::Squad_Repair_Lightning => write!(f, "Repair Squad Lightning XP"),
            ExperienceType::Squad_Repair_Magrider => write!(f, "Repair Squad Magrider XP"),
            ExperienceType::Squad_Repair_Mosquito => write!(f, "Repair Squad Mosquito XP"),
            ExperienceType::Squad_Repair_Prowler => write!(f, "Repair Squad Prowle XP"),
            ExperienceType::Squad_Repair_Reaver => write!(f, "Repair Squad Reaver XP"),
            ExperienceType::Squad_Repair_Scythe => write!(f, "Repair Squad Scythe XP"),
            ExperienceType::Squad_Repair_Sunderer => write!(f, "Repair Squad Sundere XP"),
            ExperienceType::Squad_Repair_Vanguard => write!(f, "Repair Squad Vanguard XP"),
            ExperienceType::Squad_MAX_Repair => write!(f, "Repair Squad MAX XP"),
            ExperienceType::Drop_Pod_Kill => write!(f, "Kill Drop Pod XP"),
            ExperienceType::Player_Kill_by_Sunderer_Gunner => {
                write!(f, "Sundere Gunner Kill (Player) XP")
            }
            ExperienceType::Player_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Player) XP")
            }
            ExperienceType::Player_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Player) XP")
            }
            ExperienceType::Player_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Player) XP")
            }
            ExperienceType::Player_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Player) XP")
            }
            ExperienceType::Player_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Player) XP")
            }
            ExperienceType::Flash_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Flash) XP")
            }
            ExperienceType::Sunderer_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Lightning) XP")
            }
            ExperienceType::Magrider_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Magrider) XP")
            }
            ExperienceType::Vanguard_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Prowler_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Prowler) XP")
            }
            ExperienceType::Scythe_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Scythe) XP")
            }
            ExperienceType::Reaver_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Reaver) XP")
            }
            ExperienceType::Mosquito_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Lib_Kill_by_Sunderer => {
                write!(f, "Sunderer Gunner Kill Liberator(Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Flash_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (FLash) XP")
            }
            ExperienceType::Sunderer_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Lightning) XP")
            }
            ExperienceType::Vanguard_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Prowler_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Prowler) XP")
            }
            ExperienceType::Reaver_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Reaver) XP")
            }
            ExperienceType::Mosquito_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Lib_Kill_by_Magrider => {
                write!(f, "Magrider Gunner Kill (Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Flash_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Flash) XP")
            }
            ExperienceType::Sunderer_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Lightning) XP")
            }
            ExperienceType::Magrider_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Magrider) XP")
            }
            ExperienceType::Prowler_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Prowler) XP")
            }
            ExperienceType::Scythe_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Scythe) XP")
            }
            ExperienceType::Mosquito_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Lib_Kill_by_Vanguard => {
                write!(f, "Vanguard Gunner Kill (Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Flash_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Flash) XP")
            }
            ExperienceType::Galaxy_Spawn_Bonus => write!(f, "Galaxy Spawn Bonus XP"),
            ExperienceType::Sunderer_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Lightning) XP")
            }
            ExperienceType::Magrider_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Magrider) XP")
            }
            ExperienceType::Vanguard_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Scythe_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Scythe) XP")
            }
            ExperienceType::Reaver_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Reaver) XP")
            }
            ExperienceType::Liberator_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Flash_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Flash) XP")
            }
            ExperienceType::Sunderer_Kill_by_Lib_Gunner => {
                write!(f, "Liberator Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Lightning) XP")
            }
            ExperienceType::Magrider_Kill_by_Lib_Gunner => {
                write!(f, "Liberator Gunner Kill (Magrider) XP")
            }
            ExperienceType::Vanguard_Kill_by_Lib_Gunner => {
                write!(f, "Liberator Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Prowler_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Prowler) XP")
            }
            ExperienceType::Scythe_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Scythe) XP")
            }
            ExperienceType::Reaver_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Reaver) XP")
            }
            ExperienceType::Mosquito_Kill_by_Lib_Gunner => {
                write!(f, "Liberator Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Lib_Kill_by_Liberator => {
                write!(f, "Liberator Gunner Kill (Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Flash_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Flash) XP")
            }
            ExperienceType::Sunderer_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Lightning) XP")
            }
            ExperienceType::Magrider_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Magrider) XP")
            }
            ExperienceType::Vanguard_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Prowler_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Prowler) XP")
            }
            ExperienceType::Scythe_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Scythe) XP")
            }
            ExperienceType::Reaver_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Reaver) XP")
            }
            ExperienceType::Mosquito_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Mosquito) XP")
            }
            ExperienceType::LibKill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Sunderer_Spawn_Bonus => write!(f, "Sunderer Spawn XP"),
            ExperienceType::Facility_placed_bomb => write!(f, "Facility Placed Bomb XP"),
            ExperienceType::Facility_defused_bomb => write!(f, "Facility Defused Bomb XP"),
            ExperienceType::Facility_Terminal_Hack => write!(f, "Hack Terminal XP"),
            ExperienceType::Facility_Turret_Hack => write!(f, "Hack Turret XP"),
            ExperienceType::Vehicle_Resupply => write!(f, "Vehicle Resupply XP"),
            ExperienceType::Squad_Vehicle_Resupply => write!(f, "Squad Vehicle Resupply XP"),
            ExperienceType::Spot_Kill_Flash => write!(f, "Spot Kill (Flash) XP"),
            ExperienceType::Spot_Kill_Engi_Turret => write!(f, "Spot Kill (Mana Turret) XP"),
            ExperienceType::Spot_Kill_Phalanx => write!(f, "Spot Kill (Phalanx Turret) XP"),
            ExperienceType::Spot_Kill_Drop_Pod => write!(f, "Spot Kill (Drop Pod) XP"),
            ExperienceType::Spot_Kill_Galaxy => write!(f, "Spot Kill (Galaxy) XP"),
            ExperienceType::Spot_Kill_Liberator => write!(f, "Spot Kill (Liberator) XP"),
            ExperienceType::Spot_Kill_Lightning => write!(f, "Spot Kill (Lightning) XP"),
            ExperienceType::Spot_Kill_Magrider => write!(f, "Spot Kill (Magrider) XP"),
            ExperienceType::Spot_Kill_Mosquito => write!(f, "Spot Kill (Mosquito) XP"),
            ExperienceType::Spot_Kill_Prowler => write!(f, "Spot Kill (Prowler) XP"),
            ExperienceType::Spot_Kill_Reaver => write!(f, "Spot Kill (Reaver) XP"),
            ExperienceType::Spot_Kill_Scythe => write!(f, "Spot Kill (Scythe) XP"),
            ExperienceType::Spot_Kill_Sunderer => write!(f, "Spot Kill (Sunderer) XP"),
            ExperienceType::Spot_Kill_Vanguard => write!(f, "Spot Kill (Vanguard) XP"),
            ExperienceType::Squad_Spot_Kill_Flash => write!(f, "Spot Squad Kill (Flash) XP"),
            ExperienceType::Squad_Spot_Kill_Engi_Turret => {
                write!(f, "Spot Squad Kill (Mana Turret) XP")
            }
            ExperienceType::Squad_Spot_Kill_Phalanx => {
                write!(f, "Spot Squad Kill (Phalanx Turret) XP")
            }
            ExperienceType::Squad_Spot_Kill_Drop_Pod => write!(f, "Spot Squad Kill (Drop Pod) XP"),
            ExperienceType::Squad_Spot_Kill_Galaxy => write!(f, "Spot Squad Kill (Galaxy) XP"),
            ExperienceType::Squad_Spot_Kill_Liberator => {
                write!(f, "Spot Squad Kill (Liberator) XP")
            }
            ExperienceType::Squad_Spot_Kill_Lightning => {
                write!(f, "Spot Squad Kill (Lightning) XP")
            }
            ExperienceType::Squad_Spot_Kill_Magrider => write!(f, "Spot Squad Kill (Magrider) XP"),
            ExperienceType::Squad_Spot_Kill_Mosquito => write!(f, "Spot Squad Kill (Mosquito) XP"),
            ExperienceType::Squad_Spot_Kill_Prowler => write!(f, "Spot Squad Kill (Prowler) XP"),
            ExperienceType::Squad_Spot_Kill_Reaver => write!(f, "Spot Squad Kill (Reaver) XP"),
            ExperienceType::Squad_Spot_Kill_Scythe => write!(f, "Spot Squad Kill (Scythe) XP"),
            ExperienceType::Squad_Spot_Kill_Sunderer => write!(f, "Spot Squad Kill (Sunderer) XP"),
            ExperienceType::Squad_Spot_Kill_Vanguard => write!(f, "Spot Squad Kill (Vanguard) XP"),
            ExperienceType::Squad_Spawn_Beacon_Kill => write!(f, "Squad Spawn Beacon Kill XP"),
            ExperienceType::Convert_Capture_Point => write!(f, "Capture Point Conversion XP"),
            ExperienceType::Terminal_Kill => write!(f, "Destroy Terminal XP"),
            ExperienceType::Terminal_Repair => write!(f, "Repair Terminal XP"),
            ExperienceType::Spawn_Kill => write!(f, "Spawn Kill XP"),
            ExperienceType::Priority_Kill => write!(f, "Priority Kill XP"),
            ExperienceType::High_Priority_Kill => write!(f, "High Priority Kill XP"),
            ExperienceType::Lightning_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Lightning) XP")
            }
            ExperienceType::Prowler_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Prowler) XP")
            }
            ExperienceType::Galaxy_Damage => write!(f, "Vehicle Damage (Galaxy) XP"),
            ExperienceType::Liberator_Damage => write!(f, "Vehicle Damage (Liberator) XP"),
            ExperienceType::Magrider_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Magrider) XP")
            }
            ExperienceType::Mosquito_Damage => write!(f, "Vehicle Damage (Mosquito) XP"),
            ExperienceType::Reaver_Damage => write!(f, "Vehicle Damage (Reaver) XP"),
            ExperienceType::Scythe_Damage => write!(f, "Vehicle Damage (Scythe) XP"),
            ExperienceType::Sunderer_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Sunderer) XP")
            }
            ExperienceType::Vanguard_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Vanguard) XP")
            }
            ExperienceType::Ribbon_Experience => write!(f, "Ribbon XP"),
            ExperienceType::Motion_Detect => write!(f, "Motion Detection XP"),
            ExperienceType::Squad_Motion_Spot => write!(f, "Squad Motion Spot XP"),
            ExperienceType::Vehicle_Ram_Kill_Harasser => write!(f, "Ram Kill (Harasser) XP"),
            ExperienceType::Vehicle_Destruction_Harasser => write!(f, "Destroy Harasser XP"),
            ExperienceType::Squad_Repair_Harasser => write!(f, "Repair Squad Harasser XP"),
            ExperienceType::Vehicle_Repair_Harasser => write!(f, "Repair Harasser XP"),
            ExperienceType::Kill_Assist_Harasser => write!(f, "Kill Assist Harasser XP"),
            ExperienceType::Spot_Kill_Harasser => write!(f, "Spot Kill (Harasser) XP"),
            ExperienceType::Squad_Spot_Kill_Harasser => write!(f, "Spot Squad Kill (Harasser) XP"),
            ExperienceType::Harasser_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Harasser) XP")
            }
            ExperienceType::Harasser_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Harasser) XP")
            }
            ExperienceType::Harasser_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Harasser) XP")
            }
            ExperienceType::Harasser_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Harasser) XP")
            }
            ExperienceType::Harasser_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Harasser) XP")
            }
            ExperienceType::Harasser_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Harasser) XP")
            }
            ExperienceType::Player_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Player) XP")
            }
            ExperienceType::Flash_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Flash) XP")
            }
            ExperienceType::Sunderer_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Lightning) XP")
            }
            ExperienceType::Vanguard_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Prowler_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Prowler) XP")
            }
            ExperienceType::Reaver_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Reaver) XP")
            }
            ExperienceType::Mosquito_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Lib_Kill_by_Harasser => {
                write!(f, "Harasser Gunner Kill (Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Harasser_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Harasser) XP")
            }
            ExperienceType::Magrider_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Magrider) XP")
            }
            ExperienceType::Scythe_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Scythe) XP")
            }
            ExperienceType::Tank_Mine_Despawn_or_Defusal => write!(f, "Defuse Tank Mine XP"),
            ExperienceType::Alert_Reward => write!(f, "Alert Reward XP"),
            ExperienceType::Tank_Hunter_Bonus_Prowler_Kill => {
                write!(f, "Tank Hunter Bonus Prowler Kill XP")
            }
            ExperienceType::Tank_Hunter_Bonus_Magrider_Kil => {
                write!(f, "Tank Hunter Bonus Magrider Kill XP")
            }
            ExperienceType::Dogfighter_Bonus_Mosquito_Kill => {
                write!(f, "Dogfighter Bonus Mosquito Kill XP")
            }
            ExperienceType::Dogfighter_Bonus_Reaver_Kill => {
                write!(f, "Dogfighter Bonus ReaverKill XP")
            }
            ExperienceType::Dogfighter_Bonus_Scythe_Kill => {
                write!(f, "Dogfighter Bonus Scythe Kill XP")
            }
            ExperienceType::Tank_Hunter_Bonus_Vanguard_Kil => {
                write!(f, "Tank Hunter Bonus Vanguard Kill XP")
            }
            ExperienceType::Savior_Kill_Non_MAX => write!(f, "Savior Kill XP"),
            ExperienceType::Saved => write!(f, "Saved XP"),
            ExperienceType::Holiday_Event_NPC_Kill => write!(f, "Holiday NPC Kill XP"),
            ExperienceType::Holiday_Event_NPC_Gold_Kill => write!(f, "Holiday G-NPC Kill XP"),
            ExperienceType::Snowman_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Snowman) XP")
            }
            ExperienceType::Snowman_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Snowman) XP")
            }
            ExperienceType::Snowman_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Snowman) XP")
            }
            ExperienceType::Snowman_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Snowman) XP")
            }
            ExperienceType::Snowman_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Snowman) XP")
            }
            ExperienceType::Snowman_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Snowman) XP")
            }
            ExperienceType::Snowman_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Snowman) XP")
            }
            ExperienceType::GSnowman_Kill_by_Sunder_Gunner => {
                write!(f, "Sunderer Gunner Kill (Great Snowman) XP")
            }
            ExperienceType::GSnowman_Kill_by_Mag_Gunner => {
                write!(f, "Magrider Gunner Kill (Great Snowman) XP")
            }
            ExperienceType::GSnowman_Kill_by_Vang_Gunner => {
                write!(f, "Vanguard Gunner Kill (Great Snowman) XP")
            }
            ExperienceType::GSnowman_Kill_by_Prow_Gunner => {
                write!(f, "Prowler Gunner Kill (Great Snowman) XP")
            }
            ExperienceType::GSnowman_Kill_by_Lib_Gunner => {
                write!(f, "Liberator Gunner Kill (Great Snowman) XP")
            }
            ExperienceType::GSnowman_Kill_by_Gal_Gunner => {
                write!(f, "Galaxy Gunner Kill (Great Snowman) XP")
            }
            ExperienceType::GSnowman_Kill_by_Haras_Gunner => {
                write!(f, "Harasser Gunner Kill (Great Snowman) XP")
            }
            ExperienceType::Scout_Radar_Detect => write!(f, "Scout Rader Detect XP"),
            ExperienceType::Squad_Scout_Radar_Detect => write!(f, "Squad Scout Radar Detext XP"),
            ExperienceType::Squad_Vehicle_Spawn_Bonus => write!(f, "Squad Vehicle Spawn Bonus XP"),

            //Recon drones are no longer in the game.
            /*ExperienceType::Vehicle_Ram_Kill_R_Drone => write!(f, ""),
            ExperienceType::Vehicle_Destruction_R_Drone => write!(f, ""),
            ExperienceType::Squad_Repair_R_Drone => write!(f, ""),
            ExperienceType::Vehicle_Repair_R_Drone => write!(f, ""),
            ExperienceType::Kill_Assist_R_Drone => write!(f, ""),
            ExperienceType::R_Drone_Kill_by_Harasser_Gunner => write!(f, ""),
            ExperienceType::R_Drone_Kill_by_Sunderer_Gunner => write!(f, ""),
            ExperienceType::R_Drone_Kill_by_Magrider_Gunner => write!(f, ""),
            ExperienceType::R_Drone_Kill_by_Prowler_Gunner => write!(f, ""),
            ExperienceType::R_Drone_Kill_by_Lib_Gunner => write!(f, ""),
            ExperienceType::R_Drone_Kill_by_Galaxy_Gunner => write!(f, ""),
            ExperienceType::Spot_Kill_R_Drone => write!(f, ""),
            ExperienceType::Squad_Spot_Kill_R_Drone => write!(f, ""),*/
            ExperienceType::Motion_Sensor_Spotter_Kill => write!(f, "Destroy Motion Spotter XP"),
            ExperienceType::Kill_Player_Priority_Assist => write!(f, "Kill Assist on Priority XP"),
            ExperienceType::Kill_Player_High_Priority_Assist => {
                write!(f, "Kill Assist High Priority XP")
            }
            ExperienceType::Shield_Regen_Tool_Kill => write!(f, "Destroy Shield Regen Tool XP"),
            ExperienceType::Shield_Repair => write!(f, "Shield Repair XP"),
            ExperienceType::Squad_Shield_Repair => write!(f, "Squad Shield Repair XP"),
            ExperienceType::Chain_Expl_Assist_Infantry => {
                write!(f, "Chain Explosion Assist (Infantry) XP")
            }
            ExperienceType::Chain_Expl_Assist_Flash => {
                write!(f, "Chain Explosion Assist (Flash) XP")
            }
            ExperienceType::Vehicle_Destruction_Valkyrie => write!(f, "Destroy Valkyrie XP"),
            ExperienceType::Vehicle_Ram_Kill_Valkyrie => write!(f, "Ram Kill (Valkyrie) XP"),
            ExperienceType::Vehicle_Repair_Valkyrie => write!(f, "Repair Valkyrie XP"),
            ExperienceType::Kill_Assist_Valkyrie => write!(f, "Kill Assist Valkyrie XP"),
            ExperienceType::Squad_Repair_Valkyrie => write!(f, "Repair Squad Valkyrie XP"),
            ExperienceType::Spot_Kill_Valkyrie => write!(f, "Spot Kill (Valkyrie) XP"),
            ExperienceType::Squad_Spot_Kill_Valkyrie => write!(f, "Spot Squad Kill (Valkyrie) XP"),
            ExperienceType::Valkyrie_Damage => write!(f, "Vehicle Damage (Valkyrie) XP"),
            ExperienceType::Valkyrie_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Player_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Player) XP")
            }
            ExperienceType::Flash_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Flash) XP")
            }
            ExperienceType::Sunderer_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Lightning_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Lightning) XP")
            }
            ExperienceType::Vanguard_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Prowler_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Prowler) XP")
            }
            ExperienceType::Reaver_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Reaver) XP")
            }
            ExperienceType::Mosquito_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Lib_Kill_by_Valkyrie => {
                write!(f, "Valkyrie Gunner Kill (Liberator) XP")
            }
            ExperienceType::Galaxy_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Magrider_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Magrider) XP")
            }
            ExperienceType::Scythe_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Scythe) XP")
            }
            ExperienceType::Snowman_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (SNowman) XP")
            }
            //ExperienceType::R_Drone_Kill_by_Valkyrie_Gunner => write!(f, ""),
            ExperienceType::Valkyrie_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Chain_Expl_Assist_Phalanx => {
                write!(f, "Chain Explosion Assist (Phalanx Turret) XP")
            }
            ExperienceType::Chain_Expl_Assist_Drop_Pod => {
                write!(f, "Chain Explosion Assist (Drop Pod) XP")
            }
            ExperienceType::Chain_Expl_Assist_Galaxy => {
                write!(f, "Chain Explosion Assist (Galaxy) XP")
            }
            ExperienceType::Chain_Expl_Assist_Liberator => {
                write!(f, "Chain Explosion Assist (Liberator) XP")
            }
            ExperienceType::Chain_Expl_Assist_Lightning => {
                write!(f, "Chain Explosion Assist (Lightning) XP")
            }
            ExperienceType::Chain_Expl_Assist_Magrider => {
                write!(f, "Chain Explosion Assist (Magrider) XP")
            }
            ExperienceType::Chain_Expl_Assist_Mosquito => {
                write!(f, "Chain Explosion Assist (Mosquito) XP")
            }
            ExperienceType::Chain_Expl_Assist_Prowler => {
                write!(f, "Chain Explosion Assist (Prowler) XP")
            }
            ExperienceType::Chain_Expl_Assist_Reaver => {
                write!(f, "Chain Explosion Assist (Reaver) XP")
            }
            ExperienceType::Chain_Expl_Assist_Scythe => {
                write!(f, "Chain Explosion Assist (Scythe) XP")
            }
            ExperienceType::Chain_Expl_Assist_Sunderer => {
                write!(f, "Chain Explosion Assist (Sunderer) XP")
            }
            ExperienceType::Chain_Expl_Assist_Vanguard => {
                write!(f, "Chain Explosion Assist (Vanguard) XP")
            }
            ExperienceType::Chain_Expl_Assist_Harasser => {
                write!(f, "Chain Explosion Assist (Harasser) XP")
            }
            //ExperienceType::Chain_Expl_Assist_R_Drone => write!(f, ""),
            ExperienceType::Chain_Expl_Assist_Valkyrie => {
                write!(f, "Chain Explosion Assist (Valkyrie) XP")
            }
            ExperienceType::Concussion_Grenade_Assist => write!(f, "Concussion Assist XP"),
            ExperienceType::Concussion_Grenade_Squad_Assist => {
                write!(f, "Squad Concussion Assist XP")
            }
            ExperienceType::EMP_Grenade_Assist => write!(f, "EMP Assist XP"),
            ExperienceType::EMP_Grenade_Squad_Assist => write!(f, "Squad EMP Assist XP"),
            ExperienceType::Flashbang_Assist => write!(f, "Flashbang Assist XP"),
            ExperienceType::Flashbang_Squad_Assist => write!(f, "Squad Flashbang Assist XP"),
            ExperienceType::Objective_Pulse_Defend => write!(f, "Point Defend XP"),
            ExperienceType::Objective_Pulse_Capture => write!(f, "Point Capture XP"),
            ExperienceType::Halloween_Event_NPC_GreatP_Kill => write!(f, "Great Pumpkin Kill XP"),
            ExperienceType::Pumpkin_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Pumpkin) XP")
            }
            ExperienceType::GPumpkin_Kill_by_Sunder_Gunner => {
                write!(f, "Sunderer Gunner Kill (Great Pumpkin) XP")
            }
            ExperienceType::GPumpkin_Kill_by_Mag_Gunner => {
                write!(f, "Magrider Gunner Kill (Great Pumpkin) XP")
            }
            ExperienceType::GPumpkin_Kill_by_Vang_Gunner => {
                write!(f, "Vanguard Gunner Kill (Great Pumpkin) XP")
            }
            ExperienceType::GPumpkin_Kill_by_Prow_Gunner => {
                write!(f, "Prowler Gunner Kill (Great Pumpkin) XP")
            }
            ExperienceType::GPumpkin_Kill_by_Lib_Gunner => {
                write!(f, "Liberator Gunner Kill (Great Pumpkin) XP")
            }
            ExperienceType::GPumpkin_Kill_by_Gal_Gunner => {
                write!(f, "Galaxy Gunner Kill (Great Pumpkin) XP")
            }
            ExperienceType::GPumpkin_Kill_by_Haras_Gunner => {
                write!(f, "Harasser Gunner Kill (Great Pumpkin) XP")
            }
            ExperienceType::Halloween_Event_NPC_Kill => write!(f, "Halloween NPC Kill XP"),
            ExperienceType::Harasser_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyie Gunner Kill (Harasser) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Snowman_kill_by_squad_member => {
                write!(f, "Snowman Kill By Squad Member XP")
            }
            ExperienceType::Gsnowman_kill_by_squad_member => {
                write!(f, "Great Snowman Kill By Squad Member XP")
            }
            ExperienceType::Destroy_Spitfire_Turret => write!(f, "Destroy Spitty XP"),
            ExperienceType::Vehicle_Ram_Kill_Spitfire_Turret => write!(f, "Ram Kill (Spitty) XP"),
            ExperienceType::Vehicle_Repair_Spitfire_Turret => write!(f, "Repair Spitty XP"),
            ExperienceType::Kill_Assist_Spitfire_Turret => write!(f, "Kill Assist Spitty XP"),
            ExperienceType::Squad_Repair_Spitfire_Turret => write!(f, "Repair Squad Spitty XP"),
            ExperienceType::Spot_Kill_Spitfire_Turret => write!(f, "Spot Kill (Spitty) XP"),
            ExperienceType::Squad_Spot_Kill_Spitfire_Turret => {
                write!(f, "Spot Squad Kill (Spitty) XP")
            }
            ExperienceType::Gunner_Kill_Share_Spitfire_Turret => {
                write!(f, "Gunner Kill Share (Spitty) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Spitfire_Turret => {
                //588
                write!(f, "Gunner Kill Assist (Spitty) Share XP")
            }
            //ExperienceType::Kill_Dummy_NPC => write!(f, ""),
            ExperienceType::Savior_Kill_MAX => write!(f, "Savior Kill MAX XP"),
            ExperienceType::Bounty_Kill_Bonus => write!(f, "Bounty Kill Bonus XP"),
            ExperienceType::Bounty_Kill_Cashed_In => write!(f, "Bount Claimed XP"),
            ExperienceType::Bounty_Kill_Streak => write!(f, "Bounty Kill Streak XP"),
            ExperienceType::Membership_bonus_xp => write!(f, "Membership Bonus XP"),
            ExperienceType::Victory_Point_XP => write!(f, "Victory Point XP"),
            ExperienceType::Continent_Capture_XP => write!(f, "Continent Capture XP"),
            ExperienceType::Victory_Point_XP25_percent_XP => write!(f, "Victory Point 25% XP"),
            ExperienceType::Victory_Point_XP50_percent_XP => write!(f, "Victory Point 50% XP"),
            ExperienceType::Victory_Point_XP75_percent_XP => write!(f, "Victory Point 75% XP"),
            ExperienceType::Victory_Point_XP_plus_50_percent_XP => {
                write!(f, "Victory Point +50% XP")
            }
            ExperienceType::Vehicle_Ram_Kill_Construction_Med => {
                write!(f, "Ram Kill (Construction [Med]) XP")
            }
            ExperienceType::Kill_or_Destroy_Construction_Med => {
                write!(f, "Destroy Construction (Med) XP")
            }
            ExperienceType::Squad_Repair_Construction_Med => {
                write!(f, "Repair Squad Construcion (Med) XP")
            }
            ExperienceType::Repair_Construction_Med => write!(f, "Repair Construcion (Med) XP"),
            ExperienceType::KillAssist_Construction_Med => {
                write!(f, "Kill Assist Construction (Med) XP")
            }
            ExperienceType::Spot_Kill_Construction_Med => {
                write!(f, "Spot Kill (Construction [Med]) XP")
            }
            ExperienceType::Squad_Spot_Kill_Construction_Med => {
                write!(f, "Spot Squad Kill (Construction [Med]) XP")
            }
            ExperienceType::Vehicle_Ram_Kill_Construction_Small => {
                write!(f, "Ram Kill (Constrruction [Small]) XP")
            }
            ExperienceType::Kill_or_Destroy_Construction_Small => {
                write!(f, "Destroy Construction (Small) XP")
            }
            ExperienceType::Squad_Repair_Construction_Small => {
                write!(f, "Repair Squad Construcion (Small) XP")
            }
            ExperienceType::Repair_Construction_Small => write!(f, "Repair Construcion (Small) XP"),
            ExperienceType::KillAssist_Construction_Small => {
                write!(f, "Kill Assist Construction (Small) XP")
            }
            ExperienceType::Spot_Kill_Construction_Small => {
                write!(f, "Spot Kill (Construction [Small]) XP")
            }
            ExperienceType::Squad_Spot_Kill_Construction_Small => {
                write!(f, "Spot Kill (Construction [Small]) XP")
            }
            ExperienceType::Vehicle_Ram_Kill_Construction_Large => {
                write!(f, "Ram Kill (Construction [Large]) XP")
            }
            ExperienceType::Kill_or_Destroy_Construction_Large => {
                write!(f, "Destroy Construction (Large) XP")
            }
            ExperienceType::Squad_Repair_Construction_Large => {
                write!(f, "Repair Squad Construcion (Large) XP")
            }
            ExperienceType::Repair_Construction_Large => write!(f, "Repair Construcion (Large) XP"),
            ExperienceType::KillAssist_Construction_Large => {
                write!(f, "Kill Assist Construction (Large) XP")
            }
            ExperienceType::Spot_Kill_Construction_Large => {
                write!(f, "Spot Kill (Construction [Large]) XP")
            }
            ExperienceType::Squad_Spot_Kill_Construction_Large => {
                write!(f, "Spot Squad Kill (Construction [Large]) XP")
            }
            ExperienceType::Vehicle_Ram_Kill_Construction_Core => {
                write!(f, "Ram Kill (Construction [Core]) XP")
            }
            ExperienceType::Kill_or_Destroy_Construction_Core => {
                write!(f, "Destroy Construction (Core) XP")
            }
            ExperienceType::Squad_Repair_Construction_Core => {
                write!(f, "Repair Squad Construcion (Core) XP")
            }
            ExperienceType::Repair_Construction_Core => write!(f, "Repair Construcion (Core) XP"),
            ExperienceType::KillAssist_Construction_Core => {
                write!(f, "Kill Assist Construction (Core) XP")
            }
            ExperienceType::Spot_Kill_Construction_Core => {
                write!(f, "Spot Kill (Construction [Core]) XP")
            }
            ExperienceType::Squad_Spot_Kill_Construction_Core => {
                write!(f, "Spot Squad Kill (Construction [Core]) XP")
            }
            ExperienceType::Vehicle_Destruction_ANT => write!(f, "Destroy ANT XP"),
            ExperienceType::Vehicle_Ram_Kill_ANT => write!(f, "Ram Kill (ANT) XP"),
            ExperienceType::Vehicle_Repair_ANT => write!(f, "Repair ANT XP"),
            ExperienceType::Kill_Assist_ANT => write!(f, "Kill Assist Ant XP"),
            ExperienceType::Squad_Repair_ANT => write!(f, "Repair Squad ANT XP"),
            ExperienceType::ANT_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (ANT) XP"),
            ExperienceType::ANT_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (ANT) XP")
            }
            ExperienceType::ANT_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (ANT) XP")
            }
            ExperienceType::ANT_Kill_by_Prowler_Gunner => write!(f, "Prowler Gunner Kill (ANT) XP"),
            ExperienceType::ANT_Kill_by_Lib_Gunner => write!(f, "Liberator Gunner Kill (ANT) XP"),
            ExperienceType::ANT_Kill_by_Galaxy_Gunner => write!(f, "Galaxy Gunner Kill (ANT) XP"),
            ExperienceType::Spot_Kill_ANT => write!(f, "Spot Kill (ANT) XP"),
            ExperienceType::Squad_Spot_Kill_ANT => write!(f, "Spot Squad Kill (ANT) XP"),
            ExperienceType::ANT_Damage_Infantry_vs_Vehicle => write!(f, "Vehicle Damage (ANT) XP"),
            ExperienceType::ANT_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (ANT) XP")
            }
            ExperienceType::ANT_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (ANT) XP")
            }
            ExperienceType::Chain_Expl_Assist_ANT => write!(f, "Chain Explosion Assist (ANT) XP"),
            ExperienceType::Bounty_Kill_Cashed_In_Alt => write!(f, "Alternate Bounty Claimed XP"),
            ExperienceType::Cortium_Harvest => write!(f, "Harvest Cortium XP"),
            ExperienceType::Cortium_Deposit => write!(f, "Deposit Cortium XP"),
            ExperienceType::Flash_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Flash) XP"),
            ExperienceType::Galaxy_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Galaxy) XP"),
            ExperienceType::Harasser_Kill_by_ANT_Gunner => {
                write!(f, "ANT Gunner Kill (Harasser) XP")
            }
            ExperienceType::Magrider_Kill_by_ANT_Gunner => {
                write!(f, "ANT Gunner Kill (Magrider) XP")
            }
            ExperienceType::Mosquito_Kill_by_ANT_Gunner => {
                write!(f, "ANT Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Player_Kill_by_ANT_Gunner => write!(f, "Ant Gunner Kill (Player) XP"),
            ExperienceType::Prowler_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Prowler) XP"),
            ExperienceType::Pumpkin_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Pumpkin) XP"),
            //ExperienceType::R_Drone_Kill_by_ANT_Gunner => write!(f, ""),
            ExperienceType::Reaver_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Reaver) XP"),
            ExperienceType::Scythe_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Scythe) XP"),
            ExperienceType::Snowman_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Snowman) XP"),
            ExperienceType::Sunderer_Kill_by_ANT_Gunner => {
                write!(f, "ANT Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Valkyrie_Kill_by_ANT_Gunner => {
                write!(f, "ANT Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Vanguard_Kill_by_ANT_Gunner => {
                write!(f, "ANT Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Destroy_Hardlight_Barrier => write!(f, "Destroy Hardlight Barrier XP"),
            ExperienceType::Vehicle_Ram_Kill_Hardlight_Barrier => {
                write!(f, "Ram Kill (Hardlight Barrier) XP")
            }
            ExperienceType::Vehicle_Repair_Hardlight_Barrier => {
                write!(f, "Repair Hardlight Barrier XP")
            }
            ExperienceType::Kill_Assist_Hardlight_Barrier => {
                write!(f, "Kill Assist Hardlight Barrier XP")
            }
            ExperienceType::Squad_Repair_Hardlight_Barrier => {
                write!(f, "Repair Squad Hardlight Barrier XP")
            }
            ExperienceType::Gunner_Kill_Share_Hardlight_Barrier => {
                write!(f, "Gunner Kill Share (Hardlight Barrier) XP") //1379
            }
            ExperienceType::Missing_1380 => write!(f, "Missing (1380) XP"),
            //ExperienceType::Hardlight_Cover_Blocking_Exp_placeholder_until_code_is_done => write!(f, ""),
            ExperienceType::Draw_Fire_Assist => write!(f, "Draw Fire Assist XP"),
            ExperienceType::Draw_Fire_Award => write!(f, "Draw Fire XP"),
            ExperienceType::Flash_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Flash) XP")
            }
            ExperienceType::Harasser_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Harasser) XP")
            }
            ExperienceType::Router_Kill => write!(f, "Destroy Router XP"),
            ExperienceType::Generic_Npc_Spawn => write!(f, "Generic Spawn XP"), //1410
            //ExperienceType::Event_Anomaly_Defense_Pulse => write!(f, ""),
            //ExperienceType::EQ20_Collectible => write!(f, ""),
            //ExperienceType::EQ20_Nightshade => write!(f, ""),
            ExperienceType::Vehicle_Destruction_Colossus => write!(f, "Destroy Colossus XP"),
            ExperienceType::Vehicle_Ram_Kill_Colossus => write!(f, "Ram Kill (Colossus) XP"),
            ExperienceType::Vehicle_Repair_Colossus => write!(f, "Repair Colossus XP"),
            ExperienceType::Squad_Repair_Colossus => write!(f, "Repair Squad Colossus XP"),
            ExperienceType::Kill_Assist_Colossus => write!(f, "Kill Assist Colossus XP"),
            ExperienceType::Tank_Hunter_Bonus_Colossus_Kill => {
                write!(f, "Tank Hunter Bonus Colossus Kill XP")
            }
            ExperienceType::Chain_Expl_Assist_Colossus => {
                write!(f, "Chain Explosion Assist (Colossus) XP")
            }
            ExperienceType::Squad_Spot_Kill_Colossus => write!(f, "Spot Squad Kill (Colossus) XP"),
            ExperienceType::Spot_Kill_Colossus => write!(f, "Spot Kill (Colossus) XP"),
            ExperienceType::Colossus_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Colossus) XP")
            }
            ExperienceType::Flash_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Flash) XP")
            }
            ExperienceType::Vanguard_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Vanguard) XP")
            }
            ExperienceType::ANT_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (ANT) XP")
            }
            ExperienceType::Galaxy_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Harasser_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Harasser) XP")
            }
            ExperienceType::Magrider_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Magrider) XP")
            }
            ExperienceType::Mosquito_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Player_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Player) XP")
            }
            ExperienceType::Prowler_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Prowler) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Pumpkin) XP")
            }
            //ExperienceType::R_Drone_Kill_by_Colossus_Gunner => write!(f, ""),
            ExperienceType::Reaver_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Reaver) XP")
            }
            ExperienceType::Scythe_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Scythe) XP")
            }
            ExperienceType::Snowman_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Snowman) XP")
            }
            ExperienceType::Sunderer_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Colossus_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Colossus) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Colossus => {
                //1477
                write!(f, "Gunner Kill Assist (Colossus) Share XP")
            }
            ExperienceType::Vehicle_Destruction_Javelin => write!(f, "Destroy Javelin XP"),
            ExperienceType::Squad_Repair_Javelin => write!(f, "Repair Squad Javelin XP"),
            ExperienceType::Vehicle_Repair_Javelin => write!(f, "Repair Javelin XP"),
            ExperienceType::Vehicle_Ram_Bonus_Javelin => write!(f, "Ram Kill (Javelin) XP"), //XP name typo?
            ExperienceType::Kill_Assist_Javelin => write!(f, "Kill Assist Javelin XP"),
            ExperienceType::Javelin_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Javelin) XP")
            }
            ExperienceType::Javelin_Kill_by_Magrider_Gunner => {
                write!(f, "Magrider Gunner Kill (Javelin) XP")
            }
            ExperienceType::Javelin_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Javelin) XP")
            }
            ExperienceType::Javelin_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Javelin) XP")
            }
            ExperienceType::Javelin_Kill_by_Liberator_Gunner => {
                write!(f, "Liberator Gunner Kill (Javelin) XP")
            }
            ExperienceType::Javelin_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Javelin) XP")
            }
            ExperienceType::Spot_Kill_Javelin => write!(f, "Spot Kill (Javelin) XP"),
            ExperienceType::Squad_Spot_Kill_Javelin => write!(f, "Spot Squad Kill (Javelin) XP"),
            ExperienceType::Javelin_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Javelin) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Javelin => {
                //1496
                write!(f, "Gunner Kill Assist (Javelin) Share XP")
            }
            ExperienceType::Chain_Expl_Assist_Javelin => {
                write!(f, "Chain Explosion Assist (Javelin) XP")
            }
            ExperienceType::Javelin_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Javelin) XP")
            }
            ExperienceType::Javelin_Kill_by_ANT_Gunner => write!(f, "ANT Gunner Kill (Javelin) XP"),
            ExperienceType::Javelin_Damage_Infantry_vs_Vehicle => {
                write!(f, "Vehicle Damage (Javelin) XP")
            }
            ExperienceType::Player_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Player) XP")
            }
            ExperienceType::ANT_Kill_by_Bastion_Gunner => write!(f, "Bastion Gunner Kill (ANT) XP"),
            ExperienceType::Colossus_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Colossus) XP")
            }
            ExperienceType::Flash_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Flash) XP")
            }
            ExperienceType::Galaxy_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Galaxy) XP")
            }
            ExperienceType::Harasser_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Harasser) XP")
            }
            ExperienceType::Magrider_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Magrider) XP")
            }
            ExperienceType::Mosquito_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Mosquito) XP")
            }
            ExperienceType::Prowler_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Prowler) XP")
            }
            ExperienceType::Pumpkin_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Pumpkin) XP")
            }
            //ExperienceType::R_Drone_Kill_by_Bastion_Gunner => write!(f, ""),
            ExperienceType::Reaver_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Reaver) XP")
            }
            ExperienceType::Scythe_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Scythe) XP")
            }
            ExperienceType::Snowman_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Snowman) XP")
            }
            ExperienceType::Sunderer_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Sunderer) XP")
            }
            ExperienceType::Valkyrie_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Valkyrie) XP")
            }
            ExperienceType::Vanguard_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Vanguard) XP")
            }
            ExperienceType::Vehicle_Component_Destruction_Bastion => {
                write!(f, "Destroy Bastion Component Xp")
            }
            ExperienceType::Kill_Assist_Bastion_Component => {
                write!(f, "Kill Assist Bastion Component XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Bastion_Component => {
                //1523
                write!(f, "Gunner Kill Assist (Bastion Component) Share XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Sunderer_Gunner => {
                write!(f, "Sunderer Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Vanguard_Gunner => {
                write!(f, "Vanguard Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Prowler_Gunner => {
                write!(f, "Prowler Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Lib_Gunner => {
                write!(f, "Liberator Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Galaxy_Gunner => {
                write!(f, "Galaxy Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Harasser_Gunner => {
                write!(f, "Harasser Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Valkyrie_Gunner => {
                write!(f, "Valkyrie Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_ANT_Gunner => {
                write!(f, "ANT Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Colossus_Gunner => {
                write!(f, "Colossus Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Kill_by_Bastion_Gunner => {
                write!(f, "Bastion Gunner Kill (Bastion Component) XP")
            }
            ExperienceType::Bastion_Component_Damage => write!(f, "Vehicle Damage (Bastion) XP"),
            ExperienceType::Destroy_Lightning_Arrester => {
                write!(f, "Destroy Lightning Arrestor XP")
            }
            //ExperienceType::Lightning_Arrester_Absorb_Xp => write!(f, ""),
            //ExperienceType::Convoy_Periodic_Update => write!(f, ""),
            //ExperienceType::Convoy_Convoy_Complete => write!(f, ""),
            //ExperienceType::Sunderer_Campaign_Spawn_Bonus => write!(f, ""),
            //ExperienceType::Galaxy_Campaign_Spawn_Bonus => write!(f, ""),
            //ExperienceType::tutorial2_grantBr1 => write!(f, ""),
            ExperienceType::Door_Lock_Repair => write!(f, "Repair Shield Door Lock XP"),
            ExperienceType::Destroy_Shield_Door_Lock => write!(f, "Destroy Shield Door Lock XP"),
            ExperienceType::Containment_Site_Gate_Shield_Gen_Destroy => {
                write!(f, "Destroy Gate Shield Generator XP")
            }
            ExperienceType::Containment_Site_Gate_Shield_Gen_Destroy_Assist => {
                write!(f, "Assist Destroy Gate Shield Generator XP")
            }
            ExperienceType::Containment_Site_Gate_Shield_Gen_Repair => {
                write!(f, "Repair Gate Shield Generator XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Lib => {
                write!(f, "Gunner Kill Assist (Liberator) Share XP")
            }
            ExperienceType::Unknown => write!(f, "??? XP"),
            ExperienceType::Gunner_Kill_Share_Player => {
                write!(f, "Gunner Kill Share (Infantry) XP")
            }
            ExperienceType::Gunner_Kill_Share_Flash => write!(f, "Gunner Kill Share (Flash) XP"), //374
            ExperienceType::Gunner_Kill_Share_Phalanx => {
                write!(f, "Gunner Kill Share (Phalanx Turret) XP")
            } //376
            ExperienceType::Gunner_Kill_Share_Galaxy => write!(f, "Gunner Kill Share (Galaxy) XP"),
            ExperienceType::Gunner_Kill_Share_Mana_Turret => {
                write!(f, "Gunner Kill Share (Mana Turret) XP")
            }
            ExperienceType::Gunner_Kill_Share_Liberator => {
                write!(f, "Gunner Kill Share (Liberator) XP")
            }
            ExperienceType::Gunner_Kill_Share_Lightning => {
                write!(f, "Gunner Kill Share (Lightning) XP")
            }
            ExperienceType::Gunner_Kill_Share_Magrider => {
                //381
                write!(f, "Gunner Kill Share (Magrider) XP")
            }
            ExperienceType::Gunner_Kill_Share_Mosquito => {
                //382
                write!(f, "Gunner Kill Share (Mosquito) XP")
            }
            ExperienceType::Gunner_Kill_Share_Prowler => {
                //383
                write!(f, "Gunner Kill Share (Prowler) XP")
            }
            ExperienceType::Gunner_Kill_Share_Reaver => write!(f, "Gunner Kill Share (Reaver) XP"),
            ExperienceType::Gunner_Kill_Share_Scythe => write!(f, "Gunner Kill Share (Scythe) XP"),
            ExperienceType::Gunner_Kill_Share_Sunderer => {
                write!(f, "Gunner Kill Share (Sunderer) XP")
            }
            ExperienceType::Gunner_Kill_Share_Vanguard => {
                write!(f, "Gunner Kill Share (Vanguard) XP")
            }
            ExperienceType::Gunner_Kill_Share_Harasser => {
                write!(f, "Gunner Kill Share (Harasser) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Player => {
                write!(f, "Gunner Kill Assist (Infantry) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Flash => {
                //390
                write!(f, "Gunner Kill Assist (Mana Turret) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Mana_Turret => {
                //391
                write!(f, "Gunner Kill Assist (Flash) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Phalanx => {
                // 392
                write!(f, "Gunner Kill Assist (Phalanx Turret) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Drop_Pod => {
                // 393
                write!(f, "Gunner Kill Assist (Drop Pod) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Galaxy => {
                write!(f, "Gunner Kill Assist (Galaxy) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Lightning => {
                write!(f, "Gunner Kill Assist (Lightning) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Magrider => {
                //397
                write!(f, "Gunner Kill Assist (Magrider) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Prowler => {
                write!(f, "Gunner Kill Assist (Prowler) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Mosquito => {
                write!(f, "Gunner Kill Assist (Mosquito) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Reaver => {
                write!(f, "Gunner Kill Assist (Reaver) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Scythe => {
                write!(f, "Gunner Kill Assist (Scythe) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Sunderer => {
                write!(f, "Gunner Kill Assist (Sunderer) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Vanguard => {
                write!(f, "Gunner Kill Assist (Vanguard) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Harasser => {
                write!(f, "Gunner Kill Assist (Harasser) Share XP")
            }
            ExperienceType::Passenger_Kill_Share_Player => {
                write!(f, "Passenger Kill Share (Infantry) XP")
            }
            ExperienceType::Passenger_Kill_Share_MANA => {
                //407
                write!(f, "Passenger Kill Share (Mana Turret) XP")
            }
            ExperienceType::Passenger_Kill_Share_Galaxy => {
                //410
                write!(f, "Passenger Kill Share (Galaxy) XP")
            }
            ExperienceType::Passenger_Kill_Share_Reaver => {
                //416
                write!(f, "Passenger Kill Share (Reaver XP)")
            }
            ExperienceType::Passenger_Kill_Share_Scythe => {
                //417
                write!(f, "Passenger Kill Share (Scythe XP)")
            }
            ExperienceType::Passenger_Kill_Share_Sunderer => {
                //418
                write!(f, "Passenger Kill Share (Sunderer XP)")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Infantry_To_Pilot => {
                write!(f, "Gunner Kill Assist To Pilot(Infantry) Share XP") //421
            }
            ExperienceType::Gunner_Kill_Assist_Share_Phalanx_To_Pilot => {
                write!(f, "Gunner Kill Assist To Pilot (Phalanx Turret) Share XP")
                //424
            }
            ExperienceType::Passenger_Kill_Assist_Share_Galaxy => {
                write!(f, "Gunner Kill Assist To Pilot (Galaxy) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Lightning_To_Pilot => {
                write!(f, "Gunner Kill Assist To Pilot (Lightning) Share XP") //428
            }
            ExperienceType::Gunner_Kill_Assist_Share_Reaver_To_Pilot => {
                write!(f, "Gunner Kill Assist To Pilot (Reaver) Share XP") //432
            }
            ExperienceType::Gunner_Kill_Assist_Share_Scythe_To_Pilot => {
                write!(f, "Kill Assist To Pilot (Scythe) Share XP") //433
            }
            ExperienceType::Gunner_Kill_Assist_Share_Sunderer_To_Pilot => {
                write!(f, "Kill Assist To Pilot (Sunderer) Share XP") //434
            }
            ExperienceType::Gunner_Kill_Share_Valkyrie => {
                write!(f, "Gunner Kill Share (Valkyrie) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Valkyrie => {
                write!(f, "Gunner Kill Assist (Valkyrie) Share XP")
            }
            ExperienceType::Passenger_Kill_Share_Valkyrie => {
                //518
                write!(f, "Passenger Kill Share (Valkyrie) XP")
            }
            ExperienceType::Gunner_Kill_Assit_Share_To_Pilot_Valkyrie => {
                //519
                write!(f, "Gunner Kill Assist To Pilot (Valkyrie) Share XP")
            }
            ExperienceType::Gunner_Kill_Share_Construction_Med => {
                write!(f, "Gunner Kill Share (Construction - Medium) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Construction_Med => {
                write!(f, "Gunner Kill Assist (Construction - Medium) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_To_Pilot_Construction_Med => {
                write!(
                    f,
                    "Gunner Kill Assist To Pilot (Construction - Medium) Share XP"
                ) //614
            }
            ExperienceType::Gunner_Kill_Share_Construction_Small => {
                write!(f, "Gunner Kill Share (Construction - Small) XP")
            }
            ExperienceType::Passenger_Kill_Share_Construction_Small => {
                write!(f, "Passenger Kill Share (Construction - Small) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Construction_Small => {
                write!(f, "Gunner Kill Assist (Construction - Small) Share XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_To_Pilot_Construction_Small => {
                write!(
                    f,
                    "Gunner Kill Assist To Pilot (Construction - Small) Share XP"
                ) //626
            }
            ExperienceType::Gunner_Kill_Share_Construction_Large => {
                write!(f, "Gunner Kill Share (Construction - Large) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Construction_Large => {
                write!(f, "Gunner Kill Assist (Construction - Large) Share XP")
            }
            ExperienceType::Passenger_Kill_Share_Construction_Large => {
                write!(f, "Passenger Kill Share (Construction - Large) XP")
            }
            ExperienceType::Gunner_KIll_Assist_Share_To_Pilot_Construction_large => {
                write!(
                    f,
                    "Gunner Kill Assist To Pilot (Construction-Large) Share XP"
                ) //638
            }
            ExperienceType::Gunner_Kill_Share_Ant => {
                write!(f, "Gunner Kill Share (Ant) XP") //667
            }
            ExperienceType::Gunner_Kill_Assist_Share_Ant => {
                write!(f, "Gunner Kill Assist (Ant) Share XP") //668
            }
            ExperienceType::War_Asset_Destruction_Standard => {
                //1553
                write!(f, "War Asset Destroyed (Standard) XP")
            }
            ExperienceType::War_Asset_Destruction_Valuable => {
                //1554
                write!(f, "War Asset Destroyed (Valuable) XP")
            }
            ExperienceType::War_Asset_Destruction_Epic => {
                //1555
                write!(f, "War Asset Destroyed (Epic) XP")
            }
            ExperienceType::Vehicle_Damage_Chimera => write!(f, "Vehicle Damage (Chimera) XP"),
            ExperienceType::Kill_Assist_Chimera => write!(f, "Kill Assist Chimera XP"),
            ExperienceType::Missing_1563 => write!(f, "Missing (1563) XP"),
            ExperienceType::Tank_Superiority_Bonus => write!(f, "Tank Superiority Bonus XP"), //1564
            ExperienceType::Vehicle_Destruction_Chimera => write!(f, "Destroy Chimera XP"),
            ExperienceType::Gunner_Kill_Assist_Share_Chimera => {
                //1567
                write!(f, "Guner Kill Asssit (Chimera) Share XP")
            }
            ExperienceType::Gunner_Kill_Share_Chimera => {
                //1568
                write!(f, "Gunner Kill Share (Chimera) XP")
            }
            ExperienceType::Vehicle_Repair_Chimera => write!(f, "Repair Chimera XP"), //1571
            ExperienceType::Vehicle_Destruction_Dervish => write!(f, "Destroy Dervish XP"),
            ExperienceType::Kill_Assist_Dervish => write!(f, "Kill Assist Dervish XP"),
            ExperienceType::Missing_1646 => write!(f, "Missing (1646) XP"),
            ExperienceType::Surface_To_Air_Dervish => write!(f, "Anti-air Damage (Dervish) XP"),
            ExperienceType::Dervish_Damage => write!(f, "Vehicle Damage (Dervish) XP"),
            ExperienceType::Fighter_Superiority_Bonus => write!(f, "Fighter Superiority Bonus XP"),
            ExperienceType::Gunner_Kill_Share_Dervish => {
                write!(f, "Gunner Kill Share (Dervish) XP")
            }
            ExperienceType::Gunner_Kill_Assist_Share_Dervish => {
                write!(f, "Gunner Kill Assist (Dervish) Share XP")
            }
            ExperienceType::Vehicle_Destruction_Corsair => write!(f, "Destroy Corsair XP"),
            ExperienceType::Kill_Assist_Corsair => write!(f, "Kill Assist Corsair XP"),
            ExperienceType::Gunner_Kill_Assist_Share_Corsair_Alt => {
                write!(f, "Gunner Kill Assist (Corsair) Share XPa") //2005
            }
            ExperienceType::Gunner_Kill_Assist_Share_Corsair => {
                write!(f, "Gunner Kill Assist (Corsair) Share XP") //2006
            }
            ExperienceType::Gunner_To_Pilot_Kill_Assist_Share_Corsair => {
                write!(f, "Gunner to Pilot Kill Assist (Corsair) Share XP") //2008
            }
            ExperienceType::Gunner_Kill_Corsair_Bonus => {
                write!(f, "Gunner Kill (Corsair) Bonus XP") //2055
            }
            ExperienceType::Podium_Defense_Bonus => {
                write!(f, "Podium Defense Bonus XP") //2132
            }
            ExperienceType::Conduit_Repository_Hack => {
                write!(f, "Conduit Repository Hack XP") //2135
            }
            other => write!(f, "unused xp ({})", *other as i64),
        }
    }
}
