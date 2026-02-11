# [1.5.0](https://github.com/RouHim/schweinehund/compare/v1.4.0...v1.5.0) (2026-02-11)


### Features

* **api:** accept user-provided start_date in task create/update endpoints ([3fabc94](https://github.com/RouHim/schweinehund/commit/3fabc94752066fc47076006558306d682743731a))
* **db:** add start_date filtering to hide future-dated tasks from today view ([bd491ae](https://github.com/RouHim/schweinehund/commit/bd491ae63a99ef08622d9562473c1d264ebd1ae7))
* **ui:** add start date picker to task modal with future-date badge ([3467501](https://github.com/RouHim/schweinehund/commit/3467501ed9767c08d1f02e69c27a740e920ef4e3))

# [1.4.0](https://github.com/RouHim/schweinehund/compare/v1.3.0...v1.4.0) (2026-02-11)


### Bug Fixes

* **db:** update SELECT queries to include interval_weeks and start_date columns ([04a7dd6](https://github.com/RouHim/schweinehund/commit/04a7dd6c5ae33aff6f00ba765bb7959280f9c3d9))
* **e2e:** add database reset to all test files for state isolation ([cf9bfd1](https://github.com/RouHim/schweinehund/commit/cf9bfd1cd0145abc4c65ef4363a3d00389fefc9f))
* **e2e:** expose handleDragReorder and attachTaskListeners on window ([b3a5db0](https://github.com/RouHim/schweinehund/commit/b3a5db0bb4badfedc926a887d2f63af5e25d604f))
* **e2e:** replace stale DOM locators with name-based selectors ([004640c](https://github.com/RouHim/schweinehund/commit/004640c60789aa544b5813e0a1b860ec0bad1dc8))
* **frontend:** add missing attachTaskListeners function and remove arrow reorder buttons ([150ce7d](https://github.com/RouHim/schweinehund/commit/150ce7d34689a2709012caafb9c9b08a709f0a86))
* **fun-fact:** restore popup trigger and stabilize e2e completion flow ([54bbdb3](https://github.com/RouHim/schweinehund/commit/54bbdb3d53bde18d32c52e08aac43bdf26aba31d))


### Features

* **api:** accept interval_weeks in task create/update with validation ([ca35973](https://github.com/RouHim/schweinehund/commit/ca35973914ce815b0b01d80b70450e5f5eb4572f))
* **api:** add full database reset endpoint for E2E test isolation ([3bf4914](https://github.com/RouHim/schweinehund/commit/3bf4914c6c85bad3b844345944523a462f551238))
* **api:** add GET /api/tasks/all endpoint for overview ([1fc70fc](https://github.com/RouHim/schweinehund/commit/1fc70fcf048f83a7975e4d7c8c96e5d5befa04b0))
* **db:** add interval_weeks and start_date columns to daily_tasks ([289e86f](https://github.com/RouHim/schweinehund/commit/289e86fbcaef8510c2d7040b2afef5c8f5324613))
* **db:** filter today's tasks by interval_weeks ([e72a9d7](https://github.com/RouHim/schweinehund/commit/e72a9d7ec886decdb287592db4dabdee576fa5f1))
* **frontend:** add drag-and-drop reorder for deep cleaning tasks ([ea3914e](https://github.com/RouHim/schweinehund/commit/ea3914ee2dea71700fe5984dd5da0ba866a0c112))
* **frontend:** vendor SortableJS for drag-and-drop support ([cca4dc9](https://github.com/RouHim/schweinehund/commit/cca4dc9b9d403471cbe5a7599e156a8be7803305))
* **scheduler:** make daily task reset interval-aware ([8a5d264](https://github.com/RouHim/schweinehund/commit/8a5d26472cc7ad41ba4f3f7c31e72d0b17106eb6))
* **ui:** add interval picker to daily task modal ([7467637](https://github.com/RouHim/schweinehund/commit/7467637e02003f3e6c3a8ded8d26fd031fb09b75))
* **ui:** add tab-bar navigation and all-tasks overview view ([ac13690](https://github.com/RouHim/schweinehund/commit/ac1369042ece30e4f10639168ee738f9b2589ac8))

# [1.3.0](https://github.com/RouHim/schweinehund/compare/v1.2.0...v1.3.0) (2026-02-09)


### Features

* **ui:** replace deep cleaning checkbox with 'Erledigt' rotation button ([ad2e3e7](https://github.com/RouHim/schweinehund/commit/ad2e3e7db32ba3e5ce5162578c0f8665243b3be6))

# [1.2.0](https://github.com/RouHim/schweinehund/compare/v1.1.4...v1.2.0) (2026-02-08)


### Features

* **i18n:** translate entire UI to German ([22ae294](https://github.com/RouHim/schweinehund/commit/22ae2945f0f5617efe0ada3e46cf247cd2704ecd))

## [1.1.4](https://github.com/RouHim/schweinehund/compare/v1.1.3...v1.1.4) (2026-02-07)


### Bug Fixes

* **container:** run as uid 1000 and embed debug static assets ([c17d1e7](https://github.com/RouHim/schweinehund/commit/c17d1e71c4b4656acc29c627f84ae42b6cd16de1))

## [1.1.3](https://github.com/RouHim/schweinehund/compare/v1.1.2...v1.1.3) (2026-02-07)


### Bug Fixes

* **ci:** pass musl binary via artifacts between release jobs ([0700da4](https://github.com/RouHim/schweinehund/commit/0700da47281a469884067c90f14892f46cca703b))

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
