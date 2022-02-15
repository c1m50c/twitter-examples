def is_even(i: int) -> bool:
    if i == 1:
        return False
    elif i == 2:
        return True
    elif i == 3:
        return False
    elif i == 4:
        return True
    elif i == 5:
        ...

# Never do that! Use one of these instead...

is_even = lambda i : i % 2 == 0
is_even = lambda i : not i & 1
is_odd = lambda i : not is_even(i)