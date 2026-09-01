from collections import deque
from typing import List

class lc3568:
    def minMoves(self, classroom: List[str], energy: int) -> int:
        lts = 0
        st = [-1,-1]
        rows,cols = len(classroom), len(classroom[0])
        l_map = {}
        for i in range(rows):
            for j in range(cols):
                if classroom[i][j] == 'L':
                    l_map[(i,j)] = lts
                    lts += 1
                if classroom[i][j] == 'S':
                    if st != [-1,-1]:
                        return -1
                    st = [i, j]
        total = (2**(lts)) - 1
        #print(f"l_map: {l_map} total: {total}")

        q = deque()
        q.append([st[0],st[1],0,energy,0])
        ## maybe best steps???
        best_energy = [[[-1] * cols for i in range(rows)] for m in range(total+1)]
        best_energy[0][st[0]][st[1]] = energy
        dirs = [[-1,0],[1,0],[0,-1],[0,1]]
        min_steps = -1
        while len(q) != 0:
            [i,j,mask,e,steps] = q.popleft()
            #print(f"current_step: {(i,j)} rows,cols:{(rows,cols)} e:{e}")
            if classroom[i][j] == 'R':
                e = energy

            if mask == total:
                if min_steps == -1:
                    min_steps = steps
                min_steps = min(min_steps, steps)

            if e == 0:
                continue

            for d in dirs:
                next_i = i + d[0]
                next_j = j + d[1]
                if next_i < 0 or next_j < 0 or next_i >= rows or next_j >= cols:
                    continue
                if classroom[next_i][next_j] == 'X':
                    continue

                next_mask = mask
                if (next_i,next_j) in l_map:
                    lt_id = l_map[(next_i,next_j)]
                    next_mask = mask | (1 << lt_id)

                next_e = e - 1
                if best_energy[next_mask][next_i][next_j] == -1 \
                    or (best_energy[next_mask][next_i][next_j] != -1 and best_energy[next_mask][next_i][next_j] < next_e):
                    best_energy[next_mask][next_i][next_j] = next_e
                    # print(f'best_energy: {best_energy}')
                    q.append([next_i, next_j, next_mask, next_e, steps+1])

        return min_steps

def test01():
    classroom = ["S.", "XL"]
    energy = 2
    sol = lc3568()
    res = sol.minMoves(classroom, energy)
    exp_res = 2
    passed = res == exp_res
    print(f"test_01: {'passed' if passed else 'failed'}")

def test02():
    classroom = ["LS", "RL"]
    energy = 4
    sol = lc3568()
    res = sol.minMoves(classroom, energy)
    exp_res = 3
    passed = res == exp_res
    print(f"test_02: {'passed' if passed else 'failed'}")

def test03():
    classroom = ["L.S", "RXL"]
    energy = 3
    sol = lc3568()
    res = sol.minMoves(classroom, energy)
    exp_res = -1
    passed = res == exp_res
    print(f"test_03: {'passed' if passed else 'failed'}")

def main():
    test01()
    test02()
    test03()

if __name__ == "__main__":
    main()