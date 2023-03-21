# Open Window

## [https://szmergiell.github.io/open-window/](https://szmergiell.github.io/open-window/)

The most important part of this repository is a library called `owlib`, which answers a question whether one should open windows in order to decrease indoor humidity.

Question is answered by comparing indoor and outdoor dew points calculated based on indoor/outdoor temperature and relative humidity measurements.

Basically, if outdoor dew point is lower than indoor dew point, you should open windows.

## Projects

All projects and majority of the code base (80%+) are written in Rust.

### owlib

Library responsible for answering the main question - whether one should open windows in order to decrease indoor humidity.

```
cargo test -p owlib
```

### owcli

Command line interface for the library, written with Clap.

```
cargo run -p owcli -- --help
```

```
cargo run -p owcli -- 18 50 0 85
```

### owserver

REST API (written with Axum), which exposes the functionality of the library. It is designed to be called by [https://github.com/szmergiell/open-window-web](https://github.com/szmergiell/open-window-web) Svelte web app.

```
cargo run -p owserver
```

```
curl -i -X POST localhost:3000/open-window -H 'Content-Type: application/json' -d '{ "indoor_measurement": { "temperature": 18.0, "relative_humidity": 50 }, "outdoor_measurement": { "temperature": 0.0, "relative_humidity": 85 }}'
```

### owapp

Purely client-side WebAssembly [web application](https://szmergiell.github.io/open-window/), with some PWA features (offline, installation, local storage), written in Yew.

Serve for development:

```
trunk serve owapp/index.html --open
```

Build for production:

```
trunk --config owapp/Trunk.toml build
```

## Inspiration

I've an electronic weather station, which measures indoor/outdoor relative humidity and temperature. At one point in time I've realized that comparing just indoor/outdoor relative humidity is not enough to definitely answer a question whether one should open windows in order to decrease indoor humidity. The other part of the equation is temperature. At that time I didn't have any intuition or a rule of thumb how to reliably compare the measurements on the fly, so I've started researching the topic.

## Motivation

I realize there are many relative humidity converters available on the Internet. But for me, it is just an excuse to turn this relatively simple problem into Rust learning opportunity.

_So please don't judge the code too harshly :) At my day job I'm a .NET developer with "some" experience (started in 2015), so I am aware of some of the code quality issues in this repo._

That being said, if someone, somehow, randomly stumbles upon this repository and finds it useful, I'm open for suggestions.
