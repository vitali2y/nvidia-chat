# nvidia-chat

## Simple Rust CLI AI chat via Nvidia AI API

```shell
✗ # building
✗ cargo r --release
~...~
✗ # usage
✗ export NVIDIA_API_KEY="nvapi-XXX"  # free from https://build.nvidia.com/settings/api-keys
✗ ./target/release/nvidia-chat "In Rust what is the difference between Fn, FnMut, and FnOnce?"  # simple prompt as a CLI param
~...~
✗ # using prompt as a stdin pipe, e. g. from Markdown file with your code and detailed question
✗ cat my_question.md | ./target/release/nvidia-chat
~...~
✗ # using code2prompt tool for generating basis of your project for further passing generated file to nvidia-chat
✗ code2prompt . -t ./my-simple-prompt.hbs -O my-project-$(date +%y%m%d%H%M).md
~...~
✗ # example of usage together with ripe, Rust pipe editor (https://github.com/vitali2y/ripe)
✗ more -l webext/{{*.json,*.pug,*.js},panel/{panel.js,panel.css,baza.js}} \
    ../server/{bindings/types.d.ts,swagger/openapi.json} | ripe | nvidia-chat
~...~
✗ # example of usage another model
✗ export NVIDIA_MODEL="stepfun-ai/step-3.5-flash"
✗ cat src/main.rs | ripe | ./target/release/nvidia-chat
~...~
✗
```

> [!NOTE]
> *Rust* 🦀 inside!

---

## License

MIT license ([LICENSE](https://github.com/vitali2y/nvidia-chat/blob/main/LICENSE) or <http://opensource.org/licenses/MIT>)
