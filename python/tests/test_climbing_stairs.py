from practice_problems.climbing_stairs import Solution


def test_climbStairs():
    assert Solution().climbStairs(1) == 1
    assert Solution().climbStairs(2) == 2
    assert Solution().climbStairs(3) == 3
    assert Solution().climbStairs(4) == 5
    assert Solution().climbStairs(5) == 8
    assert Solution().climbStairs(6) == 13
    assert Solution().climbStairs(7) == 21
    assert Solution().climbStairs(45) == 1836311903
