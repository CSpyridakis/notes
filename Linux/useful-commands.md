# Useful Linux commands

## Random file of size N
Create a file, containing random printable characters of specific size

`FILE_SIZE=$((1024 * 10)) ; tr -cd '[:print:]' < /dev/random | head -c ${FILE_SIZE} > file.txt`

Where:
* tr: Translates/deletes characters.
* -c: Complements the set, i.e., matches everything except the specified characters.
* -d: Deletes characters not in the set.
* [:print:]: Matches all printable characters, including spaces.