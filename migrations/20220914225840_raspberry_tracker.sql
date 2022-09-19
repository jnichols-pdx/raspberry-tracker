--Add 'team' column to session table. Census has exposed 'team_id' for many events
--however the "abandoned enemy vehicle destroyed reports the player as it's owner"
--bug remains. We can now filter these out relatively easily, but we need to be 
--able to compare the team_id of the dead vehicle to the player's session team_id,
--so why not store that team_id in the db if we are gathering it anyway.

--Increase DB version 0.60

ALTER TABLE sessions ADD team INTEGER;

--Faction and Team underlying integers are the same for the primary factions, we
--can infer the team for non NSO sessions this way.
UPDATE sessions SET team = c_faction;

--However NSO faction may change from session to session, don't infer prior team
--for NSO characters. 0 == Unknown.
UPDATE sessions SET team = 0 WHERE team IS 4;

UPDATE raspberrytracker SET version = 0.60; 
