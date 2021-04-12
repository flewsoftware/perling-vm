[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/flew-software/perling-vm)

<sub>**Perling VM** is part of the Perling project</sub>

## What is perling VM
perling vm is a interpreter for the compiled perling byte code  
**NOTE: perling VM and perling is WIP**

## DEMO
You can find examples of Perling byte code in the examples directory, they can be ran by compiling and running perlingVM with ``examples/print.perling.bin`` as a argument

## OPCODES
| OPCODE | HEX  | Description                                                                       |
|--------|------|-----------------------------------------------------------------------------------|
| HLT    | 0x00 | Halts the program                                                                 |
| LOAD   | 0x01 | Loads data to a register                                                          |
| ADD    | 0x02 | Gets the addition of two values in registers and stores it in another register    |
| SUB    | 0x03 | Gets the subtraction of two values in registers and stores it in another register |
| DIV    | 0x04 | Gets the division of two values in registers and stores it in another register    |
| JMP    | 0x05 | Changes the program counter                                                       |
| RJMP   | 0x06 | Changes the program counter relative to the position                              |
| JMPTL  | 0x07 | Changes the program counter to the position of the label (Not implemented yet)    |
| VMCALL | 0x08 | Calls the inbuilt functions(print, etc..) with upto 2 arguments                   |
| EQ     | 0x09 | checks if equal                                                                   |
| JEQ    | 0x0A | jumps if equal                                                                    |
| NEQ    | 0x0B | checks if not equal                                                               |
| JNEQ   | 0x0C | jump if not equal                                                                 |
| SWP    | 0x0D | swap two register values                                                          |
| AND    | 0x0E | and boolean                                                                       |
| OR     | 0x0F | or boolean                                                                        |
| NOT    | 0x10 | not boolean                                                                       |
| GET    | 0x11 | mv a value from a hidden register to a normal register                            |
| LOCKR  | 0x12 | marks a register as Read-only                                                     |
| PUSHRTS| 0x13 | pushes register content to stack and resets the register value                    |
| IGL    | N/A  | Illegal opcode that will cause panic                                              |
