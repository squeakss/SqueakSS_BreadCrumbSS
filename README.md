The goal of breadcrumbs is to streamline IP and Domain enumeration by automating the use of available OSINT tools while avoiding any Pay to Play API's that might get in the way.

The Talos reference function is nearly complete. Soon all desired data will be extracted from the html correctly. I'll then be adding functions for ipinfo.io, abuse.ch's URLHAUS, and more.

USAGE:

Download the Chromedriver that matches your version of Chrome.

Start the Chromedriver before starting BreadCrumbss and be sure that no instance of Chrome are currently running.

If you don't currently have a profile on Chrome, then you will need to make one to avoid an endless loop of captchas.

On line 35 'let profile_path = "";' insert the path to your profile inside of the double quotes. C:/Users/####/AppData/Local/Google/Chrome/User Data is the standard profile path for Windows, ~/.config/google-chrome/ for Linux, and ~/Library/Application Support/Google/Chrome/ for MAC.


Consider using a burner account...
This is a scraping tool...
