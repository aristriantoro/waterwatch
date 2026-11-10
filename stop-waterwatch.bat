@echo off
title Mematikan WaterWatch
chcp 65001 >nul
cls

echo ========================================================
echo    Mematikan Aplikasi WaterWatch
echo ========================================================
echo.

cd /d "%~dp0"

echo Menghentikan seluruh container...
docker compose down

echo.
echo ========================================================
echo    Aplikasi WaterWatch Berhasil Dimatikan.
echo ========================================================
echo.
pause

