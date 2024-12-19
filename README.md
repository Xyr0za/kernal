# APL Kernal 

An APL interpreter written in rust. Consists of three main files.

# Lexer

The lexical anaylser of the project; provides funcions to both read the file in question, as well as converting it into a vector of tuples in the form: ("TOKEN TYPE", VAL)

# The parseer

Runs a Shunting Yard Algorithm over the tokkens, which orders them in the correct format. These are then passed into the AST.

# The ABS Tree

The ABS Tree recursively builds the tokens into a Node object, before evaluating it iteratively down the nodes. 
