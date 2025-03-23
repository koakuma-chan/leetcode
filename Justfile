run file:
    rustfmt {{ file }}.rs
    rustc {{ file }}.rs
    ./{{ file }}
    rm {{ file }}
