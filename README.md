# uqm_cli

[![Build](https://github.com/Samarium150/uqm_cli/actions/workflows/build.yml/badge.svg)](https://github.com/Samarium150/uqm_cli/actions/workflows/build.yml)

A command-line interface tool for decrypting download from
`echo Y29tLnRlbmNlbnQucXFtdXNpYwo= | base64 -d` (the Android version) in batch.

You can find the original web version at [um-react](https://um-react.netlify.app/),
which supports more apps and platforms.
The [crypto library](um_crypto) is also developed by their team.

## Usage

Somehow extract the sqlite database at
`echo L2RhdGEvZGF0YS9jb20udGVuY2VudC5xcW11c2ljL2RhdGFiYXNlcy9wbGF5ZXJfcHJvY2Vzc19kYgo= | base64 -d`,
and provide it as the `--db` option as well as the input directory as the last argument.

See more options with:

```shell
./uqm --help
```

## License

Copyright 2025 Samarium150

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
