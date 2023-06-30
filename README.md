# sports-scores-cli-rust
soccer stats cli app - get quick soccer stats and match information using  api-football.

# Summary 
The program runs with `cargo run` and prompts the user for their team name, and country. From this, the string of the team and country gets passed into 
the async search function `search_for_team` that uses the API from api-football` to retrieve an array of teams matching the input.

The print team function then returns the general team information. From here we are able to obtain 
the team ID, and make another request in the `get_team_stats` function that retrieves the match data and recent scores for that given team, using the team ID as an argument. 

Both the team and match data are printed after being parsed using the helper functions. 

The response data is typed using structs in separate files that get imported into main.rs. 


# Plans for improvement

- adding information for players like top scorers, most assists, etc.
- showing information like team form, biggest wins, biggest loss,
- showing information for upcoming matches 
