# image-ffi
Image processing plugin system


## Plugins
### Mirror
#### command example
```
cargo run -- --input cow.png --output new_cow.png --plugin mirror --params mirror_params.json
```
#### params.json example
```
{ "mode": "horizontal" } 
```

### Blur
#### command example
```
cargo run -- --input cow.png --output new_cow.png --plugin blur --params blur_params.json
```
#### params.json example
```
{ "radius": 4, "iterations": 4 }
```
