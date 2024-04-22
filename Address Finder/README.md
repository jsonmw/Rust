Uses regular expressions to scan text for addresses.

"\\d\*\\s+\\w+\\s+\\w+\\.,\*\\s\*\\w\*\\s\*\\w\*,\*\\s+\\w{2},\*\\s+\\d{5}"

Which allows for addresses in the form 

<optional street number> <street name> <city name> <state abbreviation> <5-digit zip code>, in which each piece may or may not be separated by commas.

To break this down, the interesting parts are 
- \\d\* : some number of zero or more digits 
- \\s+ : white space
- \\w+ : one more String words 
- \\.,* : optional period and comma
- ,\*   : optional comma
- w{2} : two-character String for the state abbreviation
- d{5} : 5-digit number for the zip-code
