# Move Fast

"Move Fast" is a puzzle game where the shortest path isn't always the best option.
Made with [Bevy](https://github.com/bevyengine/bevy).

The aim of the game is simple: reach the goal tile.
During your journey to the goal tile, you will encounter enemies who move towards you every time you move.
Your job is to safely avoid these obstructions by considering your every move.

The idea behind this game is that it is purely _deterministic_.
In other words, the consequence of a certain sequence of moves will always be the same.
No randomness. No time-based mechanics. Just you and your keyboard.
So in all honesty, the name "Move Fast" is really a misnomer.

![game](https://github.com/claby2/move-fast/blob/preview/game.png)

## Custom levels

The game comes with preinstalled levels located in the directory [assets/levels](assets/levels).
Each level is a [CSV file](https://en.wikipedia.org/wiki/Comma-separated_values) where each value represents some tile.
You can easily create your own levels by adding more CSV files to the directory.

Example CSV level file:

    0,0,0,0,0,0,0,0,3
    0,0,0,0,0,0,0,0,0
    0,0,0,0,0,1,1,1,1
    0,0,0,0,0,0,0,0,0
    0,0,0,0,0,0,0,0,0
    0,0,0,0,0,0,0,0,0
    0,0,0,0,0,0,0,0,0
    0,0,0,0,0,0,0,0,0
    2,0,0,0,0,0,0,0,0

| Value | Tile  |
| ----- | ----- |
| 0     | Empty |
| 1     | Block |
| 2     | Start |
| 3     | Goal  |
| 4     | Enemy |

### Caveat

The number of columns **must** equal to the number of rows.
If a level is loaded without meeting this condition, the game will throw an error.

## Controls

| Key               | Function            |
| ----------------- | ------------------- |
| <kbd>W</kbd>      | Move upwards        |
| <kbd>A</kbd>      | Move leftwards      |
| <kbd>S</kbd>      | Move downwards      |
| <kbd>D</kbd>      | Move rightwards     |
| <kbd>Escape</kbd> | Return to main menu |

## UI Preview

![menu](https://github.com/claby2/move-fast/blob/preview/menu.png)
![level](https://github.com/claby2/move-fast/blob/preview/level.png)
