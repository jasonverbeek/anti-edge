@ECHO OFF
echo %~dp0
%~dp0\anti-edge-service.exe stop
%~dp0\anti-edge-service.exe uninstall
pause