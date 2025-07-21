@echo off
:: ========================================================
:: JARVIS 2.0 - PRODUCTION LAUNCHER
:: FULL VISIBLE BROWSER AUTOMATION - REAL JOB APPLICATIONS
:: ========================================================

echo 🤖 JARVIS 2.0 - PRODUCTION JOB APPLICATION AGENT
echo ================================================
echo ✅ PRODUCTION READY - VISIBLE BROWSER AUTOMATION

:: Validate production environment
echo 🔍 Validating production environment...
if not exist "AgentConfig.toml" (
    echo ❌ ERROR: AgentConfig.toml not found
    echo Production configuration required
    pause
    exit /b 1
)

if not exist ".env" (
    echo ❌ ERROR: .env file not found
    echo Production environment variables required
    pause
    exit /b 1
)

if not exist "documents\resume.pdf" (
    echo ❌ ERROR: Resume file not found
    echo Please ensure documents\resume.pdf exists
    pause
    exit /b 1
)

if not exist "documents\cover_letter.pdf" (
    echo ❌ ERROR: Cover letter file not found
    echo Please ensure documents\cover_letter.pdf exists
    pause
    exit /b 1
)

:: Create production directories
if not exist "state\" mkdir state
if not exist "logs\" mkdir logs
if not exist "screenshots\" mkdir screenshots

echo ✅ Environment validation complete - PRODUCTION SYSTEM READY

:: Display production configuration
echo.
echo 🎯 PRODUCTION CONFIGURATION:
echo   • Agent: John Doe (john.doe@email.com)
echo   • Mode: Visible Browser Automation
echo   • Target: 13 Real Job Boards
echo   • Rate: 20 applications/hour (MAX 75/day)
echo   • Resume: documents/resume.pdf
echo   • Cover Letter: documents/cover_letter.pdf
echo.

:: Display operational modes
echo 🚀 DEPLOYMENT OPTIONS:
echo.
echo   1. CLI Mode (Recommended):    .\target\release\autonomous_job_agent.exe
echo   2. GUI Mode:                  .\target\release\production_gui.exe
echo   3. Web Server:                .\target\release\web_server.exe
echo.

:: Launch production system
echo 🎮 CONTROLS DURING OPERATION:
echo   • Ctrl+P: Pause automation
echo   • Ctrl+R: Resume automation
echo   • Ctrl+Q: Quit application
echo   • Manual intervention: Agent pauses for login/MFA automatically
echo.

echo 🚀 Launching PRODUCTION Job Application Agent...
echo ✅ Browser will open in VISIBLE mode for real-time monitoring
echo ⚠️  Agent will apply to REAL jobs on REAL job boards
echo.

:: Start the production system
.\target\release\autonomous_job_agent.exe

echo.
echo 🔴 Production session ended
echo 📋 Check logs\ directory for session reports
pause
