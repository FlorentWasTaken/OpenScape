# OpenScape

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li><a href="#how-to-build">How to build</a></li>
    <li><a href="#dependencies">Dependencies</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li>
      <a href="#how-to-use-lua">How to use LUA</a>
      <ul>
        <li><a href="#Examples">Examples</a></li>
      </ul>
    </li>
  </ol>
</details>

## About the project

This small project is a simple block game engine. It's used to make some test with Rust and SDL2.
It also provides a LUA scripting system (used to move camera [Z, Q, S, D] or to place and destroy blocks [mouse]).
This projet can be used to learn basics of scripting.

### Built with

- ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
- ![OpenGL](https://img.shields.io/badge/OpenGL-%23FFFFFF.svg?style=for-the-badge&logo=opengl) (SDL2)
- ![Lua](https://img.shields.io/badge/lua-%232C2D72.svg?style=for-the-badge&logo=lua&logoColor=white)

## How to build

It's higly recommended to use Docker because it will be more easier to get every dependencies.
The `Dockerfile` is only configured for Linux (for now).

- If you want to build it with Docker :

```
git clone git@github.com:FlorentWasTaken/OpenScape.git
cd OpenScape
sudo docker build -t open_scape .
xhost +
sudo docker run -e DISPLAY=$DISPLAY -v /tmp/.X11-unix:/tmp/.X11-unix open_scape
```

- If you already have every dependencies (see dependencies)

```
git clone git@github.com:FlorentWasTaken/OpenScape.git
cd OpenScape
cargo run
```

## Dependencies

- Rust
- SDL2 | SDL2_image | SDL2_ttf
- OpenGl

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contributing

I you want to contribute to the project, please open an issue, I'll be happy to discuss with you !

---

## How to use LUA

If you want to create lua script you'll need to include your main function into `src/script/main.lua`
You'll also need to include `src/script/include.lua`, this file contains every functions calling Rust.
Here is all available LUA functions:

- place_block(x, y): nil | Used to place a block at x, y
- destroy_block(x, y): nil | Used to destroy a block at x, y
- get_cam_pos(): number, number | Used to get camera pos as x, y
- set_cam_pos(x, y): nil | Used to set camera pos at x, y

There is also a lua lib in `src/script/lib.lua`. It contains :

- openScape.readFile(path): string | Used to get a file content
- openScape.writeFile(path, content): bool | Used to write in / create a file
- openScape.strToWordArray(str): table | Used to transform a string into an array (table)
- openScape.wordArrayToStr(lines): string | Used to transform an array(table) into a string

### Examples

If you want some examples of the purpose of the Lua scripting system, see
`src/script/main.lua`.

![first example]("./images/example1.png")
