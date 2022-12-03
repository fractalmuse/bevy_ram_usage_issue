# bevy ram usage issue

Minimum reproduction of high RAM usage on a Macbook Pro with M1 Max.

The project simply loads ~20MB of PNG assets and two fonts from the `assets` folder,
then displays some text in one of the fonts against a plain background.

With `cargo run --release --features all_assets`  this uses ~2.8GB of RAM as reported by Activity Monitor:

![RAM usage with all assets loaded](screenshots/ram_usage_full.png)

![Info on RAM usage with all assets loaded](screenshots/ram_usage_full_info.png)

With `cargo run --release --features only_font` (which only loads the font used to display text) it uses ~200MB:

![RAM usage with only font loaded](screenshots/ram_usage_font.png)

![Info on RAM usage with only font loaded](screenshots/ram_usage_font_info.png)
