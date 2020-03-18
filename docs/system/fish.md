
# Fish shell

# Check variable
From https://stackoverflow.com/questions/47743015/fish-shell-how-to-check-if-a-variable-is-set-empty

- set -q var (note the missing "$" - this uses the variable name) can be used to check if a variable has been set.

- set -q var[1] can be used to check whether the first element of a variable has been assigned (i.e. whether it is non-empty as a list).

- test -n "$var" [fn0] (or [ -n "$var" ]) can be used to check whether a variable expands to a non-empty string (and test -z is the inverse - true if it is empty).

These will be true/false in slightly different circumstances.

When no set var has been performed at all (and it has not been inherited from the parent process), set -q var, set -q var[1] and test -n "$var" will be false, test -z "$var" will be true.

When something like set var has been done (without any additional arguments), set -q var will be true, set -q var[1] will be false.

When something like set var "" has been done, both set versions will be true.

When something like set var "somestring" (or even set var "" "" [fn1]) has been done, the sets will be true and test -z "$var" will be false.

