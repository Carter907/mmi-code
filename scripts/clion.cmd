@echo off

set waitarg=
set intellij_args=

:next
set "passwait="
if "%~1"=="--wait" set passwait=1
if "%~1"=="-w" set passwait=1
if defined passwait (set waitarg=/wait)
if not "%~1"=="" (
  if defined passwait (set "intellij_args=%intellij_args%--wait ") else (set "intellij_args=%intellij_args%%1 ")
  shift
  goto next
)

start "" %waitarg% C:\Users\%USERNAME%\AppData\Local\Programs\CLion\bin\clion64.exe %intellij_args%