from scipy.stats import hypergeom


def aggregation(population: int, success: int, draws: int, reward: int) -> float:
    end: int
    if success < draws:
        end = success + 1
    else:
        end = draws + 1

    accum: float = 0.0
    for k in range(1, end):
        tmp = hypergeom.pmf(k, population, success, draws)
        tmp *= (reward * k)
        accum += tmp

    return accum


def main():
    for d in range(1, 100):
        expected = aggregation(100, 4, d, 6000)
        ratio = expected / (300 * d)
        print(f'd:{d} exp:{expected:.2f} ratio:{ratio:.2%}')


if __name__ == "__main__":
    main()
