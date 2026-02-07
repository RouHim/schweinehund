## [1.1.2](https://github.com/RouHim/schweinehund/compare/v1.1.1...v1.1.2) (2026-02-07)


### Bug Fixes

* **ci:** install gh before uploading release binary ([814739c](https://github.com/RouHim/schweinehund/commit/814739c9a29e8a164ce75110f61ad76ad14802da))

## [1.1.1](https://github.com/RouHim/schweinehund/compare/v1.1.0...v1.1.1) (2026-02-07)


### Bug Fixes

* **ci:** wire release outputs and artifact flow ([149a669](https://github.com/RouHim/schweinehund/commit/149a669ea29bc82375aca3a61b2c5948ce9534c9))

# [1.1.0](https://github.com/RouHim/schweinehund/compare/v1.0.0...v1.1.0) (2026-02-07)


### Bug Fixes

* add missing zone column to deep_cleaning_tasks and set completed_at on completion ([57b6970](https://github.com/RouHim/schweinehund/commit/57b697086389b8f577fb2f69f9f4a320783ff874))
* **api:** correct day-of-week mapping from 0-6 to 1-7 ([1573e4d](https://github.com/RouHim/schweinehund/commit/1573e4deec9191a7de3ace2ea1a3dfe77659ccf6))
* **ci:** avoid duplicate server startup in Playwright E2E ([607bbfc](https://github.com/RouHim/schweinehund/commit/607bbfcda4810c0397e735a8bf39ef4ba48c7e06))
* **ci:** isolate e2e notifications from production topic ([e372f7f](https://github.com/RouHim/schweinehund/commit/e372f7fbfb41a176aed4c2266f491e27276f9bf6))
* resolve test regressions and isolation issues ([d53a8be](https://github.com/RouHim/schweinehund/commit/d53a8bedef81a36700ff3eda85aff0a97c4cb5ee))
* **ui:** improve dialog contrast and refactor core features ([ef33452](https://github.com/RouHim/schweinehund/commit/ef3345283b08434d5ab0de43a5890da500f1476a))


### Features

* **api:** add CRUD endpoints for daily tasks ([eac5249](https://github.com/RouHim/schweinehund/commit/eac524990c7e1afb1b2dfdca5b28429cb444e619))
* **api:** implement REST endpoints for tasks and settings ([92ca06c](https://github.com/RouHim/schweinehund/commit/92ca06cc3d6ec9b53fdb976a6746323d979a96f6))
* **assets:** add rust-embed for static file serving ([62598a3](https://github.com/RouHim/schweinehund/commit/62598a37e60f95da939e7895d28844bb5149ec80))
* **branding:** add Schweinehund mascot and app icons ([9a347f3](https://github.com/RouHim/schweinehund/commit/9a347f337995719c5a18cf8698024a22048c368d)), closes [#FF7F66](https://github.com/RouHim/schweinehund/issues/FF7F66)
* **db:** add initial schema with seed data for cleaning tasks ([b63c698](https://github.com/RouHim/schweinehund/commit/b63c698a536a2d7112eb67d5a65d61a5a1ca03aa))
* **db:** implement task data access layer with sqlx ([580b82f](https://github.com/RouHim/schweinehund/commit/580b82fd9806a124eecb69d835fbf4da95def5c7))
* **init:** scaffold Schweinehund project with dependencies ([9264bee](https://github.com/RouHim/schweinehund/commit/9264bee4f2d511d49f023d2dfe1184c3028485fc))
* **notifications:** include task list and top deep cleaning in daily reminder ([108a3e7](https://github.com/RouHim/schweinehund/commit/108a3e7bcbefd4e280a8b769e3b03d06a607ad10))
* **notifications:** load .env at startup and add notify-status debug endpoint ([4d3469e](https://github.com/RouHim/schweinehund/commit/4d3469eeec283aaca73f1a832406f18395659b51))
* **ntfy:** add push notification integration ([5a0d57e](https://github.com/RouHim/schweinehund/commit/5a0d57e80c82754ad7024ab3fb27ac7e59cb88d2))
* **pwa:** add manifest and service worker for installability ([fb2717b](https://github.com/RouHim/schweinehund/commit/fb2717bb5f70f5481e7d0c82d8789ae1167f5fea))
* **scheduler:** add weekly reset with startup reconciliation ([e370774](https://github.com/RouHim/schweinehund/commit/e37077426f89473e9139ee9487b7c9677a941efa))
* **ui:** add fun-fact popup after completing all daily tasks ([fd8e844](https://github.com/RouHim/schweinehund/commit/fd8e844702b88479ff709e0a4d0763b34e05f6de))
* **ui:** add modal dialog component for task create/edit ([a976999](https://github.com/RouHim/schweinehund/commit/a97699908775c14d612b3101e2ea657a5485b770))
* **ui:** add task buttons in section headers ([9fbd4c6](https://github.com/RouHim/schweinehund/commit/9fbd4c6dd8d04af88eef01b3ad0dc453bcfdeceb))
* **ui:** add two-column desktop layout for wider screens ([53f9731](https://github.com/RouHim/schweinehund/commit/53f9731133b645878454c0653a1876337a5ffa45))
* **ui:** implement mobile-first task UI with Schweinehund theme ([0be024c](https://github.com/RouHim/schweinehund/commit/0be024c91fdcd738d5d427fc4dd084c26b99065c))
* **ui:** reorder completed daily tasks to bottom with dimmed styling ([9d35ed1](https://github.com/RouHim/schweinehund/commit/9d35ed1add2c614df2fd842948fdd4e19ac2e197))

# 1.0.0 (2026-02-07)


### Bug Fixes

* add missing zone column to deep_cleaning_tasks and set completed_at on completion ([57b6970](https://github.com/RouHim/schweinehund/commit/57b697086389b8f577fb2f69f9f4a320783ff874))
* **api:** correct day-of-week mapping from 0-6 to 1-7 ([1573e4d](https://github.com/RouHim/schweinehund/commit/1573e4deec9191a7de3ace2ea1a3dfe77659ccf6))
* **ci:** avoid duplicate server startup in Playwright E2E ([607bbfc](https://github.com/RouHim/schweinehund/commit/607bbfcda4810c0397e735a8bf39ef4ba48c7e06))
* **ci:** isolate e2e notifications from production topic ([e372f7f](https://github.com/RouHim/schweinehund/commit/e372f7fbfb41a176aed4c2266f491e27276f9bf6))
* resolve test regressions and isolation issues ([d53a8be](https://github.com/RouHim/schweinehund/commit/d53a8bedef81a36700ff3eda85aff0a97c4cb5ee))
* **ui:** improve dialog contrast and refactor core features ([ef33452](https://github.com/RouHim/schweinehund/commit/ef3345283b08434d5ab0de43a5890da500f1476a))


### Features

* **api:** add CRUD endpoints for daily tasks ([eac5249](https://github.com/RouHim/schweinehund/commit/eac524990c7e1afb1b2dfdca5b28429cb444e619))
* **api:** implement REST endpoints for tasks and settings ([92ca06c](https://github.com/RouHim/schweinehund/commit/92ca06cc3d6ec9b53fdb976a6746323d979a96f6))
* **assets:** add rust-embed for static file serving ([62598a3](https://github.com/RouHim/schweinehund/commit/62598a37e60f95da939e7895d28844bb5149ec80))
* **branding:** add Schweinehund mascot and app icons ([9a347f3](https://github.com/RouHim/schweinehund/commit/9a347f337995719c5a18cf8698024a22048c368d)), closes [#FF7F66](https://github.com/RouHim/schweinehund/issues/FF7F66)
* **db:** add initial schema with seed data for cleaning tasks ([b63c698](https://github.com/RouHim/schweinehund/commit/b63c698a536a2d7112eb67d5a65d61a5a1ca03aa))
* **db:** implement task data access layer with sqlx ([580b82f](https://github.com/RouHim/schweinehund/commit/580b82fd9806a124eecb69d835fbf4da95def5c7))
* **init:** scaffold Schweinehund project with dependencies ([9264bee](https://github.com/RouHim/schweinehund/commit/9264bee4f2d511d49f023d2dfe1184c3028485fc))
* **notifications:** include task list and top deep cleaning in daily reminder ([108a3e7](https://github.com/RouHim/schweinehund/commit/108a3e7bcbefd4e280a8b769e3b03d06a607ad10))
* **notifications:** load .env at startup and add notify-status debug endpoint ([4d3469e](https://github.com/RouHim/schweinehund/commit/4d3469eeec283aaca73f1a832406f18395659b51))
* **ntfy:** add push notification integration ([5a0d57e](https://github.com/RouHim/schweinehund/commit/5a0d57e80c82754ad7024ab3fb27ac7e59cb88d2))
* **pwa:** add manifest and service worker for installability ([fb2717b](https://github.com/RouHim/schweinehund/commit/fb2717bb5f70f5481e7d0c82d8789ae1167f5fea))
* **scheduler:** add weekly reset with startup reconciliation ([e370774](https://github.com/RouHim/schweinehund/commit/e37077426f89473e9139ee9487b7c9677a941efa))
* **ui:** add fun-fact popup after completing all daily tasks ([fd8e844](https://github.com/RouHim/schweinehund/commit/fd8e844702b88479ff709e0a4d0763b34e05f6de))
* **ui:** add modal dialog component for task create/edit ([a976999](https://github.com/RouHim/schweinehund/commit/a97699908775c14d612b3101e2ea657a5485b770))
* **ui:** add task buttons in section headers ([9fbd4c6](https://github.com/RouHim/schweinehund/commit/9fbd4c6dd8d04af88eef01b3ad0dc453bcfdeceb))
* **ui:** add two-column desktop layout for wider screens ([53f9731](https://github.com/RouHim/schweinehund/commit/53f9731133b645878454c0653a1876337a5ffa45))
* **ui:** implement mobile-first task UI with Schweinehund theme ([0be024c](https://github.com/RouHim/schweinehund/commit/0be024c91fdcd738d5d427fc4dd084c26b99065c))
* **ui:** reorder completed daily tasks to bottom with dimmed styling ([9d35ed1](https://github.com/RouHim/schweinehund/commit/9d35ed1add2c614df2fd842948fdd4e19ac2e197))
