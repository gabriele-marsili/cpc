import random

import pytest

from naive_sol import sliding_w_max


def reference(A, k):
    return [max(A[i:i + k]) for i in range(len(A) - k + 1)]


@pytest.mark.parametrize(
    "A, k, expected",
    [
        ([1, 2, 3, 1, 4, 5, 2, 3, 1], 3, [3, 3, 4, 5, 5, 5, 3]),
        ([5], 1, [5]),
        ([1, 3, 2], 3, [3]),
        ([4, 2], 1, [4, 2]),
        ([1, 2, 3], 2, [2, 3]),
    ],
)
def test_known_cases(A, k, expected):
    assert sliding_w_max(A, k) == expected


def test_matches_reference_on_random_inputs():
    rng = random.Random(0)
    for _ in range(200):
        n = rng.randint(1, 12)
        k = rng.randint(1, n)
        A = [rng.randint(-5, 9) for _ in range(n)]
        assert sliding_w_max(A, k) == reference(A, k), (A, k)
