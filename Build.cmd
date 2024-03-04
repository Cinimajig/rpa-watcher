@ECHO OFF

cargo build --release
IF %ERRORLEVEL% NEQ 0 GOTO RETURN_LAST_ERROR

RD dist

Robocopy target\release dist *.exe *.ini *.dll
IF %ERRORLEVEL% NEQ 0 GOTO RETURN_LAST_ERROR

ECHO Build ran with success. Files are located in the "dist" folder.

:RETURN_LAST_ERROR
EXIT /B %ERRORLEVEL%
