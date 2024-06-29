
@echo off
rem This script was created by Nuitka to execute 'pystone.exe' with Python DLL being found.
set PATH=D:\python_installs;%PATH%
set PYTHONHOME=D:\python_installs
set NUITKA_PYTHONPATH=D:\gitFolders\WasmHardway\wasmedge_tuto;D:\python_installs\DLLs;D:\python_installs\Lib;D:\python_installs;D:\wasmenv;D:\wasmenv\Lib\site-packages
"%~dp0pystone.exe" %*
