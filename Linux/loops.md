

## for loop

Syntax
``` bash
for var in list; do
    commands
done
```

`list` can be:
* (( list of items ))
* {n..m} # range
* $(seq n m) # sequence

## C-Style for Loop

Syntax
``` bash
for (( initialization; condition; increment )); do
    commands
done
```


## WHILE loop

Syntax
``` bash
while condition; do
    commands
done
```


## Until loop

Syntax
``` bash
until condition; do
    commands
done
```

## Select loop 

Syntax
``` bash
select var in list; do
    commands
done
```

--- 

| **Loop Type**  | **Description**                                              | **When to Use**                                                                                     | **Example Use Case**                                                                 |
|----------------|--------------------------------------------------------------|-----------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------|
| `for`          | Iterates over a list of items or a sequence.                 | When you have a predefined list or sequence of items to process.                                    | Iterating through filenames, strings, or numbers in a range.                        |
| C-Style `for`  | Mimics C-style looping with initialization, condition, and increment. | When you need more control over iteration, such as using counters with specific increments.         | Counting from 1 to 10, skipping every 2 numbers.                                    |
| `while`        | Executes as long as a condition evaluates to true.           | When the number of iterations is not known beforehand and depends on a condition.                  | Reading a file line by line or waiting for a specific condition to be met.          |
| `until`        | Executes until a condition becomes true.                     | When you want to loop until a condition is satisfied (opposite logic of `while`).                  | Waiting for a file to exist or a process to complete.                               |
| `select`       | Presents a menu for user input and iterates over options.    | When you need to create a simple interactive menu for the user to choose an option.                | Selecting an option from a list of commands or files.                               |
| Infinite Loop  | Continues indefinitely unless broken manually.               | When running a continuous process or server-like functionality with manual exit logic.              | Monitoring a service, checking logs, or running a daemon process.                   |

### **Control Statements**
- **`break`**: Use to exit a loop prematurely.
- **`continue`**: Use to skip the current iteration and move to the next one.
