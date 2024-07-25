## Why so many connections?

The goal of breadcrumbs is to enumerate IP's of all connections to your localhost by automating the use of available OSINT tools. **SCRAPING WILL BE PERFORMED** to avoid pay to play API's.

The Talos and ipinfo references are complete. I'll be adding functions for abuse.ch's URLHAUS and more.

USAGE:

This is currently only built to function on Windows.

Download the Chromedriver that matches your version of Chrome.

Start the Chromedriver before starting BreadCrumbss and be sure that no other instance of Chrome is currently running.

If you don't have a profile on Chrome, then you will need to make one to avoid an endless loop of captchas.

On line 195 'let profile_path = "";' insert the path to your profile inside of the double quotes. C:/Users/####/AppData/Local/Google/Chrome/User Data is the standard profile path for Windows. 

(~/.config/google-chrome/ for Linux, and ~/Library/Application Support/Google/Chrome/ for MAC. Good Luck)

**Some scraping will take place.**

Make sure the file structure matches this repo. Execute "runit.bat" add it to a crontab if you please.

