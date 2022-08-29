## OpenGL Youtube Tutorial Codealong in Rust

This is a codealong for [The Cherno - OpenGL](https://youtube.com/playlist?list=PLlrATfBNZ98foTJPJ_Ev03o2oq3-GGOS2) youtube series adapted for Rust.

### Progress

#### Lessons 1 - 3

This is mostly setup and the details and is drastically different between C++ and Rust. But the general steps are the same. Namely to install gl frameworks (I will be using gl-rs + glfw-rs crates). In rust we need to install these crates (see Cargo.toml).

For `glfw-rs` we have to provide it with glfw.lib C++ installation. There are several ways to do this mentioned in [glfw-rs](https://github.com/PistonDevelopers/glfw-rs).
I am on windows and the one that worked for me was to install [CMake](https://cmake.org/download/) and have the glfw-rs crate compile glfw automatically.

See also [learn-opengl-rs](https://github.com/bwasty/learn-opengl-rs)

One more note `gl-rs` doesn't seem to support glBegin() glEnd() legacy style (cmiiw) so we won't be able to draw triangle with those commands at this point of the lesson.
