The goal of this program was to write something that could evaluate logical premises on whether or not they represented a valid deduction. While I couldn't quite get it to work, it was a good learning experience with Rust.

Here is a rough overview of what my plan was:

**Key data structures**:

•	A Vector of Strings to hold the user input premises.
•	A Struct known as “Variable”, containing the variable (or expression), its truth value, and whether or not it implies another truth.
•	A vector of these Variables that contains all known truths.
•	A HashMap of Strings that holds a copy of all implies statement if/then pairs for consulting.

**Program flow**:

•	Accept input, validate that it is only correct characters, place each premise in a vector of premises
•	Go line by line in the premise vector and check for an ‘>’, indicating an implies statement
o	In this case, break the line into left and right strings, for each side of the ‘>’
o	Iterate through each of those strings to see if all variables within are known- if they are not, add them to the “known_truths” vector of Variables.
o	This should produce a starting point with a knowledge of all variables in the given set of premises.
•	With another for loop, evaluate each premise against a nested for loop of all the other premises (except the therefore) given by the user, updating the variables in known_truths as new information is learned.
o	At this stage, entire premises can be added as Variables to the known_truths vector, in addition to simple variables. The first pass through, I would be more interested in building the smallest building blocks, variables, to update and consult.
o	When an implies is found, both sides are added as K/V pairs to the implies HashMap.
•	After the outer for loop executes, an inner for loop checks each premise against each other premise, after which the known_truths table should be fully populated. This would then be used for a final pass over the premises, validating the truthfulness and consulting the HashMap and connecting the implies statements to check the overall validity of the statement.
o	These statements would be checked against known laws of logic, such as modus ponens, modus tollens, etc.
•	The output would then be displayed in the console.
o	The original premise.
o	The validity.
o	The proof.
•	Program exits.
