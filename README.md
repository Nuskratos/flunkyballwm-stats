# flunkyballwm-stats
Stats from the flunkyballwm held on the first saturday in May. 

## TODO 
- Think of an output format for stats, that can be easily added to the website. For a nice display option

## Stats to be calculated
- Throw accuracy (individual, team, everyone) - DONE
- South- vs Northside winrate / points earned - DONE
- Chains of hits & misses (individual, team, everyone) - DONE
- Average amount of throws per round (team, everyone) - DONE
- Average amount of points (Team & individual, which isn't a great statistic) - DONE 
- Average drinking rounds to finish a drink (individual, team, everyone) - DONE
- Effect of 'Strafschluck' (average drinking rounds to finish a drink vs average while getting a 'Strafschluck') - DONE
- Running speeds (individual, team) (Rounds taken for drinks against you vs rounds taken against everyone else)
- - Maybe not doable because either it's being wrong documented or faulty running happened
- Amount of Straf schluck & beers (individual, team) (Amount & per Round) - DONE 
- Enemy accuracy - DONE


## Optional stats probably only useful when there is more data
- South/North influence on stats - Halbwegs
- Back2Back influence on stats
- Amount of beers drank influence on stats
- Effect of first right to throw - Done


# Refactoring stuff
- Learn lifetimes so you don't clone everything for the data classes
- ^Really do that first (not that important anymore since it's fast enough & the functions don't clone anymore)
- Maybe use JSON to store and access the Data 
  - Image recognition for stat adding of future tournaments
- Split calculation into submodules for different calculation themes (i.e. Accuracy, speed, hits, utility, print)
- Somehow add partial_cmp to types, where it's used in sort_by
- Create tests for the calculations with known outcomes
  - Split print&calc functions where needed 