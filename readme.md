# Waybar Athan widget

waybar_athan is waybar widget to show current/upcomming salah past/remaining time.

## Installaion

```sh
git clone https://github.com/rynidja/waybar_athan
cd waybar_athan
cargo install --path .
```

Add this to your waybar config (make sure to substitut your latitude and longitude):
```jsonc
"custom/athan": {
    "format": "󱠧  {}",
    // for more options run `waybar_athan --help`
    "exec": "~/.carog/bin/waybar_athan -l <latitude> -L <longitude>",
    "return-type": "json",
    "interval": 60,
}
```

## Notifications

To get notifications use a wrapper shell script like the one in `notifiyer` 

Todo
----

- Add `other` calculation method
- Add athan time adjustments ...

Bugs Tracking
----

Feel free to report bugs and issues on the [Issues tab][issues].

Contribute
----

Feel free to fork the project and open pull requests.

License
----

MIT
