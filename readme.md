# TxTs Treasury
This project was created to manage my own finances. Hopefully it is useful for you as well!
At this time TxTs Treasury is still in very early development, despite that I am already using it myself, so it is definitely usable!

# Installation
The only supported way to install and run TxTs Treasury is Docker. For convenience I provide a docker-compose.yml file that makes it really easy to get started.
I personally run it using [Unraids](https://unraid.net/) built in Docker feature by manually specifying the images and envoirnment variables which also works well enough.

# Configuration
Configuration is handled through envoirnment variables.
Important ones to change are:

`POSTGRES_PASSWORD` this needs to be the same for the backend and postgres services  
`ADMIN_PASSWORD` this will be the password you log in with. Right now you can only use the admin account, however multi-account support will be added later on. This variable is only relevant on first boot. Afterwards you can change it through the settings menu.  
`PEPPER` this is somewhat important for securily encrypting passwords for the database. Just roll your face across your keyboard.

