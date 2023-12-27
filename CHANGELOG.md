# Changelog

## [0.4.0](https://github.com/nfejzic/mathemascii/compare/v0.3.1...v0.4.0) (2023-12-27)


### ⚠ BREAKING CHANGES

* remove `Token::is_var` method
* improve rendering with fallbacks during parsing ([#14](https://github.com/nfejzic/mathemascii/issues/14))

### Features

* fallback to operator when no expr recognized ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* implicitly close grouping expressions ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* improve rendering with fallbacks during parsing ([#14](https://github.com/nfejzic/mathemascii/issues/14)) ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* remove `Token::is_var` method ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))


### Bug Fixes

* add fallback to Element conversion for Other keyword ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* add fallbacks when parsing color binary expr ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* correctly handle `norm` groupings ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* correctly lex set minus operators ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* correctly lex the divide symbol `-:` ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* correctly parse `cdots`, `ldots` etc ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* correctly render `and` and `or` logicals ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* fallback to default expression when parsing unary ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* fallback to operator when lexing unknown symbol ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* handle dots as operators instead of idents ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* improve handling of cdots etc ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* improve handling of matrix groupings ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* render groupings in a `mrow` element ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* simplify handling of groupings ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* simplify ungroup_map by direct conversion ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* use more operator constructors ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))
* use more pre-defined operator constructors ([7d8c86a](https://github.com/nfejzic/mathemascii/commit/7d8c86acc63a2e8337f7108c6f6a5972719ff202))


### Miscellaneous Chores

* remove release-please manifest. ([95cdecd](https://github.com/nfejzic/mathemascii/commit/95cdecd59628a1304156d5bc09f5d353123db252))

## [0.3.1](https://github.com/nfejzic/mathemascii/compare/v0.3.0...v0.3.1) (2023-12-22)


### Bug Fixes

* remove left-over `dbg!` statement ([#12](https://github.com/nfejzic/mathemascii/issues/12)) ([5677a4e](https://github.com/nfejzic/mathemascii/commit/5677a4eb272cbdf0e327cba602214e58bbe71078))

## [0.3.0](https://github.com/nfejzic/mathemascii/compare/v0.2.0...v0.3.0) (2023-12-22)


### Features

* implement parsing of ascii math ([#1](https://github.com/nfejzic/mathemascii/issues/1)) ([28e9abd](https://github.com/nfejzic/mathemascii/commit/28e9abdb548060c496a83c9a5bec3723a3f3c5d4))
* implement rendering to MathMl format ([#4](https://github.com/nfejzic/mathemascii/issues/4)) ([4171de1](https://github.com/nfejzic/mathemascii/commit/4171de1ac6ea7c93537990bfca8c2ce13f7891d7))
* reduce redundancy of grouping symbols ([#11](https://github.com/nfejzic/mathemascii/issues/11)) ([176696b](https://github.com/nfejzic/mathemascii/commit/176696bd4f5a91951a373bdbf6cf0dba6cfdf2cc))
* run tests with cargo-nextest ([92f4a45](https://github.com/nfejzic/mathemascii/commit/92f4a45e07d149cf51b0eafcb04de7ee32eb5851))

## [0.2.0](https://github.com/nfejzic/mathemascii/compare/0.1.0...v0.2.0) (2023-12-15)


### Features

* implement parsing of ascii math ([#1](https://github.com/nfejzic/mathemascii/issues/1)) ([28e9abd](https://github.com/nfejzic/mathemascii/commit/28e9abdb548060c496a83c9a5bec3723a3f3c5d4))
* implement rendering to MathMl format ([#4](https://github.com/nfejzic/mathemascii/issues/4)) ([4171de1](https://github.com/nfejzic/mathemascii/commit/4171de1ac6ea7c93537990bfca8c2ce13f7891d7))
* run tests with cargo-nextest ([92f4a45](https://github.com/nfejzic/mathemascii/commit/92f4a45e07d149cf51b0eafcb04de7ee32eb5851))
