--Manually add new weapons and tank cannons released circa Halloween 2022.

--Discovered https://github.com/carlst99/Sanctuary.Census and https://census.lithafalcon.cc.
---Here we use their weapon category IDs for Chimera, Corsair, Javelin and underwater guns.
---This may conflict with some future update to the official census data, and require corrections.
---Raspberry tracker will continue to use the official PS2 Census for data queries at this time.

--Increase DB version to 0.64

INSERT INTO weapons VALUES ('6013929','JGX12',132);
INSERT INTO weapons VALUES ('6013930','P4-120 Kingsnake',123);
INSERT INTO weapons VALUES ('6013931','Perihelion VXC',120);
INSERT INTO weapons VALUES ('6013932','Larion PPC',217);

INSERT INTO weapons VALUES ('6013945','JGX11',118);
INSERT INTO weapons VALUES ('6013946','L2-100 Kingsnake',118);
INSERT INTO weapons VALUES ('6013947','Perihelion L-VXC',118);
INSERT INTO weapons VALUES ('6013948','Larion LPPC',118);

INSERT INTO weapons VALUES ('6013950','NS-66 \"Alchemist\" Punisher',5);

INSERT INTO weapons VALUES ('6013975','NC14 \"Dawnbreaker\"',11);
INSERT INTO weapons VALUES ('6013976','M77-B \"Dawnbreaker\"',11);
INSERT INTO weapons VALUES ('6013977','XM98 \"Dawnbreaker\"',11);
INSERT INTO weapons VALUES ('6013978','SR-100 \"Dawnbreaker\"',11);

INSERT INTO weapons VALUES ('6013979','NS-R3 \"Dawnstinger\" Swarm',13);

--Javelin Primary categories
UPDATE weapons SET category = 216 WHERE id = 6010044; -- JVN-30 Salamander
UPDATE weapons SET category = 216 WHERE id = 6010076; -- JVN-X3 Hydra
UPDATE weapons SET category = 216 WHERE id = 6010078; -- JVN-50 Celeste
--Still not touching the JVN-MM N.E.S.T. - it exists, with category id 140 instead of 216, in the official census.

--Chimera Primary categories
UPDATE weapons SET category = 217 WHERE id = 6009957; -- CT-135
UPDATE weapons SET category = 217 WHERE id = 6010678; -- CT-150 Cyclops
UPDATE weapons SET category = 217 WHERE id = 6010679; -- CT-102 Satyr

--Chimera Front categories
UPDATE weapons SET category = 218 WHERE id = 6010715; -- CT2-20 HCG
UPDATE weapons SET category = 218 WHERE id = 6010716; -- CT2-XP
UPDATE weapons SET category = 218 WHERE id = 6010717; -- CT2-8M Siren 


--Corsair categories
UPDATE weapons SET category = 221 WHERE id = 6013616; -- M18 Palisade-C front
UPDATE weapons SET category = 222 WHERE id = 6013617; -- M18 Palisade-C rear

UPDATE weapons SET category = 221 WHERE id = 6013623; -- N30 Trawler-C rear
UPDATE weapons SET category = 222 WHERE id = 6013624; -- N30 Trawler-C rear

UPDATE weapons SET category = 221 WHERE id = 6013625; -- C2-20 HCG front
UPDATE weapons SET category = 222 WHERE id = 6013626; -- C2-20 HCG rear

UPDATE weapons SET category = 221 WHERE id = 6013627; -- V42 Pariah-C rear
UPDATE weapons SET category = 222 WHERE id = 6013628; -- V42 Pariah-C rear

UPDATE weapons SET category = 221 WHERE id = 6013335; -- M20 Basilisk rear
UPDATE weapons SET category = 222 WHERE id = 6013336; -- M20 Basilisk rear

--Amphibious Rifles
UPDATE weapons SET category = 220 WHERE id = 6011526; -- UBR-100 Frogman
UPDATE weapons SET category = 220 WHERE id = 6011527; -- UBR-150 Sea Lion
UPDATE weapons SET category = 220 WHERE id = 6011528; -- UBR-300 Swordfish
UPDATE weapons SET category = 220 WHERE id = 6013842; -- UBR-300 Swordfish-P

--Amphibious Pistols
UPDATE weapons SET category = 223 WHERE id = 6011538; -- UBP-1 Starfish
UPDATE weapons SET category = 223 WHERE id = 6013833; -- UBP-1 "Steel" Starfish

--Heavy Crossbows
UPDATE weapons SET category = 219 WHERE id = 6011432; -- Seeker HLX
UPDATE weapons SET category = 219 WHERE id = 6011442; -- Demon Seeker HLX
UPDATE weapons SET category = 219 WHERE id = 6011831; -- Seraphim HLX, previously assumed 'normal' crossbow category 24.

UPDATE raspberrytracker SET version = 0.64;
