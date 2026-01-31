# 1.0.0 (2026-01-31)


### Bug Fixes

* **ci:** bypass git hooks in semantic-release commits ([3d7aff2](https://github.com/RouHim/schweinehund/commit/3d7aff2053780f3c11abb2f49a160396a03edb5a))
* **ci:** disable commitlint line length rules ([f233ad6](https://github.com/RouHim/schweinehund/commit/f233ad63fec0bd00155cd302419fb2f56b081a8c))
* **ci:** explicitly set registry for Docker Hub login ([6cc368f](https://github.com/RouHim/schweinehund/commit/6cc368f2251ff9e4229d1c67443ef039d9498cea))
* **ci:** improve semantic-release version detection with verbose output ([6177afe](https://github.com/RouHim/schweinehund/commit/6177afe0594e0608a9ed3461f52cbc3f09ba7b6b))
* **ci:** remove Docker plugin from semantic-release and reorder workflow ([376e4c9](https://github.com/RouHim/schweinehund/commit/376e4c9bc7b653a6ed45ebc3b828b75c864336df))
* **ci:** remove test from pre-commit hook ([e81490c](https://github.com/RouHim/schweinehund/commit/e81490c66a661496bee9ab412eaa0c9831ccae88))


### Features

* **ci:** add Playwright tests, semantic-release, and CI/CD pipeline ([31e66ce](https://github.com/RouHim/schweinehund/commit/31e66ceac80aced1188467aa1f2788d82a5a994f))
* **db:** PocketBase schema with tasks, zones, settings ([2e32325](https://github.com/RouHim/schweinehund/commit/2e323258722049d4a5e3b4f768f04ffb087a6a04))
* **deploy:** production-ready Docker Compose setup ([d14302c](https://github.com/RouHim/schweinehund/commit/d14302ce3a9104de995c11b4b2f8980690606b41))
* **design:** add Schweinehund logo and PWA icons ([a9eb1ae](https://github.com/RouHim/schweinehund/commit/a9eb1aed762c76c36abb3584266e8d5377002ee6))
* **frontend:** base layout with HTMX and Alpine.js ([f00fd22](https://github.com/RouHim/schweinehund/commit/f00fd222693f1eca4cc7255ddb4e6bec12c4a242))
* **notify:** ntfy.sh integration for push notifications ([0238880](https://github.com/RouHim/schweinehund/commit/02388806f18cacd5636b4701e370a0cab7d87ee6))
* **ntfy:** add ntfy.sh container for push notifications ([8028979](https://github.com/RouHim/schweinehund/commit/8028979d2fdbdcafdddef3003343cebe5eb3f954))
* **pwa:** manifest.json and service worker ([4cf2815](https://github.com/RouHim/schweinehund/commit/4cf2815c2b2b738076f8af218691195c0c3989de)), closes [#FF7F50](https://github.com/RouHim/schweinehund/issues/FF7F50) [#FFF5E6](https://github.com/RouHim/schweinehund/issues/FFF5E6) [#FF7F50](https://github.com/RouHim/schweinehund/issues/FF7F50) [#FFF5E6](https://github.com/RouHim/schweinehund/issues/FFF5E6)
* **scheduler:** weekly reset for daily tasks on Monday 00:00 ([5fee54c](https://github.com/RouHim/schweinehund/commit/5fee54cbd4199e4bf8f7884be003d476e4877ff1))
* **setup:** initial project structure with Docker ([28576a5](https://github.com/RouHim/schweinehund/commit/28576a5adbaa5b90d8e3e479221a118096d1b32b))
* **ui:** playful color theme and base styles ([d2c42ab](https://github.com/RouHim/schweinehund/commit/d2c42ab7cd57b3e890300ccfb02bb67c6b6f7a71))
* **ui:** task list, edit modal, and zone management views ([a11a613](https://github.com/RouHim/schweinehund/commit/a11a61369968754b4fb8aa605d623020ba99cb82))


### BREAKING CHANGES

* **ci:** None - initial CI/CD setup
