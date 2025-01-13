#!/bin/bash

echo "Start:"
sleep 2

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

