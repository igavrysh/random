from typing import List

class lc324:
    def wiggleSort(self, nums: List[int]) -> None:
        nums.sort()
        a = [0] * len(nums)
        i = 0
        j = len(nums)-1
        idx = 0
        print(f"nums:{nums}")
        while i < j:
            a[idx] = nums[i]
            i += 1
            idx += 1
            a[idx] = nums[j]
            j -= 1
            idx += 1
        nums = a

def test01():
    nums = [1,5,1,1,6,4]
    exp_output = [1,6,1,5,1,4]
    sol = lc324()
    output = sol.wiggleSort(nums)
    passed = output == exp_output
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
    test01()

if __name__ == "__main__":
    main()