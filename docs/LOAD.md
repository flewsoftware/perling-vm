## Load
The ``LOAD`` instruction allows the application to load values to register
```
LOAD 01 00 02 # loads 2 to register 1
LOAD 02 00 01 # loads 1 to register 2 (can be used as a pointer that points to register 1)
# these instuctions can be comiled using the PASM compiler
```

As you can see, you can load any value to any of the 32 registers provided by Perling.   
many instructions require you to have another register point to the data. for this you can `LOAD` a the register number of the data you want to point to another register.
