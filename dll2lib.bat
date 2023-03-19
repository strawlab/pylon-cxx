REM Usage: dll2lib libldap.dll

SETLOCAL
set dll_file=%1
set dll_file_no_ext=%dll_file:~0,-4%
set exports_file=%dll_file_no_ext%-exports.txt
set def_file=%dll_file_no_ext%.def
set lib_file=%dll_file_no_ext%.lib
set lib_name=%dll_file_no_ext%

dumpbin /exports %dll_file% > %exports_file%

echo LIBRARY %lib_name% > %def_file%
echo EXPORTS >> %def_file%
for /f "skip=19 tokens=1,4" %%A in (%exports_file%) do if NOT "%%B" == "" (echo %%B @%%A >> %def_file%)

lib /def:%def_file% /out:%lib_file% /machine:x64

REM Clean up temporary intermediate files
del %exports_file% %def_file% %dll_file_no_ext%.exp
