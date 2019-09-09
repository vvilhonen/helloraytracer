## Ray tracer from <i>Ray Tracing in One Weekend</i> from Peter Shirley

![demo](https://vesacdn.s3.eu-north-1.amazonaws.com/demo2.gif)

### Running

#### With a window animated

        $ cargo run --release --bin window

#### With a single output image generated to `out.png`
        
        $ cargo run --release --bin png -- out.png
        
#### With the above gif generated to `demo.gif`

        $ cargo run --release --bin gif -- demo.gif
