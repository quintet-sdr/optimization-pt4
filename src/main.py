def bisection_method(f, a, b, epsilon) -> float:
    if f(a) * f(b) >= 0:
        raise ValueError("The function must have opposite signs at a and b.")

    c = a
    while (b - a) / 2 > epsilon:
        c = (a + b) / 2
        if abs(f(c)) < epsilon:
            break
        elif f(c) * f(a) < 0:
            b = c
        else:
            a = c

    return c


def gradient_ascent(f_prime, x0, alpha, iterations) -> tuple:
    x = x0
    for _ in range(iterations):
        x = x + alpha * f_prime(x)
    return x, -x ** 2 + 4 * x + 1


def golden_section_method(f, a, b, epsilon) -> tuple:
    phi = (1 + 5 ** 0.5) / 2
    resphi = 2 - phi

    c = a + resphi * (b - a)
    d = b - resphi * (b - a)

    while abs(b - a) > epsilon:
        if f(c) < f(d):
            b = d
            d = c
            c = a + resphi * (b - a)
        else:
            a = c
            c = d
            d = b - resphi * (b - a)

    xmin = (a + b) / 2
    return xmin, f(xmin)


def f_1(x) -> float:
    return x ** 3 - 6 * x ** 2 + 11 * x - 6


def f_2(x) -> float:
    return (x - 2) ** 2 + 3


def f_3_prime(x) -> float:
    return -2 * x + 4


def solve_1() -> None:
    a, b = 1, 20
    epsilon = 1e-6
    root = bisection_method(f_1, a, b, epsilon)
    print(f"Approximate root: {root}")


def solve_2() -> None:
    a, b = 0, 5
    epsilon = 1e-4
    xmin, f_xmin = golden_section_method(f_2, a, b, epsilon)
    print(f"Approximate xmin: {xmin}, f(xmin): {f_xmin}")


def solve_3() -> None:
    x0 = 0
    alpha = 0.1
    iterations = 100
    xmax, f_xmax = gradient_ascent(f_3_prime, x0, alpha, iterations)
    print(f"Approximate xmax: {xmax}, f(xmax): {f_xmax}")


def main() -> None:
    solve_1()
    solve_2()
    solve_3()


if __name__ == "__main__":
    main()
