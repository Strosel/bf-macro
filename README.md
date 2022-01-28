# BF_MACRO

A rust declarative macro that interprets and runs brainfuck.

## Tokens
`brainfuck!` does strip non brainfuck tokens, however due to the recursive nature of declarative
macros this will increase the recusion level and you might have to set a higher recusion limit.
This is also true for large programs.
