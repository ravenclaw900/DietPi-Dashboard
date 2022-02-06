# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.5.0](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.4.1...v0.5.0) (2022-02-06)


### Features

* **filebrowser:** quality-of-life improvements ([#123](https://github.com/ravenclaw900/DietPi-Dashboard/issues/123)) ([8e2e404](https://github.com/ravenclaw900/DietPi-Dashboard/commit/8e2e40433d3ca543af3a086adb33d992976ad16e))
* **frontend:** allow clicking 'Enter' to send passwords ([#130](https://github.com/ravenclaw900/DietPi-Dashboard/issues/130)) ([0de4bbf](https://github.com/ravenclaw900/DietPi-Dashboard/commit/0de4bbff163972bac75c45bc83c9ad256c2d9ff5))
* **frontend:** store passwords for multiple backend nodes in localStorage ([8dceedc](https://github.com/ravenclaw900/DietPi-Dashboard/commit/8dceedc94c973bc419cd6d38de477aea15601ad6)), closes [#105](https://github.com/ravenclaw900/DietPi-Dashboard/issues/105)
* **processes:** use resident memory instead of virtual memory ([9d96c90](https://github.com/ravenclaw900/DietPi-Dashboard/commit/9d96c9087a5b4ef8653b7c04a81053bd78732ae8))


### Bug Fixes

* **backend:** add more error handling in socket_handler ([172a6e0](https://github.com/ravenclaw900/DietPi-Dashboard/commit/172a6e031ffccc61e8327654ad7d01745804b814))
* **frontend:** fix misspelling of "available" ([00e8703](https://github.com/ravenclaw900/DietPi-Dashboard/commit/00e8703b535f2dbd154a6317bc8cc2d94cf0c7b2))
* **frontend:** stop using deprecated `substr` method ([1a4c79b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/1a4c79bcd6062a9fdf1c4f87980e8fc8ede180a5))
* **terminal:** add more error handling ([7b1c051](https://github.com/ravenclaw900/DietPi-Dashboard/commit/7b1c051e96c513da593765131e7a73fc8853b790))
* **terminal:** allow terminal to work with multi-node passwords ([7a47661](https://github.com/ravenclaw900/DietPi-Dashboard/commit/7a476614d34cce3649545597430f550543bd9505))
* **terminal:** stop from freezing when typed into really quickly ([#144](https://github.com/ravenclaw900/DietPi-Dashboard/issues/144)) ([c5ebc71](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c5ebc7170b48318dc25615f3940be3b195718178))

### [0.4.1](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.4.0...v0.4.1) (2021-12-30)


### Features

* **frontend:** Show "Incorrect password" if the password is wrong ([02c540e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/02c540e43a443f9990c9913fec8f4cd5cd59e5ff))


### Bug Fixes

* **backend:** Remove 'IconDefinition' ([e521fb4](https://github.com/ravenclaw900/DietPi-Dashboard/commit/e521fb4540cf57f70b611d731be80723cd03e4cb))
* **config:** remove testing nodes ([4bab08e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/4bab08e6e27cd105296a18e88d27110042d056a3))
* don't show select box unless other nodes are available to select ([ea6b94a](https://github.com/ravenclaw900/DietPi-Dashboard/commit/ea6b94a8bc01c396c44156e6cf493ffe49bc8aa0))

## [0.4.0](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.3.2...v0.4.0) (2021-12-30)


### Features

* **frontend:** allow managing multiple nodes on one page ([#66](https://github.com/ravenclaw900/DietPi-Dashboard/issues/66)) ([c2b6953](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c2b695349233adbf4f2cf196dd25b75737dddb3f))


### Bug Fixes

* **backend:** change port to 5252 ([962ef1d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/962ef1db3c391d2464cb96b826f566b1c9b6af73))
* **backend:** strip extra null characters from output ([fd7711c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/fd7711c2bf06617e81ac924c8aa7e31b1d42ae23))
* **backend:** use UTC timestamps on logger ([a3853c2](https://github.com/ravenclaw900/DietPi-Dashboard/commit/a3853c2697d5a0606499c41fc6cf456543e01a4e))
* **filebrowser:** only set path if file can be opened ([e901295](https://github.com/ravenclaw900/DietPi-Dashboard/commit/e901295f3c05e82f4c845a4eecb5d941f8f4883c))

### [0.3.3](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.3.2...v0.3.3) (2021-12-30)


### Features

* **frontend:** allow managing multiple nodes on one page ([#66](https://github.com/ravenclaw900/DietPi-Dashboard/issues/66)) ([c2b6953](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c2b695349233adbf4f2cf196dd25b75737dddb3f))


### Bug Fixes

* **backend:** change port to 5252 ([962ef1d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/962ef1db3c391d2464cb96b826f566b1c9b6af73))
* **backend:** strip extra null characters from output ([fd7711c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/fd7711c2bf06617e81ac924c8aa7e31b1d42ae23))
* **backend:** use UTC timestamps on logger ([a3853c2](https://github.com/ravenclaw900/DietPi-Dashboard/commit/a3853c2697d5a0606499c41fc6cf456543e01a4e))
* **filebrowser:** only set path if file can be opened ([e901295](https://github.com/ravenclaw900/DietPi-Dashboard/commit/e901295f3c05e82f4c845a4eecb5d941f8f4883c))

### [0.3.4](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.3.2...v0.3.4) (2021-12-30)


### Features

* **frontend:** allow managing multiple nodes on one page ([#66](https://github.com/ravenclaw900/DietPi-Dashboard/issues/66)) ([c2b6953](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c2b695349233adbf4f2cf196dd25b75737dddb3f))


### Bug Fixes

* **backend:** change port to 5252 ([962ef1d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/962ef1db3c391d2464cb96b826f566b1c9b6af73))
* **backend:** strip extra null characters from output ([fd7711c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/fd7711c2bf06617e81ac924c8aa7e31b1d42ae23))
* **backend:** use UTC timestamps on logger ([a3853c2](https://github.com/ravenclaw900/DietPi-Dashboard/commit/a3853c2697d5a0606499c41fc6cf456543e01a4e))
* **filebrowser:** only set path if file can be opened ([e901295](https://github.com/ravenclaw900/DietPi-Dashboard/commit/e901295f3c05e82f4c845a4eecb5d941f8f4883c))

### [0.3.3](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.3.2...v0.3.3) (2021-12-30)


### Bug Fixes

* **backend:** change port to 5252 ([962ef1d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/962ef1db3c391d2464cb96b826f566b1c9b6af73))
* **backend:** strip extra null characters from output ([fd7711c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/fd7711c2bf06617e81ac924c8aa7e31b1d42ae23))
* **backend:** use UTC timestamps on logger ([a3853c2](https://github.com/ravenclaw900/DietPi-Dashboard/commit/a3853c2697d5a0606499c41fc6cf456543e01a4e))
* **filebrowser:** only set path if file can be opened ([e901295](https://github.com/ravenclaw900/DietPi-Dashboard/commit/e901295f3c05e82f4c845a4eecb5d941f8f4883c))

### [0.3.2](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.3.1...v0.3.2) (2021-11-27)


### Features

* **frontend:** add dark mode trigger ([38bbd3a](https://github.com/ravenclaw900/DietPi-Dashboard/commit/38bbd3a63b846a647912c0cd4b5d4ae31dddd342))
* **frontend:** add persistent dark mode storage ([87b9366](https://github.com/ravenclaw900/DietPi-Dashboard/commit/87b936634ac508c29d00eeccf110b58740b69838)), closes [#52](https://github.com/ravenclaw900/DietPi-Dashboard/issues/52)
* implement password protection ([6eff076](https://github.com/ravenclaw900/DietPi-Dashboard/commit/6eff076ebe7863be8744797d84fef6c10d28e449))
* **software:** seperate tables ([7cb9b72](https://github.com/ravenclaw900/DietPi-Dashboard/commit/7cb9b7266a6d02693502b5e06602624d2faff43c)), closes [#48](https://github.com/ravenclaw900/DietPi-Dashboard/issues/48)
* **terminal:** add token validation to terminal ([3b138da](https://github.com/ravenclaw900/DietPi-Dashboard/commit/3b138da8b526d46214d102c446517f518a5bd17b))


### Bug Fixes

* **backend:** close connection even without handshake ([630d38d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/630d38d6dab2dbd90f458f4905bb66dc440493d4))
* **backend:** don't use synchronus sleep on asyncronus threads ([8a5d801](https://github.com/ravenclaw900/DietPi-Dashboard/commit/8a5d80176cf18f2ad8d2148c2c833c6c8372bff5))
* **backend:** fix terminal on single-core machines ([122c38b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/122c38b65414b2502fb69983038360ffb793b878))
* **config:** actually get config from executable directory, instead of cwd ([5965551](https://github.com/ravenclaw900/DietPi-Dashboard/commit/5965551fec41d355749c92c9c743c75b536bd815))
* **config:** make config variables mutable ([db7032e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/db7032e4eaa7810d579cf14b13575cf7d7e641d4))
* **dashboard:** have cards be vertical on portrait displays ([e0647b0](https://github.com/ravenclaw900/DietPi-Dashboard/commit/e0647b0b7b098c269549d8adb3799f507cd975b4))
* **dashboard:** make charts look better in portrait mode ([0773943](https://github.com/ravenclaw900/DietPi-Dashboard/commit/0773943aae9b57bbc429ae86098a8d7049c3d755))
* don't push changes to service and management pages ([9bd3033](https://github.com/ravenclaw900/DietPi-Dashboard/commit/9bd30338e458666a46542067ed6b2584f42d019e))
* **frontend:** add connect-src to CSP ([f8afa8b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/f8afa8b5604d2f34f66237a2cbe3d16d45dd55ed))
* **frontend:** allow not using passwords ([1607ece](https://github.com/ravenclaw900/DietPi-Dashboard/commit/1607ecec1ff56a52056406706a20049507cbf984))
* **frontend:** allow using token with every action ([109ddc7](https://github.com/ravenclaw900/DietPi-Dashboard/commit/109ddc78ed87e52553daa32d44a667cb893f595c))
* **frontend:** fix pages with tables not loading by direct access ([8c8b63c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/8c8b63cd555f4ee71a3d14dc282736981e46ff07))
* **frontend:** hide terminal when not logged in ([274ed46](https://github.com/ravenclaw900/DietPi-Dashboard/commit/274ed466b1e785e7b5d49b4e8bfd26444b7a5581))
* **frontend:** soft code port for websocket ([#35](https://github.com/ravenclaw900/DietPi-Dashboard/issues/35)) ([bc5bf51](https://github.com/ravenclaw900/DietPi-Dashboard/commit/bc5bf51c805a210fd2189972543a43e7f5aedc7c))
* **software:** change color of response box on dark mode ([cf81782](https://github.com/ravenclaw900/DietPi-Dashboard/commit/cf817825179300cfc370602abe214aaae2df7b81))
* **software:** fix software page not working when there are free IDs ([19e3e39](https://github.com/ravenclaw900/DietPi-Dashboard/commit/19e3e39f6d78df65421ba81231e4545a9ed5cf60))
* **terminal:** stop zombie bash processes from stacking up ([cdaa017](https://github.com/ravenclaw900/DietPi-Dashboard/commit/cdaa0178d2f21635bc148a657b0624cbe820c385)), closes [#24](https://github.com/ravenclaw900/DietPi-Dashboard/issues/24)

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
