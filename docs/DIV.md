## DIV
The ``DIV`` instruction allows the application to get values of registers and return the div to a register.
```
LOAD 01 00 02 # load 2 to register 1
LOAD 02 00 01 # pointer to register 1

LOAD 04 00 03 # save location pointer
DIV 02 02 04 # will save the div of the 2 numbers to register 3
```
