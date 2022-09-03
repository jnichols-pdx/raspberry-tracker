--Manually add newer weapons still missing from the API (A7 NSX weapons, Archer reworks, empire specific topguns, UBR guns, BOAT! weapons, etc.)
--Increase DB version 0.52



INSERT INTO weapons VALUES ('6011872','N30 Trawler-S',129);
INSERT INTO weapons VALUES ('6011873','N30 Trawler-S',130);
INSERT INTO weapons VALUES ('6011881','N30 Trawler-A',144);
INSERT INTO weapons VALUES ('6011885','N30 Trawler-H',114);
INSERT INTO weapons VALUES ('6011889','N30 Trawler',131);
INSERT INTO weapons VALUES ('6013623','N30 Trawler-C',0); --Front, Corsair gun categories unknown
INSERT INTO weapons VALUES ('6013624','N30 Trawler-C',0); --Rear

INSERT INTO weapons VALUES ('6011874','M18 Palisade-S',129);
INSERT INTO weapons VALUES ('6011875','M18 Palisade-S',130);
INSERT INTO weapons VALUES ('6011879','M18 Palisade-A',144);
INSERT INTO weapons VALUES ('6011883','M18 Palisade-H',114);
INSERT INTO weapons VALUES ('6011887','M18 Palisade',123);
INSERT INTO weapons VALUES ('6013616','M18 Palisade-C',0); --Front
INSERT INTO weapons VALUES ('6013617','M18 Palisade-C',0); --Rear

INSERT INTO weapons VALUES ('6011876','V42 Pariah-S',130); --BACKWARD THESE TWO ARE
INSERT INTO weapons VALUES ('6011877','V42 Pariah-S',129); --ERA OWT ESEHT DRAWKCAB
INSERT INTO weapons VALUES ('6011880','V42 Pariah-A',144);
INSERT INTO weapons VALUES ('6011884','V42 Pariah-H',114);
INSERT INTO weapons VALUES ('6011888','V42 Pariah',119);
INSERT INTO weapons VALUES ('6013627','V42 Pariah-C',0); --Front
INSERT INTO weapons VALUES ('6013628','V42 Pariah-C',0); --Rear

INSERT INTO weapons VALUES ('6013335','M20 Basilisk',0); --Corsair Front
INSERT INTO weapons VALUES ('6013336','M20 Basilisk',0); --Corsair Rear

INSERT INTO weapons VALUES ('6012441','NS SQU-34K',2);

INSERT INTO weapons VALUES ('6011869','S2-20 HCG',129);
INSERT INTO weapons VALUES ('6011870','S2-20 HCG',130);
INSERT INTO weapons VALUES ('6011882','A2-20 HCG',144);
INSERT INTO weapons VALUES ('6011886','H2-20 HCG',114);
INSERT INTO weapons VALUES ('6013625','C2-20 HCG',0); --Front
INSERT INTO weapons VALUES ('6013626','C2-20 HCG',0); --Rear

INSERT INTO weapons VALUES ('6011526','UBR-100 Frogman',0); --Amphibious rifle
INSERT INTO weapons VALUES ('6011527','UBR-150 Sea Lion',0); --Amphibious rifle
INSERT INTO weapons VALUES ('6011528','UBR-300 Swordfish',0); --Amphibious rifle
INSERT INTO weapons VALUES ('6011538','UBP-1 Starfish',0); --Amphibious pistol
INSERT INTO weapons VALUES ('6013833','UBP-1 "Steel" Starfish',0); --Ampibious pistol
INSERT INTO weapons VALUES ('6013842','UBR-300 Swordfish-P',0); --Amphibious rifle

INSERT INTO weapons VALUES ('6011152','NSX Kuwa',157); --Dat Hybrid Rifle Tho
INSERT INTO weapons VALUES ('6011153','NSX Kappa',5);
INSERT INTO weapons VALUES ('6011154','NSX Yawara',3);
INSERT INTO weapons VALUES ('6011155','NSX Sesshin',12);

INSERT INTO weapons VALUES ('6011389','Mega Soldier Soaker XP100',14);
INSERT INTO weapons VALUES ('6011409','Mega Soldier Soaker (Imperial Purple)',14);
INSERT INTO weapons VALUES ('6011412','Mega Soldier Soaker (Ballistic Blue)',14);
INSERT INTO weapons VALUES ('6011415','Mega Soldier Soaker (Riot Red)',14);
INSERT INTO weapons VALUES ('6011418','Mega Soldier Soaker (Electric Sky)',14);

INSERT INTO weapons VALUES ('6011432','Seeker HLX',0); --Heavy Crossbow category
INSERT INTO weapons VALUES ('6011442','Demon Seeker HLX',0);  --Heavy Crossbow category

INSERT INTO weapons VALUES ('6012324','SPRK-33''s Stomper',14);

INSERT INTO weapons VALUES ('6013463','LA60 Masthead',11);
INSERT INTO weapons VALUES ('6013464','DAGR-81',11);
INSERT INTO weapons VALUES ('6013465','VX4-3 Slicer',11);
INSERT INTO weapons VALUES ('6013466','NE-36 Linecutter',11);

INSERT INTO weapons VALUES ('6010117','NS-66 Punisher SE',5);

INSERT INTO weapons VALUES ('6013480','NC1 Gauss Rifle',7);
INSERT INTO weapons VALUES ('6013477','"Apex" T1 Cycler',7);
INSERT INTO weapons VALUES ('6013476','"Apex" Pulsar VS1',7);

INSERT INTO weapons VALUES ('6011773','NS "Tigerstrike" Decimator',13);

INSERT INTO weapons VALUES ('6013893','NS-45 "Assassin" Pilot',3);
INSERT INTO weapons VALUES ('6013894','NS-7 "Assassin" PDW',5);

UPDATE raspberrytracker SET version = 0.52; 
