# ttdl-lunar-calendar

A lunar calendar plugin for [TTDL](https://github.com/VladimirMarkelov/ttdl).

## Usage

Add the `!lunar-calendar` special tag with a comma separated list, an item of
the list can be one of the following:

- A special tag prefixed with `#` (e.g. `#due` for "due")
- An optional (e.g. `created` for "Creation Date")

> Tips:
>
> - For definitions of "Special Tags" or "Optional", please see
>   [todo.txt format](https://github.com/todotxt/todo.txt) for details.
> - For optional names from [TTDL](https://github.com/VladimirMarkelov/ttdl),
>   please see [Plugin interaction](https://github.com/VladimirMarkelov/ttdl#plugin-interaction)
>   for details.

For example, the item
`2000-01-01 example due:2000-01-01 !lunar-calendar:created,#due` will be
converted to `2000-02-05 example due:2000-02-05 !lunar-calendar:created,#due`.

> Notes:
>
> - The lunar date format is `YYYY-MM-DD`
>   (4 year digits - 2 month digits - 2 day digits), that is same with the TTDL
>   date format for now, and we will continue to support it in the future.
> - TTDL support plugins when executing command `list` only for now.

## Installation

Download the pre-compiled binaries from the
[release page](https://github.com/eugnma/ttdl-lunar-calendar/releases), and move
it to the paths from the
[PATH environment variable](<https://en.wikipedia.org/wiki/PATH_(variable)>).

## Code of Conduct

Help us keep this project open and inclusive. Please read and follow our
[Code of Conduct](CODE_OF_CONDUCT.md).

## Contributing

We welcome all people who want to contribute. Please see the
[contributing guidelines](CONTRIBUTING.md) for more information.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## License

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

- [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Related projects

- [The todo.txt format](https://github.com/todotxt/todo.txt)
- [TTDL](https://github.com/VladimirMarkelov/ttdl)
