@ECHO OFF
SETLOCAL EnableDelayedExpansion

:: BUILDING CLIENT AND SERVER.
cargo build --release
cargo build --release -F windows --bin rpa-watcherw
IF %ERRORLEVEL% NEQ 0 GOTO RETURN_LAST_ERROR

RD /Q /S target\dist > NUL
MD target\dist > NUL

:: COPYING SERVER FILES.
Robocopy target\release     target\dist\server          *srv*.exe /PURGE
Robocopy wwwroot            target\dist\server\wwwroot  /MIR
:: :: ICON FILES.
COPY /Y assets\rpa-watcher.ico target\dist\server\wwwroot\favicon.ico
COPY /Y assets\rpa-watcher.ico target\dist\server\wwwroot\view\favicon.ico

:: COPYING CLIENT FILES.
Robocopy target\release     target\dist\client          *.exe *.ini /XF *srv*.exe /PURGE

:: ZIPPING FILES.
IF /I "%1"=="ZIP" (
    SET /P VERSION=Set a version number ^(ie^: 1.2.3^)^:

    DEL /F /Q target\RPA.Watcher*.zip > NUL
    powershell -Command Compress-Archive target\dist\* target\RPA.Watcher-v!VERSION!-Win32.zip -Force
)

:: DONE!
ECHO Build ran with success. Files are located in the "dist" folder.

:RETURN_LAST_ERROR
EXIT /B %ERRORLEVEL%
