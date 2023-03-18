# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.6.1](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.6.0...v0.6.1) (2022-08-15)


### Features

* **backend:** make warp listen on both IPv4 and IPv6 ([#216](https://github.com/ravenclaw900/DietPi-Dashboard/issues/216)) ([8bcf1b2](https://github.com/ravenclaw900/DietPi-Dashboard/commit/8bcf1b25ed62930f2164e84f122dbc852f103242))
* **backend:** store files in binary as compressed ([#263](https://github.com/ravenclaw900/DietPi-Dashboard/issues/263)) ([cd80f5a](https://github.com/ravenclaw900/DietPi-Dashboard/commit/cd80f5a6b01856bbe6f361199ad07d1140e1f62d))
* **dashboard:** add CPU temperature ([#232](https://github.com/ravenclaw900/DietPi-Dashboard/issues/232)) ([6567d10](https://github.com/ravenclaw900/DietPi-Dashboard/commit/6567d10b2f88aca09c8292d85f479a397c3a5a1e))


### Bug Fixes

* **backend:** add error handling ([#247](https://github.com/ravenclaw900/DietPi-Dashboard/issues/247)) ([1acbd83](https://github.com/ravenclaw900/DietPi-Dashboard/commit/1acbd8303997517769249dc235585b4e7b7cf954))
* **backend:** quit socket_handler if there's a websocket error ([#312](https://github.com/ravenclaw900/DietPi-Dashboard/issues/312)) ([5bb4142](https://github.com/ravenclaw900/DietPi-Dashboard/commit/5bb41421f98170754c7f42ac512ba7ef258a6394))
* **backend:** remove useless `take` call when getting software ([711952e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/711952e97260d0da7ae23d9f690673260265d94c))
* **backend:** replace blocking functions with async functions ([#270](https://github.com/ravenclaw900/DietPi-Dashboard/issues/270)) ([aa6e69f](https://github.com/ravenclaw900/DietPi-Dashboard/commit/aa6e69fcddb78af1a3b25bc9a08f8249447ca406))
* **dashboard:** fix error about cpu temp when changing nodes ([495fe5e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/495fe5e48113613f957ba47693c32cf6d9b840f0))
* **dashboard:** fix typescript errors ([0c1934c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/0c1934c2ccb88eed2c2286c6fac63c106e892488))
* **deps:** update rust crate serde to 1.0.140 ([#309](https://github.com/ravenclaw900/DietPi-Dashboard/issues/309)) ([0dc7565](https://github.com/ravenclaw900/DietPi-Dashboard/commit/0dc756501283f23d04f4947c0b61fd4b25f30fb5))
* **deps:** update rust crate tracing-subscriber to 0.3.15 ([#310](https://github.com/ravenclaw900/DietPi-Dashboard/issues/310)) ([79e8be4](https://github.com/ravenclaw900/DietPi-Dashboard/commit/79e8be473e97a2ad7bef2d12be8aa677d678ba57))
* **filebrowser:** fix "couldn't get parent of path" error ([7817b65](https://github.com/ravenclaw900/DietPi-Dashboard/commit/7817b655db767ff1f4cf9b91df253c389069287e))
* **filebrowser:** give file editor black background on dark mode without syntax highlighting ([07be869](https://github.com/ravenclaw900/DietPi-Dashboard/commit/07be869c97418e1c1a7fa42081438d6a8ac05d7f)), closes [#218](https://github.com/ravenclaw900/DietPi-Dashboard/issues/218)
* **frontend:** make websocket reconnect ([#325](https://github.com/ravenclaw900/DietPi-Dashboard/issues/325)) ([0db5fb0](https://github.com/ravenclaw900/DietPi-Dashboard/commit/0db5fb0a842e126a12a0f484c43fe58e819776d1))
* **services:** fix dashboard sometimes crashing when reloading services page ([657f5e9](https://github.com/ravenclaw900/DietPi-Dashboard/commit/657f5e964b55b482dd99402069d8df9239bd0d97))

## [0.6.0](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.5.0...v0.6.0) (2022-04-08)


### Features

* **backend:** add customizable token timeout ([30a3477](https://github.com/ravenclaw900/DietPi-Dashboard/commit/30a3477ba017c01417033d81e51215fbe1d5132d))
* **backend:** add privacy HTTP headers ([52f8e16](https://github.com/ravenclaw900/DietPi-Dashboard/commit/52f8e1643858e479b29b46e44acf4bcb8b056e36))
* **backend:** allow to set log level via config file ([#182](https://github.com/ravenclaw900/DietPi-Dashboard/issues/182)) ([5643d51](https://github.com/ravenclaw900/DietPi-Dashboard/commit/5643d5194734f8fd2bacd062d7c5f2c1ca852973))
* **backend:** allow using environment variables to set settings ([#186](https://github.com/ravenclaw900/DietPi-Dashboard/issues/186)) ([9b80ade](https://github.com/ravenclaw900/DietPi-Dashboard/commit/9b80ade5211ac247ecd43419d592fa807b1281c2))
* **frontend:** check for updates ([3abb628](https://github.com/ravenclaw900/DietPi-Dashboard/commit/3abb6285299c4778ce484474e2e0c5c18841b9b5))
* **frontend:** warn if backend and frontend nodes have different versions ([c9eaba6](https://github.com/ravenclaw900/DietPi-Dashboard/commit/c9eaba621e8df919dcd70b09745fb4a98e9f00d8))
* **terminal:** allow setting login user ([#202](https://github.com/ravenclaw900/DietPi-Dashboard/issues/202)) ([fdb4891](https://github.com/ravenclaw900/DietPi-Dashboard/commit/fdb48913886614018ad0638aa1df4fd628cd1db7))


### Bug Fixes

* **backend:** fix clippy errors ([bb5b9cc](https://github.com/ravenclaw900/DietPi-Dashboard/commit/bb5b9cc9b0badd5bb9f9973c9dbf8f19b53cd5a7))
* **backend:** fix clippy lints ([b00ca1c](https://github.com/ravenclaw900/DietPi-Dashboard/commit/b00ca1c6e73a226c95ac359d48ee8da0028e56e6))
* **backend:** fix github svg not showing up ([169467d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/169467d2a06314387a5ff47ca30fe9514cd63fe8))
* **backend:** fix login dialog not showing up on page change ([979655b](https://github.com/ravenclaw900/DietPi-Dashboard/commit/979655bb0594d8a6fbb6273c8fd25d75882643c7))
* **backend:** notify only waiting pages to quit ([d074ca1](https://github.com/ravenclaw900/DietPi-Dashboard/commit/d074ca188ce0fe4512c911c946bfeeef6cc46540))
* **config:** use correct config file path ([f08fabe](https://github.com/ravenclaw900/DietPi-Dashboard/commit/f08fabe2f457ae0bce59d2577026d890a93125cd))
* **filebrowser:** don't allow viewing text files larger than 2MB ([63c8527](https://github.com/ravenclaw900/DietPi-Dashboard/commit/63c852703e56a4a03ec4b8d890cf46608781db24))
* **filebrowser:** fix angle brackets and ampersands with syntax highlighting ([f850397](https://github.com/ravenclaw900/DietPi-Dashboard/commit/f8503978151b4f090b39a7fda1ea6a8a03ba38e1))
* **filebrowser:** fix syntax highlighting ([3742917](https://github.com/ravenclaw900/DietPi-Dashboard/commit/37429171ad88e97749af65f305dca400ca864640))
* **filebrowser:** highlight files and directories when clicked ([7ce6eba](https://github.com/ravenclaw900/DietPi-Dashboard/commit/7ce6ebaa19a3fe50dc92f3b6db3ec87db8afb3ae))
* **frontend:** capitalize 'P' in uPlot ([3213cf2](https://github.com/ravenclaw900/DietPi-Dashboard/commit/3213cf285e22c02aa8b1b2282ba8ebb0d69c00b5))
* **frontend:** fix notification text on dark mode ([24e226d](https://github.com/ravenclaw900/DietPi-Dashboard/commit/24e226dfe1efce712a682c20beafa097d22beafc))
* **frontend:** fix update check timer ([523087e](https://github.com/ravenclaw900/DietPi-Dashboard/commit/523087e03fa19ec93de2d710b78eb841246797cd))
* **frontend:** increase z-index on login dialog ([4cf65af](https://github.com/ravenclaw900/DietPi-Dashboard/commit/4cf65af4846fee234ff1838d2ee2a4488addf78f))
* **frontend:** use [@import](https://github.com/import) for importing css ([824efbf](https://github.com/ravenclaw900/DietPi-Dashboard/commit/824efbfd66d29c87e743cfcf14a25b1a2b678c4e))
* **software:** fix software page not working with password prompt ([d592538](https://github.com/ravenclaw900/DietPi-Dashboard/commit/d592538cd9a8147e62e175016a0dc3eff2b24194))
* **software:** fix tab colors ([a808c17](https://github.com/ravenclaw900/DietPi-Dashboard/commit/a808c17d4fd64bc39aa8fcbbac7c6b9c8db1f2bc))
* **terminal:** fix password dialog on terminal page ([#198](https://github.com/ravenclaw900/DietPi-Dashboard/issues/198)) ([807f954](https://github.com/ravenclaw900/DietPi-Dashboard/commit/807f954c91139e3378a48f94609494d7342f45e9))

### [0.5.1](https://github.com/ravenclaw900/DietPi-Dashboard/compare/v0.5.0...v0.5.1) (2022-02-13)


### Bug Fixes

* **security:** fix token not being checked on first load ([aed47d9](https://github.com/ravenclaw900/DietPi-Dashboard/commit/aed47d9726349cabb7ab128ec699f4b5cf80e814))

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
