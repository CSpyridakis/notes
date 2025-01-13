# Conditions

## "No brackets" 
Relying on Command Exit Codes

E.g.
``` bash
if grep -q "pattern" file.txt; then
    echo "Pattern found"
fi
```

Avoid "no brackets" approaches unless your use case is simple or implicit logic suffices.

## Single Brackets []
Used for simple conditional checks, such as string comparisons, file checks, or numeric comparisons.

**Caution:**
Requires spaces between the brackets and the expression.

E.g.
``` bash
# String comparison
if [ "$var" = "value" ]; then
    echo "Match"
fi

# File existence
if [ -f "file.txt" ]; then
    echo "File exists"
fi

# Numeric comparison
if [ "$num" -eq 10 ]; then
    echo "Number is 10"
fi
```

Limitations:

* Cannot handle complex logical expressions (e.g., && or ||) without combining multiple conditions.

* Doesn't support regex matching.

---

## Double Brackets [[]]
Enhanced test command with more features and improved syntax.

**Caution:**
Specific to Bash (not POSIX-compliant). Supports additional operators like &&, ||, and regex matching. Quoting variables is less strict (no errors for empty variables).

E.g.
``` bash
# Logical operators
if [[ "$var" = "value" && "$num" -gt 5 ]]; then
    echo "Conditions met"
fi

# Regex matching
if [[ "$var" =~ ^[A-Za-z]+$ ]]; then
    echo "Variable contains only letters"
fi
```

Advantages:

* Easier syntax for logical operators and regex.

* Safer and more readable compared to [ ].


## Double Parentheses (( ))
Used for arithmetic evaluation.

**Caution:**
Supports C-like arithmetic operations (+, -, *, /, ++, --).
Return status indicates whether the result is non-zero (useful in conditions).

E.g.
``` bash
# Arithmetic comparison
if (( num > 10 )); then
    echo "Number is greater than 10"
fi

# Arithmetic operations
(( count++ ))
(( sum = num1 + num2 ))

```

Advantages:

* Cleaner syntax for arithmetic operations and comparisons.

* No need to use -eq, -gt, etc., for numeric comparisons.
