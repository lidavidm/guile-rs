# guile-rs

Safe Rust bindings to [GNU Guile][gnu_guile].

## Status

I took the maintainer role of this project back in 2017.

I've been working on trying to get the library in better shape, as I
wish to use it in my text editor as an embedded scripting/
configuration language.

However, it has proven to be rather.. tricky. GNU Guile has a really
_weird_ way of exposing its API. I have yet to cleanly bindto it.

I've put it on hold from my side for now, but if anyone has any
thoughts on improvements -- do let me know! I want this crate to
succeed, its just tricky with such a complex project such as Guile..

There are other attempts to bring Guile into the Rust ecosystem, and
it would be great to unify these attempts into one implementation.

## Community

The IRC channel is no more, as Mozilla switched to Matrix.

I am looking into a different way to communicate, possibly Zulip.

[gnu_guile]: https://www.gnu.org/software/guile/
