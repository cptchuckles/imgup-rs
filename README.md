# imgup-rs

This is a Rust CLI tool for pushing image files up to imgur ~~and copying the URL to the system
clipboard~~.

## Installation

You need to create an API Client-ID for Imgur: follow their developer instructions
[here](https://apidocs.imgur.com/).  It should be a long number.  Once you have this, save it to a
file **directly adjacent to the executable** and name it `imgup.secret`.  `imgup` will use this
file's contents as your API Client-ID authorization header.  You can't use mine because you'll
upload some dumb bullshit and get my client-id banned.

## Example

Usage:
```bash
$ imgup ~/pictures/file.jpg
```

Output:
```console
https://i.imgur.com/rnSZuQH.jpg
Delete: <hash>
```

## Notes

- The image url is printed to `stdout` and the delete hash is printed to `stderr`, so that simply
  piping the output of `imgup` to something will include only the url and won't accidentally
  include the delete-hash.  You can use redirection `2>&1` to pipe both the url and delete hash
  somewhere.
- Currently does not support copying URL to clipboard by itself.  I probably won't implement that
  anyway because it can be platform-specific, and also I am lazy and you should learn how to do
  things for yourself you crybaby little shit.

### btw

here is an example of a bash snippet that automatically copies the url to clipboard using `xclip`:
```bash
{ IFS= read -r url
  IFS= read -r del
} < <(imgup "$imagefile" 2>&1)

xclip -sel c -i <<<"$url"

notify-send -t 60000 "Image uploaded" "$url\nDel: $del"
```
