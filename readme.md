## RUST WEBSERVER

# Config
Config file: ~/.rustws/config
Each line of the config file defines a server route containing two tokens: the route and the filepath of the file to serve.
The filepath should be absolute, but defining a "ROOT" route will set a root directory from which to start looking for files.

Ex:
ROOT ~/webserver
/ templates/hello.html
/aboutme templates/aboutme.html