# Changelog 

All notable changes to this project will be documented in this file.

## [0.0.8] - 2022-04-25

https://github.com/nlargueze/gitext/compare/0.0.7...0.0.8

### New features

- Added config for types included in the changelog [#68253](https://github.com/nlargueze/gitext/commit/682534cbd1249c407050928e45866eef931a779e)

### Bug fixes

- Fixed commit order in changelog [#14ece](https://github.com/nlargueze/gitext/commit/14ecefe36f0616dbb98ac7c016851d5af5dc0c47)
- Fixed get_tags when there is no tag in the repo [#4b327](https://github.com/nlargueze/gitext/commit/4b3273eb197499a69c91895f0a686cb1e01b0320)
- Fixed install-hooks script [#3480d](https://github.com/nlargueze/gitext/commit/3480daa99da8839374a568aa34abf2d1c295485b)
- Set current directory of custom bump commands [#4a9d5](https://github.com/nlargueze/gitext/commit/4a9d5bf871448061f21549381e7fdb078f342853)

## [0.0.7] - 2022-04-23

https://github.com/nlargueze/gitext/compare/0.0.6...0.0.7

### Bug fixes

- Fixing bump script [#3613d](https://github.com/nlargueze/gitext/commit/3613d5b8f593aae0eaab603cc9513ebc0d6106c8)
- Fixing bumping script [#41823](https://github.com/nlargueze/gitext/commit/4182323dbadf78c81a3b22d0d799fa7a7bea020d)

### Other changes

- Created release 0.0.7 [#ea895](https://github.com/nlargueze/gitext/commit/ea8956fe9eee27464e510fbd78fe98d8c00a5fc6)

## [0.0.6] - 2022-04-23

https://github.com/nlargueze/gitext/compare/0.0.5...0.0.6

### Bug fixes

- Fixing cargo bump script [#452e9](https://github.com/nlargueze/gitext/commit/452e920b605ec512d0c1720d01d5deb209496512)

### Other changes

- Created release 0.0.6 [#3ba9e](https://github.com/nlargueze/gitext/commit/3ba9e3987d5a8f7ae12eee4e53d11839958b2d81)

## [0.0.5] - 2022-04-23

https://github.com/nlargueze/gitext/compare/0.0.4...0.0.5

### Bug fixes

- Fixed Cargo.lock file [#16257](https://github.com/nlargueze/gitext/commit/16257542d1ee8159359ce842b45367549c2bdf60)

### Other changes

- Created release 0.0.5 [#3ef89](https://github.com/nlargueze/gitext/commit/3ef896656b9900bd30dbb8e816c95f1edaee6c3b)

## [0.0.4] - 2022-04-23

https://github.com/nlargueze/gitext/compare/0.0.3...0.0.4

### Bug fixes

- Fixedcustom bumop scripts execution [#c80f9](https://github.com/nlargueze/gitext/commit/c80f9e2b2373bd7e27e94eaaebe6633e7908f05e)

### Other changes

- Created release 0.0.4 [#94083](https://github.com/nlargueze/gitext/commit/940837236985beab7386e00244e3004b8a240a88)

## [0.0.3] - 2022-04-23

https://github.com/nlargueze/gitext/compare/0.0.1...0.0.3

### New features

- Added allow_dirty option to release [#e68c7](https://github.com/nlargueze/gitext/commit/e68c7669bbeb45dbaa7c5ffd6c26f0dac654535b)
- Added option to commit the release explicitly [#37a60](https://github.com/nlargueze/gitext/commit/37a601726076fd54330454cde1a8b6adb12ba734)

### Bug fixes

- Added cargo bump to repo gitx config [#54bc7](https://github.com/nlargueze/gitext/commit/54bc79ea7a08f72e7cf35d21b1c8111fb44abdd9)
- Fixed typo [#351e8](https://github.com/nlargueze/gitext/commit/351e8b277eaefab2d6386145e3250a8eeafe9613)
- Fixed commit body not separated from the subject when fetching [#ad664](https://github.com/nlargueze/gitext/commit/ad664a09a140a65173223e265c21f7202ca78f25)
- Fixed annotated tags not picking up the commit hashes [#814a1](https://github.com/nlargueze/gitext/commit/814a107c7a56985df823bf01a97a024adaf95b2a)
- Config looked up recursively and not created automatically [#0640b](https://github.com/nlargueze/gitext/commit/0640b882c2cfff4d85268438b89ede8a05d0d8eb)

### Code refactoring

- Moved git_log() to another file [#d5d56](https://github.com/nlargueze/gitext/commit/d5d56c7b4d214741a421499a0e9ecae50a481c47)

### Other changes

- Created release 0.0.3 [#c8bbd](https://github.com/nlargueze/gitext/commit/c8bbd30717ad5eb47d0673bfb70db7924b43b35d)

## [0.0.1] - 2022-04-14

### New features

- Added message to hook scripts [#fd538](https://github.com/nlargueze/gitext/commit/fd5382db3e556feef23f08d4aed544602d16a95c)
- Added custom hooks [#a5db2](https://github.com/nlargueze/gitext/commit/a5db2b6a825305e5cfb450499e7756df578356d6)
- Added push workflow to release [#41917](https://github.com/nlargueze/gitext/commit/4191740882ac93ea5d95415b55aed8665bbc8203)
- Added release command [#c61f1](https://github.com/nlargueze/gitext/commit/c61f149ae8d3c1cda3ab5eae8100e5b135e32715)
- Added info line indicating not tagged [#ebdd1](https://github.com/nlargueze/gitext/commit/ebdd1f346cf8f47ff14fc6a02345ff3706dd49c8)
- Added change log commit url [#f3195](https://github.com/nlargueze/gitext/commit/f3195d35ad0645ded0f334e044598c452c7bf919)
- Added changelog generatiom [#560f1](https://github.com/nlargueze/gitext/commit/560f1d7cb76d87c0694e7d361f5603a824a12a76)
- Added lowercase checks [#9cba0](https://github.com/nlargueze/gitext/commit/9cba0ccd57060df83c529048286053566ebed0e6)
- Added commit parsing [#7ba6a](https://github.com/nlargueze/gitext/commit/7ba6a171fea1d8b87e4da7a30e5441b7ff39996c)
- Added ammend option [#b2c2b](https://github.com/nlargueze/gitext/commit/b2c2b0ee9dbc2c09441dfa47bf71531b5f0185f5)
- Removed displayed commit message on push [#eaef3](https://github.com/nlargueze/gitext/commit/eaef3ea5b4c4bbe25de54dc252a8de0a9db36446)
- Added push option to commit [#1e402](https://github.com/nlargueze/gitext/commit/1e4020ce88eed72c992a212f1d09f449b2888b21)

### Bug fixes

- Added extra info to manifest for publishing [#66a63](https://github.com/nlargueze/gitext/commit/66a631d2c0094d54a8bc2144283d8fcfb3829e61)
- Removed file extension on scripts [#24e19](https://github.com/nlargueze/gitext/commit/24e1901f3332dc3d7aa86f5a5799cfdd4897b248)
- Fixed install-hooks [#b96db](https://github.com/nlargueze/gitext/commit/b96db05851ea2a3581a37d7460f609860cb2032c)
- Added git add to release workflow [#fbff2](https://github.com/nlargueze/gitext/commit/fbff27d0f6573514f341192c57faf4146da804ee)
- Fixed changelog [#c5356](https://github.com/nlargueze/gitext/commit/c5356ecf6747b95622468da90d2700299c27a1a9)
- Modified bump option &#x27;print-only&#x27; -&gt; &#x27;tag&#x27; [#815e4](https://github.com/nlargueze/gitext/commit/815e4390e0bd14436f06496cfc383e3fdd74f4ef)
- Fixed bump abort if no commits [#1dcaa](https://github.com/nlargueze/gitext/commit/1dcaaeeb0c233a63f2bcecad587cc0aa4ab8189a)
- Changed bump option &#x27;set&#x27; to &#x27;print-only&#x27; [#01441](https://github.com/nlargueze/gitext/commit/01441ffdd7750f879ee7209ed6a8804ff29675e5)
- Fixed incorrect group title [#f21b4](https://github.com/nlargueze/gitext/commit/f21b4926e4b0fc2a9cb3e7718bbe1bbdadf563c3)
- Fixed lint issue [#10714](https://github.com/nlargueze/gitext/commit/1071470fda7e26a005194c415ab5021cf763e35c)
- Modified config object [#aec84](https://github.com/nlargueze/gitext/commit/aec84640d7349d82958fd442912d57a87a3e9bec)
- Added misc changes [#aeba0](https://github.com/nlargueze/gitext/commit/aeba0061f8f31bea84068fc45afa34d6cb85a561)
- Added misc stuff [#eed1e](https://github.com/nlargueze/gitext/commit/eed1ef301ca9edd4cea2ebcbbbc06a25e37d5ddf)
- Misc changes [#b799a](https://github.com/nlargueze/gitext/commit/b799a06fbbd0adbc1a5248c27c25816a7557fca4)
- Remove gitt file from root [#b869a](https://github.com/nlargueze/gitext/commit/b869a54446ca1a46d377113b0b8edacc82747bae)
- Misc fixes [#20903](https://github.com/nlargueze/gitext/commit/20903f5d49378822dcbdcd5d3a1f59c8ad627179)
- Added stdout for git add wrapper [#c3a86](https://github.com/nlargueze/gitext/commit/c3a868da0762255a6090daabb69ca86c8ef73785)
- Added unix exit codes [#55d79](https://github.com/nlargueze/gitext/commit/55d7982fccb748b2291877053817a2c6d2387d8b)
- Removed displayed commit message on push [#3dd75](https://github.com/nlargueze/gitext/commit/3dd7532b9225bc5bbc183904999589c15a7762d5)
- Removed commit message on push [#b8ea7](https://github.com/nlargueze/gitext/commit/b8ea739cc4bdc1096f3f203d0a2c14d3bf7f776c)
- Removed print message [#e8cc4](https://github.com/nlargueze/gitext/commit/e8cc423394d4e677c6ecb8e43c76819505db5082)

### Code refactoring

- Separated commands in separate binaries [#ceb39](https://github.com/nlargueze/gitext/commit/ceb390d8fc9f61fd5003f0b55b61cc297a7fc72c)
- Changed repo/create name [#847ae](https://github.com/nlargueze/gitext/commit/847ae58b8086af81db3776e3d964978889f9f982)
- Changed repo/create name [#c27bd](https://github.com/nlargueze/gitext/commit/c27bd2265433694924fb3aca226867e1425e5b2f)
- Refactored all code [#04daa](https://github.com/nlargueze/gitext/commit/04daa20351fdec40264a8bb53531231f81167c88)
- Refactored a bunch of stuff [#ae9db](https://github.com/nlargueze/gitext/commit/ae9dbd630c48b20e9ff0e6f0c2d24f4eda5e1c57)

### Other changes

- Created release 0.0.1 [#9e8ce](https://github.com/nlargueze/gitext/commit/9e8ce4124d7ffd89759533a931c91880192740f4)
- Created release 0.0.1 [#bdebf](https://github.com/nlargueze/gitext/commit/bdebfb546bdbe100c0ba9b2b3ffb539dd0bb0e78)
- Changed option from &#x27;add&#x27; to &#x27;set&#x27; [#7a672](https://github.com/nlargueze/gitext/commit/7a672f5742364c8db09bc349a2f56dc5fdbc936d)
- Testing commit command [#01a43](https://github.com/nlargueze/gitext/commit/01a435ef093dde85af2f306e4e3419b479e4f260)

