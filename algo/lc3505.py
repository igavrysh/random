from typing import List
import heapq

class lc3505:
    def minOperations(self, nums: List[int], x: int, k: int) -> int:
        n = len(nums)
        def deltas_to_medians(nums: list[int], x: int) -> list[float]:
            def insert_num(num, min_h, max_h, sz_min_h, sz_max_h, sum_min_h, sum_max_h) -> List[int]:
                heapq.heappush_max(max_h, num)
                sz_max_h+=1
                sum_max_h+=num
                if sz_max_h>sz_min_h:
                    val = heapq.heappop_max(max_h)
                    heapq.heappush(min_h, val)
                    sz_max_h-=1
                    sum_max_h-=val
                    sz_min_h+=1
                    sum_min_h+=val
                elif max_h[0]>min_h[0]:
                    val1 = heapq.heappop_max(max_h)
                    val2 = heapq.heappop(min_h)
                    sum_max_h-=val1
                    sum_min_h-=val2
                    heapq.heappush_max(max_h, val2)
                    heapq.heappush(min_h, val1)
                    sum_max_h+=val2
                    sum_min_h+=val1
                return [sz_min_h, sz_max_h,sum_min_h,sum_max_h]
            def clean(heap, outmap):
                while heap and heap[0] in outmap:
                    val = heap[0]
                    heapq.heappop(heap)
                    outmap[val]-=1
                    if outmap[val]==0:
                        del outmap[val]
            min_h = []
            max_h = []
            sum_min_h = 0
            sum_max_h = 0
            for i in range(x):
                [sz_min_h, sz_max_h, sum_min_h, sum_max_h] = insert_num(nums[i], min_h, max_h, len(min_h), len(max_h), sum_min_h, sum_max_h)
                print(f'i:{i} sz_min_h:{sz_min_h}, sz_max_h:{sz_max_h} sum_min_h:{sum_min_h} sum_max_h:{sum_max_h} min_h:{min_h} max_h:{max_h}')
            sz_min_h = len(min_h)
            sz_max_h = len(max_h)
            outmap = {}
            res = [-1]*n
            # before was to calculated median, now sum of abs diffs to "median" (in case even - chose either upper of lower median  as "median" )
            #res[x-1] = (max_h[0]+min_h[0])/2.0 if sz_max_h==sz_min_h else min_h[0]
            res[x-1] = sum_min_h-sz_min_h*min_h[0] + sz_max_h*min_h[0]-sum_max_h
            for i in range(x, n):
                num_out = nums[i-x]
                if num_out not in outmap:
                    outmap[num_out]=0
                outmap[num_out]+=1

                if num_out >= min_h[0]:
                    sz_min_h-=1
                    sum_min_h -= num_out
                else:
                    sz_max_h-=1
                    sum_max_h -= num_out

                clean(min_h, outmap)
                clean(max_h, outmap)

                num_in = nums[i]
                [sz_min_h, sz_max_h, sum_min_h, sum_max_h] = insert_num(num_in, min_h, max_h, sz_min_h, sz_max_h, sum_min_h, sum_max_h)


                
                print(f'i:{i} sz_min_h:{sz_min_h}, sz_max_h:{sz_max_h} sum_min_h:{sum_min_h} sum_max_h:{sum_max_h} min_h:{min_h} max_h:{max_h}')


                # before was to calculated median
                #res[i] = (max_h[0]+min_h[0])/2.0 if sz_max_h==sz_min_h else min_h[0]
                res[i] = sum_min_h-sz_min_h*min_h[0] + sz_max_h*min_h[0]-sum_max_h
            return res
        deltas_to_medians = deltas_to_medians(nums, x)
        print(f'deltas_to_medians: {deltas_to_medians}')
        dp = [[-1]*n for _k in range(k)]
        curr = [0]*n
        for k_ptr in range(k):
            for i in range((k_ptr+1)*x-1, n):
                curr[i] = deltas_to_medians[i] + (dp[k_ptr-1][i-x] if (k_ptr-1 >=0) else 0)
            for i in range((k_ptr+1)*x-1, n):
                dp[k_ptr][i] = min(curr[i], dp[k_ptr][i-1] if ((i-1)>=((k_ptr+1)*x-1)) else curr[i])
        print(f'dp:{dp}')
        if n<k*x:
            return -1
        return dp[-1][-1]

def test01():
    nums = [-3,0,-12,1,-17,-17,11]
    x = 4
    k = 1
    sol = lc3505()
    res = sol.minOperations(nums, x, k)
    exp_res = 16
    passed = res == exp_res
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
    test01()

if __name__ == "__main__":
    main()