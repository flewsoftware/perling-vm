## ADD
The ``ADD`` instruction allows the application to get values of registers and return the addition to a register.
```
LOAD 01 00 02 # load 2 to register 1
LOAD 02 00 01 # pointer to register 1

LOAD 04 00 03 # save location pointer
ADD 02 02 04 # will save the addition of the 2 numbers to register 3
```
