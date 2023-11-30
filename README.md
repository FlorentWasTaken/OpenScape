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
  </ol>
</details>

## About the project

This small project is a simple block game engine. It's used to make some test with Rust and SDL2.
It also provides a LUA scripting system (used to move camera [Z, Q, S, D] or to place and destroy blocks [mouse]).

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
