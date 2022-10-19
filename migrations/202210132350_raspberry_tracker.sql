--ID# for Perihelion VXC from previous migration is incorrect or already out of date. Correcting it.

--Increase DB version to 0.65

UPDATE weapons SET id = '6014236' WHERE id IS '6013931';

UPDATE raspberrytracker SET version = 0.65;
