--Add a column for record weapon classes (item_category_id on API) to weapons table.
--Insert assumed weapon classes for NSO / bundle weapons not currently in the census API.
--Increase DB version 0.4

ALTER TABLE weapons ADD category INTEGER;

UPDATE weapons SET category = 7  WHERE id IS '6011252';
UPDATE weapons SET category = 8  WHERE id IS '6011251';
UPDATE weapons SET category = 6  WHERE id IS '6011254';
UPDATE weapons SET category = 6  WHERE id IS '6011565';
UPDATE weapons SET category = 12 WHERE id IS '6010045';
UPDATE weapons SET category = 12 WHERE id IS '6010047';
UPDATE weapons SET category = 12 WHERE id IS '6010046';
UPDATE weapons SET category = 12 WHERE id IS '6011257';
UPDATE weapons SET category = 4  WHERE id IS '6011256';
UPDATE weapons SET category = 3  WHERE id IS '6011258';
UPDATE weapons SET category = 11 WHERE id IS '6010510';
UPDATE weapons SET category = 11 WHERE id IS '6010512';
UPDATE weapons SET category = 11 WHERE id IS '6010511';
UPDATE weapons SET category = 11 WHERE id IS '6011255';
UPDATE weapons SET category = 5  WHERE id IS '6011253';
UPDATE weapons SET category = 14 WHERE id IS '6011064';
UPDATE weapons SET category = 14 WHERE id IS '6011140';
--UPDATE weapons SET category =  WHERE id IS '6010044'; --Javelin weapon categories are unknown
--UPDATE weapons SET category =  WHERE id IS '6010078';
--UPDATE weapons SET category =  WHERE id IS '6010076';
--UPDATE weapons SET category =  WHERE id IS '6010679'; --Chimera weapon categories are unkonwn
--UPDATE weapons SET category =  WHERE id IS '6009957';
--UPDATE weapons SET category =  WHERE id IS '6010678';
--UPDATE weapons SET category =  WHERE id IS '6010717';
--UPDATE weapons SET category =  WHERE id IS '6010715';
--UPDATE weapons SET category =  WHERE id IS '6010716';
--UPDATE weapons SET category =  WHERE id IS '6009861'; --Dervish weapon categories are unknown
--UPDATE weapons SET category =  WHERE id IS '6009991';
--UPDATE weapons SET category =  WHERE id IS '6009992';
UPDATE weapons SET category = 6  WHERE id IS '6011563';
UPDATE weapons SET category = 6  WHERE id IS '6011564';
UPDATE weapons SET category = 6  WHERE id IS '6011562';
UPDATE weapons SET category = 3  WHERE id IS '6011555';
UPDATE weapons SET category = 2  WHERE id IS '6011556';
UPDATE weapons SET category = 24 WHERE id IS '6011831';

UPDATE weapons SET category = 0  WHERE category IS NULL; --Unknown

UPDATE raspberrytracker SET version = 0.40; 
