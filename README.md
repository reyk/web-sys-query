# jQuery-like API for web-sys

[![Build Status](https://travis-ci.org/reyk/web-sys-query.svg?branch=main)](https://travis-ci.org/reyk/web-sys-query)
[![License](https://img.shields.io/badge/license-ISC-blue.svg)](https://raw.githubusercontent.com/reyk/web-sys-query/main/LICENSE)

_jQuery is dead_ and everyone agrees that you should use a modern
approach like React instead (or, in the Rust/WASM world,
[Yew](https://crates.io/crates/yew)).

`web-sys-query` allows you to port simple jQuery applications to
Rust/WASM with a familiar API instead of the rather complex
[web-sys](https://crates.io/crates/web-sys) DOM API.

## TODO

- Documentation and examples
- Unit tests
- AJAX
- ...

## API

### Attributes

| jQuery: Attributes   | `Document`      | `Element` +  `Collection`      |
| -------------------- | --------------- | ------------------------------ |
| .addClass()          |                 | add_class                      |
| .attr()              |                 | attr, set_attr                 |
| .hasClass()          |                 | has_class                      |
| .html()              |                 | html, set_html                 |
| .prop()              |                 |                                |
| .removeAttr()        |                 | remove_attr                    |
| .removeClass()       |                 | remove_class                   |
| .removeProp()        |                 |                                |
| .toggleClass()       |                 | toggle_class                   |
| .val()               |                 | val, set_val, *_i32, *_f64     |

### Manipulation

| jQuery: Traversing   | `Document`      | `Element` + `Collection`       |
| -------------------- | --------------- | ------------------------------ |
| .after()             |                 |                                |
| .append()            |                 |                                |
| .appendTo()          |                 |                                |
| .before()            |                 |                                |
| .clone()             |                 |                                |
| .css()               |                 |                                |
| .css()               |                 |                                |
| .detach()            |                 |                                |
| .empty()             |                 |                                |
| .height()            |                 |                                |
| .innerHeight()       |                 |                                |
| .innerWidth()        |                 |                                |
| .insertAfter()       |                 |                                |
| .insertBefore()      |                 |                                |
| $.cssNumber          |                 |                                |
| $.htmlPrefilter()    |                 |                                |
| .offset()            |                 |                                |
| .outerHeight()       |                 |                                |
| .outerWidth()        |                 |                                |
| .position()          |                 |                                |
| .prepend()           |                 |                                |
| .prependTo()         |                 |                                |
| .remove()            |                 |                                |
| .replaceAll()        |                 |                                |
| .replaceWith()       |                 |                                |
| .scrollLeft()        |                 |                                |
| .scrollTop()         |                 |                                |
| .text()              |                 | text, set_text                 |
| .unwrap()            |                 |                                |
| .width()             |                 |                                |
| .wrap()              |                 |                                |
| .wrapAll()           |                 |                                |
| .wrapInner()         |                 |                                |

### Traversing

| jQuery: Traversing   | `Document`      | `Element` + `Collection`       |
| -------------------- | --------------- | ------------------------------ |
| .add()               |                 |                                |
| .addBack()           |                 |                                |
| ~.andSelf()~         | -               | -                              |
| .children()          | children        | children                       |
| .closest()           |                 |                                |
| .contents()          |                 |                                |
| .each()              |                 |                                |
| .end()               |                 |                                |
| .eq()                |                 |                                |
| .even()              |                 |                                |
| .filter()            |                 |                                |
| .find()              | find            | find                           |
| .first()             |                 | first                          |
| .has()               |                 |                                |
| .is()                |                 |                                |
| .last()              |                 | last                           |
| .map()               |                 |                                |
| .next()              |                 | next                           |
| .nextAll()           |                 |                                |
| .nextUntil()         |                 |                                |
| .not()               |                 |                                |
| .odd()               |                 |                                |
| .offsetParent()      |                 |                                |
| .parent()            |                 | parent                         |
| .parents()           |                 |                                |
| .parentsUntil()      |                 |                                |
| .prev()              |                 | prev                           |
| .prevAll()           |                 |                                |
| .prevUntil()         |                 |                                |
| .siblings()          |                 |                                |
| .slice()             |                 |                                |

## Copyright and license

Licensed under an OpenBSD-ISC-style license, see [LICENSE] for details.
