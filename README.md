# Brainfck Interpreter

A simple brainf*ck interpreter written in Rust.

## Current Issues
RightJMP instruction interpretation at the moment jumps to the closest [. 
This is invalid behaviour as it does not account for nested loops.
Will likely need to have some kind of stack for these instructions, or something like that.