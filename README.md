# jQuery-like API for web-sys

[![docs.rs](https://docs.rs/web-sys-query/badge.svg)](https://docs.rs/web-sys-query)
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

## Feature flags

- `serde-serialize`: use `serde_derive` to enable `Serialize` and
  `Deserialize` support on types such as `FormData` (disabled by
  default).

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

### Events

| jQuery: Attributes   | `Document`      | `Element` +  `Collection`      |
| -------------------- | --------------- | ------------------------------ |
| ~.bind()~            |                 |                                |
| .blur()              |                 | blur, set_blur                 |
| .change()            |                 | change, set_change             |
| .click()             |                 | click, set_click               |
| .contextmenu()       |                 | context_menu, set_context_menu |
| .dblclick()          |                 | dbl_click, set_dbl_click       |
| ~.delegate()~        |                 |                                |
| ~.die()~             |                 |                                |
| ~.error()~           |                 |                                |
| .focus()             |                 | focus, set_focus               |
| .focusin()           |                 |                                |
| .focusout()          |                 |                                |
| .hover()             |                 |                                |
| .keydown()           |                 | key_down, set_key_down         |
| .keypress()          |                 | key_press, set_key_press       |
| .keyup()             |                 | key_up, set_key_up             |
| ~.live()~            |                 |                                |
| .load()              |                 |                                |
| .mousedown()         |                 | mouse_down, set_mouse_down     |
| .mouseenter()        |                 | mouse_enter, set_mouse_enter   |
| .mouseleave()        |                 | mouse_leave, set_mouse_leave   |
| .mousemove()         |                 | mouse_move, set_mouse_move     |
| .mouseout()          |                 | mouse_out, set_mouse_out       |
| .mouseover()         |                 | mouse_over, set_mouse_over     |
| .mouseup()           |                 | mouse_up, set_mouse_up         |
| .off()               |                 | set_off                        |
| .on()                |                 | on, set_on                     |
| .one()               |                 |                                |
| .ready()             |                 |                                |
| .resize()            |                 | resize, set_resize             |
| .scroll()            |                 | scroll, set_scroll             |
| .select()            |                 | select, set_select             |
| .submit()            |                 | submit, set_submit             |
| ~.toggle()~          |                 |                                |
| .trigger()           |                 |                                |
| .triggerHandler()    |                 |                                |
| ~.unbind()~          |                 |                                |
| ~.undelegate()~      |                 |                                |
| ~.unload()~          |                 |                                |

### Manipulation

| jQuery: Traversing   | `Document`      | `Element` + `Collection`       |
| -------------------- | --------------- | ------------------------------ |
| .after()             |                 |                                |
| .append()            |                 |                                |
| .appendTo()          |                 |                                |
| .before()            |                 |                                |
| .clone()             |                 |                                |
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
| ~.andSelf()~         |                 |                                |
| .children()          | children        | children                       |
| .closest()           |                 | closest                        |
| .contents()          |                 |                                |
| .each()              |                 |                                |
| .end()               |                 |                                |
| .eq()                |                 |                                |
| .even()              |                 |                                |
| .filter()            |                 | filter                         |
| .find()              | find            | find                           |
| .first()             |                 | first                          |
| .has()               |                 | has                            |
| .is()                |                 | is                             |
| .last()              |                 | last                           |
| .map()               |                 |                                |
| .next()              |                 | next                           |
| .nextAll()           |                 |                                |
| .nextUntil()         |                 |                                |
| .not()               |                 | not                            |
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

### Helper Functions

| jQuery: Traversing   | `Document`      | `Element` + `Collection`       |
| -------------------- | --------------- | ------------------------------ |
| jQuery.param()       |                 |                                |
| .serialize()         |                 |                                |
| .serializeArray()    |                 | serialize_array, `FormData`    |

## Copyright and license

Licensed under an OpenBSD-ISC-style license, see [LICENSE](LICENSE) for details.
