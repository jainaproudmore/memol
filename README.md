# Memol

Your task stack

## Install
```
$ cargo install memol
```

## Usage
```sh
Your task stack 0.2.0
LeafChage (https://github.com/LeafChage)

USAGE:
memol <SUBCOMMAND>

OPTIONS:
-h, --help       Print help information
-V, --version    Print version information

SUBCOMMANDS:
all      all your latest task
clear    clear your latest task
help     Print this message or the help of the given subcommand(s)
peek     check your latest task
pop      pop your latest task
push     push your latest task
top      check your latest task
```

## Example
### Push
```sh
memol push "I do something Task1"
memol push "I do something Task2"

memol all
# I do something Task2 (at: 0000000000)
# I do something Task1 (at: 0000000000)

memol push -r "I do something Task3 to first"
memol all
# I do something Task2 (at: 0000000000)
# I do something Task1 (at: 0000000000)
# I do something Task3 to first (at: 0000000000)
```

### POP
```sh
memol all
# I do something Task3 (at: 0000000000)
# I do something Task2 (at: 0000000000)
# I do something Task1 (at: 0000000000)

memol pop
memol all
# I do something Task2 (at: 0000000000)
# I do something Task1 (at: 0000000000)

memol pop -r
memol all
# I do something Task2 (at: 0000000000)
```

### PEEK / TOP and ALL
```sh
memol all
# I do something Task3 (at: 0000000000)
# I do something Task2 (at: 0000000000)
# I do something Task1 (at: 0000000000)

memol all -n 1
# I do something Task3 (at: 0000000000)

memol all -r -n 2
# I do something Task1 (at: 0000000000)
# I do something Task2 (at: 0000000000)

memol peek # memol top
# I do something Task3 (at: 0000000000)

memol peek -r # memol top -r
# I do something Task1 (at: 0000000000)
```

## Recommend
Customize your shell

```sh
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

