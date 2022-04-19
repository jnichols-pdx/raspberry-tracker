--Remove the no longer needed windows table, we are now using the
--builtin Egui persistence system for window / panel sizes.
--Increase DB version to 0.51

DROP TABLE windows;

UPDATE raspberrytracker SET version = 0.51; 
