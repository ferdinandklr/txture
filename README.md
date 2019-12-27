Ever wanted to simply generate tileable noise textures in rust ? You are at the right place ! This crate allows you to generate perlin noise texture and many more with ease.

# Installing
Simply add the lib to your Cargo.toml to start playing :
```
[dependencies]
txture = "*"
```
To learn more about this library, go check on [crates.io](https://crates.io/crates/txture) or on [github](https://github.com/ferdinandklr/txture) !

# How to use
***Please note that some use cases of this lib can be seen in the [test](https://google.com) folder !***

## A simple perlin noise texture
Start by importing the crate where you need it :

```rust
use txture::PerlinNoise;
```
Then create a new `PerlinNoise` instance, precising :
1) The size of the picture as `u32`
2) The number of gradient points you want as `u8` _(check pictures for a better understanding)_
3) whether the picture should be tilable or not as `bool`

The output is of type `Result<PerlinNoise, txture::Error>`, so you have to unwrap it. You cant try for exemple :
```rust
let perlin_noise = PerlinNoise::new(400, 5, true).unwrap();
```

Then,  to access any pixel in the picture, all you have to do is write :
```rust
let gray: u8 = perlin_noise.get_pixel_value(i, j);
```

# Author
- **Ferdinand Keller** - feel free to explore my github repos
# License
This project is under the MIT license _(see the [LICENSE.md](https://github.com/ferdinandklr/txture/LICENSE.md) for more details)_
