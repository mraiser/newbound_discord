# newbound_discord
Select any Newbound Metabot Control to act as a Discord bot. Once connected to your Discord server, all the Commands in the Control's API become commands you can invoke from Discord. The Newbound Discord app will listen for messages that start with an exclamation point (!) and call the Command named with whatever follows the exclamation point. Parameters to the Command follow the command name with spaces between them.

# Dependencies
1. This project requires an up-to-date working installation of the Newbound software
   https://github.com/mraiser/newbound

2. Discord4J and dependencies
   https://search.maven.org/artifact/com.discord4j/discord4j-core/3.1.2/jar

# Installation
1. Move the data/discord and runtime/discord folders into your Newbound installation's data and runtime folders, respectively
2. Download the Discord4J jar files and their dependencies and drop them in your Newbound installation's "lib" folder
3. Launch the Newbound software
4. Publish the "discord" control in the "discord" library using the Metabot app
5. Restart the Newbound software

*Instead of moving the data/discord and runtime/discord folders you can create symbolic links to them, leaving your git project folder intact for easy updating*
