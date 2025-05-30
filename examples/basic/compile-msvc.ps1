cl /GS- /c /Fo:basic.obj basic.c
boflink -o basic.bof basic.obj -lkernel32 -ladvapi32
