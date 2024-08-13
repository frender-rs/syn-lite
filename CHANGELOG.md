# Changelog

## [0.2.0](https://github.com/frender-rs/syn-lite/compare/syn-lite-v0.1.2...syn-lite-v0.2.0) (2024-08-13)


### ⚠ BREAKING CHANGES

* for consume_till_outer_gt!, consume_till_outer_gt_inclusive! and consume_bounds!, the `>` in `->` is now never considered as an outer `>`

### Features

* for consume_till_outer_gt!, consume_till_outer_gt_inclusive! and consume_bounds!, the `&gt;` in `->` is now never considered as an outer `>` ([e9296ad](https://github.com/frender-rs/syn-lite/commit/e9296ad7cec8f510cd7fab673792c080e74fcd1e))

## [0.1.2](https://github.com/frender-rs/syn-lite/compare/syn-lite-v0.1.1...syn-lite-v0.1.2) (2024-08-13)


### Features

* consume_till_outer_gt_inclusive! and consume_optional_angle_bracketed! ([ecc27bf](https://github.com/frender-rs/syn-lite/commit/ecc27bffc8ae7de38aeacb998f782e3cce2dada4))

## [0.1.1](https://github.com/frender-rs/syn-lite/compare/syn-lite-v0.1.0...syn-lite-v0.1.1) (2024-08-13)


### Features

* consume_till_outer_gt! and consume_bounds! ([d1dc835](https://github.com/frender-rs/syn-lite/commit/d1dc8359fc1536e8b7870797bf4eedb75afa5bc2))

## 0.1.0 (2023-03-18)


### Features

* expand_or and expand_if_else ([b162da3](https://github.com/frender-rs/syn-lite/commit/b162da3836841db88ff79a1ccbf6cf7d1d54b578))
* parse_generics ([3036a97](https://github.com/frender-rs/syn-lite/commit/3036a976488670a7b7e2e3b1dd01b61cc68a64a6))
* parse_inner_attrs ([7011f49](https://github.com/frender-rs/syn-lite/commit/7011f49e885208c9964fde0d52bdd89c599dde2c))
* parse_item_fn ([5d5fc62](https://github.com/frender-rs/syn-lite/commit/5d5fc62bc1cafa7f3c1451903e873e157b5d5e00))
* parse_where_clause ([8142eeb](https://github.com/frender-rs/syn-lite/commit/8142eebc64a213535aacb40c0230f54a8e09e536))


### Bug Fixes

* where clause magical macros `__![...]: __` not working ([549081e](https://github.com/frender-rs/syn-lite/commit/549081e96cc75d9b1f3b00f0142149444fe91158))
