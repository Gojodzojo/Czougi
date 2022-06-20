# Czougi
Czougi is a set of of projects implementing the same game in different programming languages and using different renderign tehniques (OpenGL, HTML5 canvas, terminal ui, etc). The game is a new version of other [Czougi](https://openprocessing.org/sketch/799340) game which has been inspired by an old game [Battle City](https://en.wikipedia.org/wiki/Battle_City). The goal is to make a multiplayer game that can be played both locally and online across multiple platforms via websockets. The game should also contain a client-side editor that allows the user to create maps and save them to a json file.

## Game assumptions
In order to make the game playable across multiple platforms, the following assumptions are made:

* Gameplay: each player is controlling a single tank. The tank can move and shoot. A player can score points by destroying other tanks. The game ends when one player scores certain amount of points.
* Map size: 50x50
* Players number: 2-4 players can play at the same time
* Tank size: 2x2
* Tank colors: red, green, blue, yellow
* Tiles: there are 4 types of tiles. Each of them has four variants (upper left, upper right, lower left, lower right). Each of tile types has different behaviour.

### Tile types
* Brick: the tank cannot move through it. It can be destroyed by a tank shot.
* Concrete: the tank cannot move through it. It cannot be destroyed.
* Water: the tank cannot move through it, but the bullets can.
* Leaves: the tank can move through them and it can hide behind them.

### Server api
TODO