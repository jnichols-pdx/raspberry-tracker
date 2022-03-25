--Manually add NSO / bundle weapons not currently described in census API
--Increase DB version 0.21

--Some of the NSO weapons are listed in Census, some are not.

--INSERT INTO weapons VALUES ('6009864','AR-100');
--INSERT INTO weapons VALUES ('6009891','AR-101');
--INSERT INTO weapons VALUES ('6009892','AR-N203');
INSERT INTO weapons VALUES ('6011252','AR-ARX Maxwell');
--INSERT INTO weapons VALUES ('6009893','CB-100');
--INSERT INTO weapons VALUES ('6009895','CB-200');
--INSERT INTO weapons VALUES ('6009894','CB-X75');
INSERT INTO weapons VALUES ('6011251','CB-ARX Newton');
--INSERT INTO weapons VALUES ('6009899','XMG-100');
--INSERT INTO weapons VALUES ('6009900','XMG-155');
--INSERT INTO weapons VALUES ('6009901','XMG-200');
INSERT INTO weapons VALUES ('6011254','XMG-ARX Galilei');
INSERT INTO weapons VALUES ('6011565','"Treasured" XMG-100');
--INSERT INTO weapons VALUES ('6009905','LAV-AG');
--INSERT INTO weapons VALUES ('6009906','LAV-LG');
--INSERT INTO weapons VALUES ('6009907','LAV-LA');
INSERT INTO weapons VALUES ('6010045','BAR-100');
INSERT INTO weapons VALUES ('6010047','BAR-200');
INSERT INTO weapons VALUES ('6010046','BAR-A75');
INSERT INTO weapons VALUES ('6011257','BAR-ARX Feynman');
--INSERT INTO weapons VALUES ('6009908','HSG-400');
--INSERT INTO weapons VALUES ('6009910','SG-100');
--INSERT INTO weapons VALUES ('6009909','SG-A25');
INSERT INTO weapons VALUES ('6011256','SG-ARX Rutherford');
--INSERT INTO weapons VALUES ('6009902','U-100 Lastly');
--INSERT INTO weapons VALUES ('6009903','U-150 Recall');
--INSERT INTO weapons VALUES ('6009904','U-200 Harbinger');
INSERT INTO weapons VALUES ('6011258','U-ARX Dirac');
--INSERT INTO weapons VALUES ('6006850','ADVX // Mako');
INSERT INTO weapons VALUES ('6010510','SR-100');
INSERT INTO weapons VALUES ('6010512','SR-150');
INSERT INTO weapons VALUES ('6010511','SR-L75');
INSERT INTO weapons VALUES ('6011255','SR-ARX Einstein');
--INSERT INTO weapons VALUES ('6009896','PMG-100');
--INSERT INTO weapons VALUES ('6009897','PMG-200');
--INSERT INTO weapons VALUES ('6009898','PMG-3XB');
INSERT INTO weapons VALUES ('6011253','PMG-ARX Schrodinger');
--INSERT INTO weapons VALUES ('6008686','Grenade Printer');
INSERT INTO weapons VALUES ('6011064','D7 Hummingbird');
INSERT INTO weapons VALUES ('6011140','D11 Detonator');
--INSERT INTO weapons VALUES ('6008687','Defector Claws');
INSERT INTO weapons VALUES ('6010044','JVN-30 Salamander');
INSERT INTO weapons VALUES ('6010078','JVN-50 Celeste');
--INSERT INTO weapons VALUES ('6008733','JVN-MM N.E.S.T.');
INSERT INTO weapons VALUES ('6010076','JVN-X3 Hydra');
INSERT INTO weapons VALUES ('6010679','CT-102 Satyr');
INSERT INTO weapons VALUES ('6009957','CT-135');
INSERT INTO weapons VALUES ('6010678','CT-150 Cyclops');
INSERT INTO weapons VALUES ('6010717','CT2-8M Siren');
INSERT INTO weapons VALUES ('6010715','CT2-20 HCG');
INSERT INTO weapons VALUES ('6010716','CT2-XP');
--INSERT INTO weapons VALUES ('6009860','DV-22 Raycaster');
--INSERT INTO weapons VALUES ('6009928','DV-21 Lotus');
--INSERT INTO weapons VALUES ('6009927','DV-XE');
INSERT INTO weapons VALUES ('6009861','DV-22T Lightweaver');
INSERT INTO weapons VALUES ('6009991','DV-LAT Pixie');
INSERT INTO weapons VALUES ('6009992','DV-XAT');

--Include the new weapons from the Expedition to Oshur release.
INSERT INTO weapons VALUES ('6011563','"Treasured" LA-1 Anchor');
INSERT INTO weapons VALUES ('6011564','"Treasured" MSW-R');
INSERT INTO weapons VALUES ('6011562','"Treasured" Orion VS54');
INSERT INTO weapons VALUES ('6011555','NS-44AE Mutiny');
INSERT INTO weapons VALUES ('6011556','The Swashbuckler');

--Include the 2022 Valentine's day bundle weapon
INSERT INTO weapons VALUES ('6011831','Seraphim HLX');

--Unable to find ID#s yet for the 2022 ST Patrick's day bundle weapons
--INSERT INTO weapons VALUES ('??','NS-44 "Jackpot" Commissioner');
--INSERT INTO weapons VALUES ('??','Wyrdwood Shillelagh');

UPDATE raspberrytracker SET version = 0.21; 
