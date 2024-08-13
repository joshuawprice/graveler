from itertools import repeat
import random


def main():
    largest_paralysis_count = max(
        (
            sum((not random.getrandbits(2) for _ in repeat(None, 231)))
            for _ in repeat(None, 1_000_000)
        )
    )
    print("Largest number of paralyses:", largest_paralysis_count)


if __name__ == "__main__":
    main()
