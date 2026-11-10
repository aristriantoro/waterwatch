@echo off
title WaterWatch - Smart Water Complaint System
chcp 65001 >nul
cls

echo ========================================================
echo    Menjalankan Aplikasi WaterWatch (Provinsi Lampung)
echo ========================================================
echo.

cd /d "%~dp0"

echo [1/3] Menyalakan container Docker (Database, Backend, Web Server)...
docker compose up -d

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Gagal menyalakan Docker! 
    echo Pastikan Docker Desktop sudah dibuka dan berstatus 'Engine running'.
    echo.
    pause
    exit /b %errorlevel%
)

echo.
echo [2/3] Menunggu server siap...
timeout /t 3 /nobreak >nul

echo [3/3] Membuka browser ke http://localhost...
start http://localhost

echo.
echo ========================================================
echo    Aplikasi Berhasil Dijalankan!
echo ========================================================
echo.
echo Alamat Web : http://localhost
echo.
echo Akun Login Pengujian:
echo  - Admin : admin@waterwatch.id / Admin@1234!
echo  - Warga : warga1@example.com  / Warga@1234!
echo.
echo Untuk mematikan aplikasi, jalankan stop-waterwatch.bat
echo ========================================================
echo.
pause

