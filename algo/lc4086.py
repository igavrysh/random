from typing import List

class lc4086:
    def minSelection(self, availvals: List[List[int]], tvals: List[List[int]]) -> int:
        tvals.sort()
        tvals_m = []
        curr_s = tvals[0][0]
        curr_e = tvals[0][1]
        for i in range(len(tvals)):
            if tvals[i][0] > curr_e:
                tvals_m.append([curr_s,curr_e])
                curr_s, curr_e = tvals[i][0],tvals[i][1]
            else:
                curr_e = max(curr_e,tvals[i][1])
        tvals_m.append([curr_s,curr_e])
        tvals = tvals_m
        availvals = sorted(availvals, key=lambda intval: (intval[0], -intval[1]))
        print(f'availvals:{availvals}')
        print(f'tvals:{tvals}')

        ptr=0
        used=0
        curr_end = tvals[0][0]

        for i in range(len(tvals)):
            t_curr = max(tvals[i][0], curr_end)
            t_end = tvals[i][1]
            curr_end = max(curr_end, tvals[i][0])
            while ptr<len(availvals) and curr_end < t_end:
                if availvals[ptr][0] <= t_curr and availvals[ptr][1] >= curr_end:
                    used += 1
                    curr_end = availvals[ptr][1]
                ptr += 1
            if curr_end < t_end:
                return -1
        return used

def test01():
    availableIntervals = [[1,4],[1,7],[3,9],[5,10],[10,15]]
    targetIntervals = [[1,3],[2,5],[7,8],[10,13],[11,15],[12,13]]
    exp_output = 3
    sol = lc4086()
    output = sol.minSelection(availvals=availableIntervals, tvals=targetIntervals)

    passed = output == exp_output
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
    test01()

if __name__ == "__main__":
    main()