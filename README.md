# Brainfck Interpreter

A simple brainf*ck interpreter written in Rust.

## Overview

This should work consistently now.
I've tested:
- Simple addition script
- Hello world script

Both are available in add.bf and hello.bf respectively.

# Usage
There are two modes for this particular binary. File mode and interpreter mode.
To interpret a brainfck file, run the application with the filepath as the first argument.
To enter interpreter mode, run the binary with no arguments.

Interpreter mode will take one line of input, and run the code on that line, before allowing you to enter another line.
Each line is a new program with its own memory.