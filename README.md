### Pong

A Pong-clone developed in [Bevy](https://bevyengine.org/) game engine.

Run the game by cloning this repo and running `cargo run`.

Requires [Rust](https://www.rust-lang.org/) to be installed.

Player 1 controls: W, S
Player 2 controls: Up, Down arrows

Esc pauses the game, and the game can also be restarted from there

### TODO:
 * ~~Make the ball go faster as the round progresses~~
   * Done'd, every paddle collision accelerates the ball. 
 * Create 'sections' for the paddles, so that the ball bounces differently when hitting different sections
 * ~~Add a menu screen and the ability to pause, restart and quit the game~~
   * ~~Game starts at menu screen, can be paused/continued and quit. Still needs restart.~~
      * Now starts from menu screen, can be paused/continued, restarted and quit.
 * Some goal (10 points?) for which to play, then show splashscreen for the winner and begin new game
 * Add a settings menu in which the paddle and ball colors can be changed
 * Change paddles and ball to use sprite assets and allow users to apply custom skins
 * Add powerups
 * Other unimportant and over-the-top-extra fun stuff.. :)