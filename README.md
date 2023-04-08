# shfonts
[![CI](https://github.com/asartalo/shfonts/actions/workflows/ci.yml/badge.svg)](https://github.com/asartalo/shfonts/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/asartalo/shfonts/badge.svg?branch=main)](https://coveralls.io/github/asartalo/shfonts?branch=main)

Command line tool to download fonts from third-party font hosting sites for
self-hosting.

## Installation

Coming soon...

## Usage

For basic usage, pass in the URL of the CSS font declaration of the hosting
site:

```sh
shfonts "https://fonts.googleapis.com/css?family=Roboto:300"
```

Assuming that the URL passed is a CSS file with `@font-face` declarations, the
previous command will download all the font files specified inside it. It will
also write a CSS file with contents copied from that CSS file replacing the font
URLs declared in `@font-face > src` `url()` with the file names of the font
files. For example, if the content of the CSS file above is the following:

```css
/* latin */
@font-face {
  font-family: 'Roboto';
  font-style: italic;
  font-weight: 300;
  src: url(https://fonts.gstatic.com/s/roboto/v30/KFOjCnqEu92Fr1Mu51TjASc6CsQ.woff2) format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}
```

...it will rewrite it to something like the following:

```css
/* latin */
@font-face {
  font-family: 'Roboto';
  font-style: italic;
  font-weight: 300;
  src: url(KFOjCnqEu92Fr1Mu51TjASc6CsQ.woff2) format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}
```

### Setting a Download Directory

If you want to save the font files and the CSS file to a different directory,
use `--dir` or `-d` option:

```sh
shfonts -d "/download/path" "https://fonts.googleapis.com/css?family=Roboto:300"
```

### Customizing the Font URL Path

If you need to customize the font URL inside the `url()` declaration, use the
`--font-url-prefix` or `-p` option:


```sh
shfonts -p "/assets/fonts/" "https://fonts.googleapis.com/css?family=Roboto:300"
```

For the previous example CSS file, it will rewrite it like in the following:

```css
/* latin */
@font-face {
  font-family: 'Roboto';
  font-style: italic;
  font-weight: 300;
  src: url(/assets/fonts/KFOjCnqEu92Fr1Mu51TjASc6CsQ.woff2) format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}
```
