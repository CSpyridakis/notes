# Shell Expansions

Shell expansions occur due to **word splitting** in Bash. For more details, refer to the [GNU Bash Manual on Shell Expansions](https://www.gnu.org/software/bash/manual/html_node/Shell-Expansions.html).

---

## **Key Concepts and Symbols**

| **Symbol**          | **Description**                                                                                                                                         |
|----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| **IFS**             | The character where word splitting occurs.                                                                                                             |
| **`*`**             | File expansion (matches filenames).                                                                                                                    |
| **`?`**             | Matches a single character.                                                                                                                            |
| **`~`**             | Tilde expansion: expands to the contents of `$HOME`.                                                                                                   |
| **`~+`**            | Tilde plus expansion: expands to the contents of `$PWD`.                                                                                               |
| **`${}`**           | Variable expansion:                                                                                                                                     |
|                      | - Alternatives: `$variable`, `${variable}`, `"$variable"`, `"${variable}"` (**Preferred**).                                                            |
|                      | - Parameter expansion examples:                                                                                                                       |
|                      |     - `${#var}`: Queries the length in bytes.                                                                                                         |
|                      |     - `${var:start:end}`: Substring expansion (cuts a substring).                                                                                     |
|                      |     - `${var/pattern/replacement}`: Replaces a substring (one time).                                                                                  |
|                      |     - `${var//pattern/replacement}`: Replaces all occurrences of a substring.                                                                         |
| **`''`**            | Disables word splitting entirely.                                                                                                                      |
| **`""`**            | Disables most expansions (except variables).                                                                                                           |
| **`\`**             | Escaping (also considered an expansion).                                                                                                               |
| **`{}`**            | Brace expansion (since Bash 4, **does not work with quotes**).                                                                                         |
|                      | - Example:                                                                                                                                             |
|                      |     - `echo data.{csv,txt}` → `data.csv data.txt`.                                                                                                    |
|                      |     - `echo {a..z}` or `touch file{1..10}.txt`.                                                                                                       |
| **`$()`**           | Command substitution: executes a command in a subshell and captures its output.                                                                        |
| **`` (backticks)**| Command substitution (older style, less readable).                                                                                                     |
| **`<()`**           | Process substitution: dynamic input.                                                                                                                   |
| **`()>`**           | Process substitution: dynamic output. Uses the process’s input/output as a temporary file.                                                            |

---

## **Important Notes**
- **Quotes**: Define how Bash handles expansion and word splitting:
  - `cat $PWD/*.txt`: All expansions apply.
  - `cat '$PWD/*.txt'`: Disables all expansions and word splitting.
  - `cat "$PWD/*.txt"`: Disables most expansions (like `*`, word splitting), but **Variable/Parameter** expansions remain enabled.

- **Order of Execution**:
  1. Command is expanded.
  2. Word splitting occurs.
  3. Quotes are removed.
  4. The command is executed.

- **The default action of whitespace is word splitting**
- 
---

## **Examples**

```bash
# Example 1
touch echo hello world
# This will print hello world

# Example 2
touch ./-al file1 file2
ls *

# Example 3
ls *
echo *.txt    # This also includes filtering
```