@echo off
:: ========================================================
:: JARVIS 2.0 - PRODUCTION DEPLOYMENT SCRIPT
:: SINGLE WINDOW REAL-WORLD AUTOMATION ONLY
:: ========================================================

echo 🤖 JARVIS 2.0 - MONOLITH CORE DEPLOYMENT
echo =============================================
echo NO MOCK DATA - PRODUCTION READY ONLY

:: Validate environment
echo 🔍 Validating production environment...
if not exist "AgentConfig.toml" (
    echo ❌ ERROR: AgentConfig.toml not found
    echo Please create AgentConfig.toml with real configuration
    pause
    exit /b 1
)

:: Check for required files in AgentConfig.toml
echo 🔍 Checking resume and cover letter paths in configuration...
findstr /C:"resume_path" AgentConfig.toml > nul
if %ERRORLEVEL% neq 0 (
    echo ❌ ERROR: resume_path not found in AgentConfig.toml
    echo Please update AgentConfig.toml with the correct resume path from AgentSecrets.template
    pause
    exit /b 1
)

findstr /C:"cover_letter_path" AgentConfig.toml > nul
if %ERRORLEVEL% neq 0 (
    echo ❌ ERROR: cover_letter_path not found in AgentConfig.toml
    echo Please update AgentConfig.toml with the correct cover letter path from AgentSecrets.template
    pause
    exit /b 1
)

echo ✅ Resume and cover letter paths found in configuration

:: Create production directories
echo 📁 Creating production directories...
if not exist "state" mkdir state
if not exist "logs" mkdir logs
if not exist "screenshots" mkdir screenshots

:: Build PRODUCTION binary ONLY
echo 🔨 Building JARVIS 2.0 monolith core...
cargo build --release --features visible-chrome

if %ERRORLEVEL% neq 0 (
    echo ❌ Build failed! Check compilation errors.
    pause
    exit /b 1
)

echo ✅ JARVIS 2.0 build completed successfully!

:: Create production startup script
echo 🚀 Creating production startup script...
echo @echo off > start_jarvis.bat
echo echo 🤖 JARVIS 2.0 - MONOLITH CORE STARTING... >> start_jarvis.bat
echo echo Single window automation - NO MOCK DATA >> start_jarvis.bat
echo echo Controls: Ctrl+P (Pause) ^| Ctrl+R (Resume) ^| Ctrl+Q (Quit) >> start_jarvis.bat
echo echo. >> start_jarvis.bat
echo .\target\release\autonomous_job_agent.exe >> start_jarvis.bat
echo pause >> start_jarvis.bat

:: Create environment setup helper
echo 📝 Creating environment setup helper...
echo @echo off > setup_environment.bat
echo echo 🔧 JARVIS 2.0 - Environment Setup >> setup_environment.bat
echo echo. >> setup_environment.bat
echo echo Please set the following environment variables: >> setup_environment.bat
echo echo. >> setup_environment.bat
echo echo Required: >> setup_environment.bat
echo echo   set AGENT_NAME="Your Full Name" >> setup_environment.bat
echo echo   set AGENT_EMAIL="your.email@example.com" >> setup_environment.bat
echo echo   set RESUME_PATH="C:\path\to\your\resume.pdf" >> setup_environment.bat
echo echo   set COVER_LETTER_PATH="C:\path\to\your\cover_letter.pdf" >> setup_environment.bat
echo echo. >> setup_environment.bat
echo echo Optional (for account creation): >> setup_environment.bat
echo echo   set LINKEDIN_EMAIL="your.linkedin@email.com" >> setup_environment.bat
echo echo   set LINKEDIN_PASSWORD="your_password" >> setup_environment.bat
echo echo   set INDEED_EMAIL="your.indeed@email.com" >> setup_environment.bat
echo echo   set INDEED_PASSWORD="your_password" >> setup_environment.bat
echo echo. >> setup_environment.bat
echo pause >> setup_environment.bat

echo.
echo 🎯 JARVIS 2.0 DEPLOYMENT COMPLETE!
echo.
echo 📋 PRODUCTION CHECKLIST:
echo   1. ✅ Binary compiled successfully
echo   2. 📝 Update AgentConfig.toml with your real information
echo   3. 📄 Ensure your resume and cover letter paths in AgentConfig.toml are correct
echo   4. 🔧 Run setup_environment.bat to configure credentials
echo   5. 🚀 Run start_jarvis.bat to begin automation
echo.
echo 🎮 OPERATION MODES:
echo   • Single Window: Visual browser automation
echo   • Real-Time Monitoring: Console + log files
echo   • Manual Intervention: Pause for login/MFA
echo   • Rate Limited: Respectful job board interaction
echo.
echo ⚠️  PRODUCTION REQUIREMENTS:
echo   • Display environment (Windows desktop)
echo   • Real resume and cover letter files at the paths specified in AgentConfig.toml
echo   • Valid email addresses and credentials
echo   • NO MOCK DATA - Real job applications only
echo.
echo 🚀 Ready to deploy: start_jarvis.bat
echo.
pause