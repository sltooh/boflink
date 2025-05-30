lib /machine:x64 /def:myapi.def /out:myapi.lib
cl /GS- /c /Fo:custom-api.obj custom-api.c
boflink --custom-api myapi.lib -o custom-api.bof custom-api.obj
