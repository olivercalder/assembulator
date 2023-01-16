# Turing

An architecture based on Alan Turing's "a-machine", commonly known as a Turing machine.

## Instruction Set

Programs are defined as a list of states and transitions which read from, write to, and move along a "tape" which is initially populated from stdin.
Given a current state and position on the tape, a symbol is read from the tape, and if it matches any of the transitions from the current state, that transition is taken, which entails overwriting the current cell of the tape (optional), moving left, right, or staying, and setting the current state to the next state indicated by the transition.
Once the program halts, the current contents of the "tape" are printed to stdout.

### States

Each state is labeled `qX` where `X` may be a number, a special state, or a string.
Each program begins at the state `q0`.
The other special states are `qA` (accept), `qH` (halt), and `qR` (reject).
When any of these states are reached, the program halts and the current contents of the "tape" are printed to stdout.
Additionally, if the program halts on state `qA` or `qH`, the program exits with exit code `0`, and if it halts on state `qR`, the program exits with exit code `1`.

### Transitions

For a given current and next state, a transition is defined by a string of symbols to which the symbol read from the tape is compared, a symbol to write at the current position on the tape (optional), and a direction to move along the tape: `L` (left), `R` (right), or `S` (stay).
Each transition is of the form:

```
CURRENT_STATE->NEXT_STATE: SYMBOL_STRING;[SYMBOL_TO_WRITE,]DIRECTION
```

If the string of symbols begins with the negation symbol `!`, then the transition will match if the symbol read from the tape does not match any of the remaining symbols in the string.
The underscore character `_` is used by default to indicate a blank cell, which occurs when the position on the tape moves beyond the string of characters which was loaded from stdin.
For all symbols which are not matched by the symbol strings of any defined transition from the current state, it is implied that there exists a transition to `qH` from the current state which contains those symbols in its symbol string.

### Special Characters

One may wish to define transitions which contain one or more of the symbols used for formatting: `!`, `_`, `;`, `,`, and ` ` (space).
To include one of these symbols in a transition's symbol string or symbol to write, it may be escaped using the backslash character `\`.
Thus, to include a literal backslash character as a symbol, it must be escaped as well.

For example, the following is a transition which matches any of `a`, `b`, `!`, or `_` and writes a `,` character:

```
q4->q2: ab\!\_;\,,L
```

Other ASCII characters which are traditionally escaped, including `\0`, `\a`, `\b`, `\t`, `\n`, `\v`, `\f`, and `\r`, may be escaped as usual.

Any ASCII character may be expressed in its hex value using an expression of the form `\x61`.

Thus, the complete list of special characters is as follows:

| Character | Description                |
| ----      | ----                       |
| `\0`      | null character             |
| `\a`      | bell character             |
| `\b`      | backspace                  |
| `\t`      | tab                        |
| `\n`      | newline                    |
| `\v`      | vertical tab               |
| `\f`      | form feed                  |
| `\r`      | carriage return            |
| `\ `      | space                      |
| `\!`      | literal `!`                |
| `\,`      | literal `,`                |
| `\;`      | literal `;`                |
| `\x`      | literal byte, as in `\x7f` |
| `\\`      | literal `\`                |
| `\_`      | literal `_`                |

#### Unicode Support vs Arbitrary Bytes

By default, unicode characters are supported as symbols.
Rust allows characters up to `0x7f` to be specified using expressions the form `\x61`.
A value greater than `0x7f` cannot be interpreted as a char by the compiler.
However, it may be the case that the program is intended to operate on arbitrary bytes as integers.

Reading input as literal bytes precludes treating it as unicode, since multi-byte symbols are split into multiple separate bytes.
However, it may be the case that a program was written to operate on, for example, the SYN character (`0x16`) while also working with unicode characters which span multiple bytes.

Thus, the user must specify whether to treat the input as raw bytes rather than unicode.
This can be done using the `--raw` flag.

TODO: unimplemented

### Other Specifications

- Any line beginning with a `#` character is treated as comment and ignored during parsing.
- Blank lines are ignored.
- After the `:` symbol which follows the destination state of a transition, any number of spaces may occur before the beginning of the symbol string.
  - This is the only place where non-escaped spaces are allowed.
- State names may not contain any special characters listed in the [#special-characters](Special Characters) section.

## Example Program

The following program computes whether the input on the tape contains the string `CAT`, and overwrites all characters except any occurrence of `CAT` with `x` symbols.

```
q0->q0: !C;x,R
q0->q1: C;R
q1->q2: A;R
q1->q4: !A;x,L
q2->q3: T;R
q2->q4: !T;x,L
q3->q3: !_;x,R
q3->qA: _;S
q4->q4: !C;x,L
q4->q0: C;x,R
```

## Linking Programs

Allow jumping to `q0` and from `q[AHR]` of an external program using a `block` directive of the following form:

```
block: qMyHelper=some_other_file.tm
```

TODO: unimplemented
