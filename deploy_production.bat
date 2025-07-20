@echo off
:: ========================================================
:: PRODUCTION DEPLOYMENT SCRIPT - WINDOWS
:: ========================================================

echo 🚀 AI Job Agent - Production Deployment System
echo ==============================================

:: Create necessary directories
echo 📁 Creating production directories...
if not exist "state" mkdir state
if not exist "logs" mkdir logs
if not exist "screenshots" mkdir screenshots
if not exist "config" mkdir config
if not exist "data" mkdir data

:: Build all production binaries
echo 🔨 Building production binaries...
cargo build --release --features production-gui,web-server,visible-chrome

if %ERRORLEVEL% neq 0 (
    echo ❌ Build failed! Check compilation errors.
    pause
    exit /b 1
)

echo ✅ Build completed successfully!

:: Create desktop shortcuts
echo 🖥️ Creating desktop shortcuts...
echo @echo off > start_gui.bat
echo echo 🎮 Starting AI Job Agent GUI... >> start_gui.bat
echo .\target\release\autonomous_job_agent.exe --gui >> start_gui.bat
echo pause >> start_gui.bat

echo @echo off > start_web.bat
echo echo 🌐 Starting AI Job Agent Web Server... >> start_web.bat
echo echo Dashboard: http://localhost:8080 >> start_web.bat
echo .\target\release\autonomous_job_agent.exe --web >> start_web.bat
echo pause >> start_web.bat

echo @echo off > start_hybrid.bat
echo echo 🚀 Starting AI Job Agent Hybrid Mode... >> start_hybrid.bat
echo echo GUI + Web + CLI automation active >> start_hybrid.bat
echo .\target\release\autonomous_job_agent.exe >> start_hybrid.bat
echo pause >> start_hybrid.bat

echo.
echo 🎯 Production system ready! Deployment options:
echo.
echo 1. 🎮 GUI Mode:     Double-click start_gui.bat
echo 2. 🌐 Web Mode:     Double-click start_web.bat
echo 3. 🚀 Hybrid Mode:  Double-click start_hybrid.bat
echo 4. ⚡ CLI Mode:     .\target\release\autonomous_job_agent.exe --cli
echo.
echo 📋 Direct executables:
echo    • GUI Only:      .\target\release\production_gui.exe
echo    • Web Only:      .\target\release\web_server.exe
echo    • Main App:      .\target\release\autonomous_job_agent.exe
echo.
echo 🔧 Configuration:
echo    • Edit AgentConfig.toml for settings
echo    • Copy AgentSecrets.template to .env and add credentials
echo.
echo ✅ Production deployment complete!
echo.
pause
