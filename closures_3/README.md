

### closures_3

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: 
- Borrowing immutably
- Borrowing mutably
- Taking ownership.

The closure will decide which of these to use based on what the body of the function does with the captured values.