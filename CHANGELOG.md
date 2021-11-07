# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.3.1](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.3.0...v0.3.1) (2021-11-07)


### Features

* add TLS support ([5a8eb21](https://github.com/ravenclaw900/DietPi-Dashboard/commit/5a8eb215ae3ebe407afa6f2ad55b822418ca295c))
* **backend:** add config file ([10d917b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/10d917b6045a37b61873a298507615d57a313593))
* **filebrowser:** allow viewing images from browser ([921913d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/921913d9bc2b0c86f012703864640b8117336bb4))
* **processes:** skip kernel threads ([4a0ab2d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/4a0ab2d22af5bb3dfc59ef6769220e943ac03245))


### Bug Fixes

* **dashboard:** switch back to old method of getting CPU usage ([5f3cc1e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/5f3cc1e4bd55d06c73f665a3bbad97775798e629))
* **filebrowser:** add `/root` fallback in case $HOME variable is not set ([9e606bb](https://github.com/ravenclaw900/DietPi-Dashboard/commit/9e606bb89b70dc090c35e6b0560c4d55abe14e35))
* **filebrowser:** convert "/root" into an OsString ([279c71a](https://github.com/ravenclaw900/DietPi-Dashboard/commit/279c71a9a90541bbfe10d20ca9adf8eedf2da109))
* **frontend:** change table header text color from white to black ([#17](https://github.com/ravenclaw900/DietPi-Dashboard/issues/17)) ([d89dc4c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/d89dc4c355b92f2a28fa7905e0d8d0f39735e53a))
* **management:** fix page crashing due to channel send error ([8731174](https://github.com/ravenclaw900/DietPi-Dashboard/commit/8731174075bc03b50558cf9b2cf680cdae2d0f9c)), closes [#20](https://github.com/ravenclaw900/DietPi-Dashboard/issues/20)
* **processes:** fix more NoProcessErrors ([a3e8d5b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/a3e8d5bd9ce5c3990a95f62dc1feb8ce32d3263e)), closes [#22](https://github.com/ravenclaw900/DietPi-Dashboard/issues/22)
* **processes:** stop NoSuchProcess error ([da0841b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/da0841b836a42aa64f81cf331e0ab82712b35240))
* **processes:** use tabs instead of colon for getting service status ([80c6ce5](https://github.com/ravenclaw900/DietPi-Dashboard/commit/80c6ce5a98de057526eada9e26114ec85d83a5cb)), closes [#14](https://github.com/ravenclaw900/DietPi-Dashboard/issues/14)

## [0.3.0](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.2.0...v0.3.0) (2021-10-31)


### Features

* **filebrowser:** add file browser ([c0ea2de](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c0ea2defec8ff701d5f255879e307cc2039fbced))
* **frontend:** show updates to DietPi ([1ba4b97](https://github.com/ravenclaw900/DietPi-Dashboard/commit/1ba4b9721ab55e33f6deac5dda06915a7a18e2bc))
* **management:** show installed and upgradable APT packages ([ca6a447](https://github.com/ravenclaw900/DietPi-Dashboard/commit/ca6a447ab1aa15417c1db6796fbc589e96ee8e42))
* **services:** add services page ([db128e6](https://github.com/ravenclaw900/DietPi-Dashboard/commit/db128e65773f70de9c0c3ad3ca5b7c663280eb30))


### Bug Fixes

* **backend:** add patch for heim to run on ARM64 ([de8e431](https://github.com/ravenclaw900/DietPi-Dashboard/commit/de8e431fee817bb6ae3fbc724f3811c299132d84))
* **backend:** fix errors for closed connections ([35c9351](https://github.com/ravenclaw900/DietPi-Dashboard/commit/35c935120a8c0f52c2056fde281df8187644ae95))
* **backend:** fix getting cpu percentage on dead processes ([f2b0e28](https://github.com/ravenclaw900/DietPi-Dashboard/commit/f2b0e28b4a0077c221c0515701965581078605e8))
* **backend:** make "arch" variable mutable ([2e4e0f0](https://github.com/ravenclaw900/DietPi-Dashboard/commit/2e4e0f04d754229b139aa42c599b3a66855c42c6))
* fix webpage getting stuck on "getting data" ([10de8f4](https://github.com/ravenclaw900/DietPi-Dashboard/commit/10de8f423fbca15dd90b06a7de71e6a48749d5f2))
* **frontend:** fix ChartConfiguration compilation error ([b3def7b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/b3def7b6119ffd0176990dfac4ff595f3de13f74))
* **frontend:** fix update sometimes being shown, even when none was avalible ([f812776](https://github.com/ravenclaw900/DietPi-Dashboard/commit/f812776482935fc785c4c2e41c181e9ad6cfba22))
* **frontend:** hide menu on screens smaller than 768px (tailwind md breakpoint) ([c5e6a34](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c5e6a34db1c0826bc9979cfbfec865a3b1e93b97))
* **management:** fix arch showing up as "unknown" for armv6 ([18215c7](https://github.com/ravenclaw900/DietPi-Dashboard/commit/18215c736fd895f5866bbd36533179ae3f35dfb6))
* **management:** fix uptime not being able to extend past 24h ([d3e45d1](https://github.com/ravenclaw900/DietPi-Dashboard/commit/d3e45d107ebbc42e1bbced5c32d8234924b96d2d))
* **processes:** fix getting cpu usage ([11dda26](https://github.com/ravenclaw900/DietPi-Dashboard/commit/11dda260f6c4d8e29dff3a8457cdb26363203101))
* **software:** fix serialization by removing `` (color) control character ([14c30a9](https://github.com/ravenclaw900/DietPi-Dashboard/commit/14c30a92dc77f3606cc1766e69168bc1530ca1b4))
* **software:** sometimes dietpi-software got started without args requiring the program to be killed ([056dc1a](https://github.com/ravenclaw900/DietPi-Dashboard/commit/056dc1a74bdfd4f901eb4da666acf4abcdb30e07))

### [0.2.1](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.2.0...v0.2.1) (2021-10-07)


### Bug Fixes

* **hotfix:** fix webpage getting stuck on "getting data" ([ef35126](https://github.com/ravenclaw900/DietPi-Dashboard/commit/ef351262d564a523451424d0c7f91b5eb91a86ab))

## [0.2.0](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.1.0...v0.2.0) (2021-09-26)


### Features

* add request logging ([ce6ff09](https://github.com/ravenclaw900/DietPi-Dashboard/commit/ce6ff09f974e0127fee8ed85bc0f71be4e992ffa))
* **dashboard:** make network data incremental, instead of all at once ([00b7753](https://github.com/ravenclaw900/DietPi-Dashboard/commit/00b77538ae6983214b3bc4249201fd824efc26f9))
* **frontend:** add dark mode ([03c3343](https://github.com/ravenclaw900/DietPi-Dashboard/commit/03c334324fda9b6fec41ffdd44e673c78f84586e))
* **frontend:** add gzip compression to pages ([1d99df8](https://github.com/ravenclaw900/DietPi-Dashboard/commit/1d99df87ddecf7d35152fd04e0be6f0e06018d7d))
* **frontend:** add page not found default route ([bff80a9](https://github.com/ravenclaw900/DietPi-Dashboard/commit/bff80a9d75b687df3ad8c46d4f54aa9397cd38b7))
* **processes:** add actions and status to process page ([8e6f32e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/8e6f32e7eb37abad7f4f84a17fa5bde8de4cd7a0))


### Bug Fixes

* **dashboard:** default to 0 if bytes go negative ([c09088c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c09088ca3826e1e42df0e421f35f5e2958fe162c))
* **processes:** fix other pages not working when changing away from process page ([e72fe5a](https://github.com/ravenclaw900/DietPi-Dashboard/commit/e72fe5a549035d4bcc3cfa5cfb69a9a32c0d7243))
* **terminal:** kill process after closing terminal page ([40db41c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/40db41c97083cd12847fc657a46d4a9e1700a24a))

## 0.1.0 (2021-09-06)
Initial Release!
