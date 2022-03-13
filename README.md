# Memol

Your task stack

## Install
```
$ cargo install memol
```

## Usage
```sh
$ memol --help

Your task stack 0.1.0
LeafChage (https://github.com/LeafChage)

USAGE:
    memol <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    peek    check your latest task
    pop     pop your latest task
    push    push your latest task
    top     check your latest task
```

## Example
```
$ memol push "I do something1"

$ memol push "I do something2"

$ memol peek
> I do something2 (at: 0000000000)

$ memol pop

$ memol peek
> I do something1 (at: 0000000000)
```

## Recommend
Customize your shell

```bash
###
### for example
###

PS1_TASK=""
task() {
    memol peek | sed "s/ (at [0-9]*)//g"
}
PS1_TASK='$(task)'

export PS1="$PS1_TASK\n >"
```

