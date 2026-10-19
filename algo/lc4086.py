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
        #print(f'availvals:{availvals}')
        #print(f'tvals:{tvals}')

        ptr=0
        used_intervals=0
        curr_cover = tvals[0][0]-1
        n_a = len(availvals)

        for i in range(len(tvals)):
            t_start = tvals[i][0]
            t_end = tvals[i][1]

            if curr_cover >= t_end:
                continue

            if curr_cover < t_start:
                curr_cover = t_start

            used = False
            while ptr<n_a and curr_cover < t_end or (curr_cover == t_end and not used):
                best_end = curr_cover
                while ptr<n_a and availvals[ptr][0] <= curr_cover:
                    best_end = max(best_end, availvals[ptr][1])
                    ptr += 1
                    used = True
                if not used:
                    return -1

                curr_cover = best_end
                used_intervals += 1

                if ptr<n_a and availvals[ptr][0] > curr_cover and curr_cover < t_end:
                    return -1

            if curr_cover < t_end or (curr_cover==t_end and not used):
                return -1

        return used_intervals

def test_tle2():
    availableIntervals = [[1,2],[3,4]]
    targetIntervals = [[1,4]]
    exp_output = -1
    sol = lc4086()
    output = sol.minSelection(availvals=availableIntervals, tvals=targetIntervals)
    passed = output == exp_output
    print(f"test_tle2: {'passed' if passed else 'failed'} output: {output} exp_output: {exp_output}")

def test_tle():
    availableIntervals = [[4,6],[1,4],[2,5]]
    targetIntervals = [[3,4],[2,5],[6,7]]
    exp_output = -1
    sol = lc4086()
    output = sol.minSelection(availvals=availableIntervals, tvals=targetIntervals)
    passed = output == exp_output
    print(f"test_tle: {'passed' if passed else 'failed'} output: {output} exp_output: {exp_output}")

def test326():
    availableIntervals = [[3,29],[2,2],[30,30]]
    targetIntervals = [[3,9],[2,2],[28,28],[7,7],[20,25],[5,28],[29,29],[16,16],[10,10],[16,29],[23,23],[20,20],[12,12],[11,11],[26,26],[30,30],[3,3],[13,13],[21,21]]
    exp_output = 3
    sol = lc4086()
    output = sol.minSelection(availvals=availableIntervals, tvals=targetIntervals)
    passed = output == exp_output
    print(f"test_326: {'passed' if passed else 'failed'} output: {output} exp_output: {exp_output}")

def test01():
    availableIntervals = [[1,4],[1,7],[3,9],[5,10],[10,15]]
    targetIntervals = [[1,3],[2,5],[7,8],[10,13],[11,15],[12,13]]
    exp_output = 3
    sol = lc4086()
    output = sol.minSelection(availvals=availableIntervals, tvals=targetIntervals)

    passed = output == exp_output
    print(f"test_01: {'passed' if passed else 'failed'} output: {output} exp_output: {exp_output}")

def main():
    test01()
    test_tle2()
    test_tle()
    test326()


if __name__ == "__main__":
    main()