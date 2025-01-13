# Brackets

For more details regarding conditions read [this](./conditions.md).

| **Construct** | **Description**                              | **When to Use**                                                             |
|---------------|----------------------------------------------|-----------------------------------------------------------------------------|
| `( )`         | Subshell: commands run in a separate process | When you want to isolate temporary changes, like directory changes or variables, without affecting the parent shell. |
| `{ }`         | Grouping in the current shell                | When you need to group multiple commands that share variables or environment changes in the current shell.           |
| `[]`          | Basic test                                   | For simple conditional checks (POSIX-compliant), such as file existence or string/numeric comparisons.              |
| `[[ ]]`       | Advanced test                                | For more complex Bash-specific conditions, including logical operators (`&&`, `\|\|`) or regex matching.              |
| `(( ))`       | Arithmetic evaluation                        | When performing numeric operations or comparisons directly within a script.                                        |
