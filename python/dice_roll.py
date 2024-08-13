from itertools import repeat
import multiprocessing
import random


def task(_):
    return sum((not random.getrandbits(2) for x in repeat(None, 231)))


def main():
    with multiprocessing.Pool() as pool:
        largest_paralysis_count = max(
            pool.imap_unordered(task, repeat(None, 1_000_000), int(1_000_000 / 1000))
        )
    print("Largest number of paralyses:", largest_paralysis_count)


if __name__ == "__main__":
    main()
