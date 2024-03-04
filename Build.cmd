@ECHO OFF

:: BUILDING CLIENT AND SERVER.
cargo build --release
IF %ERRORLEVEL% NEQ 0 GOTO RETURN_LAST_ERROR

MD target\dist

:: COPYING SERVER FILES.
Robocopy target\release     target\dist\server          *srv*.exe /PURGE
Robocopy wwwroot            target\dist\server\wwwroot  /MIR

:: COPYING CLIENT FILES.
Robocopy target\release     target\dist\client          *.exe *.ini /XF *srv*.exe /PURGE

:: ZIPPING FILES.
powershell -Command Compress-Archive target\dist\* target\RPA.Watcher.zip -Force

:: DONE!
ECHO Build ran with success. Files are located in the "dist" folder.

:RETURN_LAST_ERROR
EXIT /B %ERRORLEVEL%
