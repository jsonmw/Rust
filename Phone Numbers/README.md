This program uses regular expressions to find phone numbers in given text. This was actually pretty complicated, as I tried to make it work for both US (NANP) and international formatting.

I used this regular expression:

`“^[+1]*[-.\( ]*\d{3}{0,1}[-.\) ][2-9]*\d{2}[-. ]*\d{4}$|^[0-9]11$|^988$|^(?:\+|0|00)[-.\( ]*\d{1,3}[-.\( ]*\d{1,8}[-.\( ]*\d{2,10}$"`

Broken down,

NANP numbers :

- `^[+1]` : allows the number to start with a +, +1, or neither
- `[-.\( ]*` : repeated through the expression, this allows for the digits to be separated by whitespace, -, or ., and accounts for ( to begin an area code.
- `\d{3}{0,1}` : allows for an optional area code if not a “local” number. d{3}? should have done the same thing as the {0,1}, specifying one or more occurrences, but for some reason I couldn’t get it to work.
- `[2-9]\d{2}` : gives the exchange code, starting with 2-9 and followed by 2 more digits
- `\d{4}$` : gives the individual subscriber number at the end


NANP emergency numbers :

- `^[0-9]11$` : covers the X11 emergency numbers
- `^988$` : covers 988 emergency numbers


International numbers : 

- `(?:\+|0|00)` : this covered any number that started with some combination of +, 0, 00, which the specification said should be considered international numbers.
- `[-.\( ]*` : covered some combination of whitespace, -, ., and ( ,to allow for varied formats.
- `\d{1,3}` : 1-3 digits representing country code
- `\d{1,8}` : 1-8 digits representing the area/city/region code
- `\d{2,10}$` : 2-10 digits representing the subscriber number, and ending the expression
