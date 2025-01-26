# PS1 (Prompt String 1)

### Common Escape Sequences in PS1
- `\u` : Username  
- `\h` : Hostname (up to the first dot)  
- `\H` : Full hostname  
- `\w` : Working directory (full path)  
- `\W` : Working directory (last directory only)  
- `\t` : Time in 24-hour format  
- `\@` : Time in 12-hour am/pm format  

---

### Notes
1. **Escape Sequences and Line Length**  
   Escape sequences are counted as characters, so they may affect the line length of the prompt. To avoid issues, the shell needs to know the actual length of `PS1`. Use `\[ ... \]` to enclose escape sequences and exclude them from length calculations.  

   - Inside `\[ ... \]`, only `tput` commands should be used.  
     **Example:**  
     ```bash
     PS1="\[$(tput bold)> $(tput sgr0)\]"
     ```

   - Refer to the [Bash manual](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html) for more details.

2. **Unicode Support**  
   Unicode characters can be included in `PS1`. See [this list](https://en.wikipedia.org/wiki/List_of_Unicode_characters) for examples.

---

### Example
```bash
PS1="\u@\h:\w$ "
```

---

# Colors and Escape Sequences

### Color Formatting
- **Reset Changes:** `echo -e "\e[0m"`  
- **General Format:** `echo -e "\e[X;Ym"`  
  - `X`: Foreground (FG) color  
  - `Y`: Background (BG) color  

| **FG Code** | **BG Code** | **Color**             |
|-------------|-------------|-----------------------|
| 30          | 40          | Black                |
| 31          | 41          | Red                  |
| 32          | 42          | Green                |
| 33          | 43          | Yellow               |
| 34          | 44          | Blue                 |
| 35          | 45          | Magenta              |
| 36          | 46          | Cyan                 |
| 37          | 47          | White                |
| 90          | 100         | Bright Black (Gray)  |
| 91          | 101         | Bright Red           |
| 92          | 102         | Bright Green         |
| 93          | 103         | Bright Yellow        |
| 94          | 104         | Bright Blue          |
| 95          | 105         | Bright Magenta       |
| 96          | 106         | Bright Cyan          |
| 97          | 107         | Bright White         |

---

### Notes
- You can omit `X` or `Y` to apply only one setting.  
  **Examples:**  
  - `\e[31m` : Red text  
  - `\e[1;31m` : Bold red text  

---

### Examples
```bash
echo -e "\e[1;31mThis is bold red text\e[0m"
```

---

# `tput` Commands and `infocmp`

### Common `tput` Commands
- `tput clear` : Clears the screen  
- `tput cup [x] [y]` : Moves the cursor to position `(x, y)`  
- `tput sgr0` : Resets formatting  
- `tput bold` : Enables bold text  
- `tput smul` : Starts underline  
- `tput rmul` : Stops underline  
- `tput setaf [color]` : Sets foreground color  
- `tput setab [color]` : Sets background color  

| **Color Code** | **Color**  |
|----------------|------------|
| 0              | Black      |
| 1              | Red        |
| 2              | Green      |
| 3              | Yellow     |
| 4              | Blue       |
| 5              | Magenta    |
| 6              | Cyan       |
| 7              | White      |

---

### Additional `tput` Commands
- `tput lines` : Gets the number of lines in the terminal  
- `tput cols` : Gets the number of columns in the terminal  
- `tput colors` : Gets the number of supported colors  

---

### Notes
- Use the `infocmp` command to find all available `tput` options and escape sequences.

---

### Examples
```bash
echo "$(tput bold)This is bold text$(tput sgr0)"
echo "$(tput smul)This is underlined text$(tput rmul)"
echo "$(tput setaf 1)This is red text$(tput sgr0)"
for i in {0..15}; do echo "$(tput setab ${i}) Background color: $i $(tput sgr0)"; done
```