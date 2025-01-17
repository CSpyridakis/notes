# Bash Variables and Their Outputs

| **Variable**   | **Description**                                    | **Output Example**                  |
|-----------------|----------------------------------------------------|-------------------------------------|
| `$0`           | Name of the script                                 | `script.sh`                         |
| `$1, $2, ...`  | Positional parameters (arguments to the script)    | `arg1`, `arg2`, etc.                |
| `$#`           | Number of arguments passed to the script           | `2` (if two arguments are passed)   |
| `$@`           | All arguments passed to the script                 | `arg1 arg2`                         |
| `$?`           | Exit status of the last executed command           | `0` (success) or `non-zero` (failure) |
| `$$`           | Process ID (PID) of the script                     | `12345`                             |
| `$USER`        | Username of the user executing the script          | `john`                              |
| `$HOSTNAME`    | Hostname of the machine                            | `my-computer`                       |
| `$SECONDS`     | Number of seconds since the script started         | `45`                                |
| `$RANDOM`      | A random number                                    | `28657`                             |
| `$LINENO`      | Current line number in the script                  | `15`                                |

Example:
```
echo "> CMD: [\$0] - Name of the script - Output: [$0]"
echo "> CMD: [\$1-\$9] - Bash script input arguments - Output: ['$1', '$2', ...]"
echo "> CMD: [\$#] - Number of arguments - Output: [$#]"
echo "> CMD: [\$@] - Input arguments - Output: [$@]"
echo "> CMD: [\$?] - Exit status of previous command - Output: [$?]"
echo "> CMD: [\$$] - The script process ID - Output: [$$]"
echo "> CMD: [\$USER] - User executing script - Output: [$USER]"
echo "> CMD: [\$HOSTNAME] - Machine hostname - Output: [$HOSTNAME]"
echo "> CMD: [\$SECONDS] - Seconds passed since stript started - Output: [$SECONDS]"
echo "> CMD: [\$RANDOM] - A random number - Output: [$RANDOM]"
echo "> CMD: [\$LINENO] - Current line number - Output: [$LINENO]"
```