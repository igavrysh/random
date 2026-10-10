from collections import defaultdict

class lc_weekly_contest_193_2:
    def resilientSubarray(self, nums: list[int], k: int) -> int:
        rems = {}
        res = 1
        l = 0
        total_sum = 0
        for r in range(len(nums)):
            new_rems = {}
            for rem in rems:
                if rem in rems and rems[rem] == 0:
                    continue
                new_rems[(rem + k + nums[r])%k] = rems[rem] if rem in rems else 0
            new_rems[total_sum%k] = 1 + (total_sum%k if (total_sum%k) in new_rems else 0)
            rems = new_rems
            total_sum = (total_sum + nums[r])%k
            print(f'rems1: {rems}')
            while l<=r and (0 not in new_rems or new_rems[0] != r-l+1):
                total_sum = (total_sum + k - nums[l]%k) % k
                rems[total_sum%k] -= 1
                new_rems = {}
                for rem in rems:
                    if rems[rem] == 0:
                        continue
                    new_rems[(rem + k - nums[l]%k)%k] = (rems[rem]-1) if rem in rems else 0
                rems = new_rems
                l += 1
            print(f"l:r:{l},{r} rems: {rems}")
            res = max(res, r-l+1)
        return res

def test01():
    nums = [1,4,7,1]
    k = 3
    sol = lc_weekly_contest_193_2()
    output = sol.resilientSubarray(nums, k)
    passed = output == 4
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
    test01()

if __name__ == "__main__":
    main()