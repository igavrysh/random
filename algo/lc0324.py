from typing import List

class lc324:
    def wiggleSort(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        nums.sort()
        n = len(nums)
        i = n-1
        j = n // 2
        a = [0] * n
        idx = 0
        while idx < n:
            a[idx] = nums[j]
            idx += 1
            j -= 1
            if idx < 0:
                break
            a[idx] = nums[i]
            idx += 1
            i -= 1
        for i in range(n):
            nums[i] = a[i]

def test01():
    nums = [1,5,1,1,6,4]
    exp_output = [4,6,1,5,1,4]
    sol = lc324()
    sol.wiggleSort(nums)
    passed = nums == exp_output
    print(f"test_02 " + ("passed" if passed else "failed"))

def main():
    test01()

if __name__ == "__main__":
    main()