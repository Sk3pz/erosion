# Erosion

Erosion is a 2D game / game engine developed in rust that simulates every pixel and has different types of materials that interact with the world in different ways.

## Material Types
* Sand - falls if nothing is below it
* Water - flows and fills the container it is given
* Static / Stationary - If connected to something, wont move. (I.E. wood, stone, etc)
* Gas
  - Lighter than Air: Will float up
  - Same as Air: Will not move unless pushed by something else (i.e. other gas particles, sand, water, etc)
  - Heavier than Air: Will sink down
* Acid - Flows like water, will corrode anything it touches
* Lava - Flows slower than water, will ignite anything it touches
