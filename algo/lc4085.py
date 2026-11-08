from typing import List
from collections import defaultdict

class lc4085:
    def findColoredRoot(self, edges: list[list[int]], cs: str) -> int:
        n = len(cs)
        colors = list(cs)
        G = {}
        degree = [0]*n
        for i in range(n):
            G[i] = set()
        combs = {"RB":0,"BW":1,"WR":2,"BR":3,"WB":4,"RW":5}
        root_to_idx = {"R":0,"B":1,"W":2}
        for e in edges:
            key = colors[e[0]]+colors[e[1]]
            if key not in combs:
                return -1
            G[e[0]].add(e[1])
            degree[e[0]] += 1
            G[e[1]].add(e[0])
            degree[e[1]] += 1
        start = -1
        for i in range(n):
            if start==-1 and degree[i]==1:
                start = i
            if degree[i]>=4:
                return -1
        if start==-1:
            return -1
        visited = [False]*n
        counter = []
        for _ in range(2): # 0 to visit, 1 visited
            d = {}
            counter.append(d)
            for key in combs:
                d[key] = [0]*3  # depth mod 3 as index, freq as value

        def check(v: int, counter: dict[str, list[int]]) -> bool:
            offset = root_to_idx[colors[v]]
            for key in counter:
                frm = root_to_idx[key[0]]
                allowed_positive = (offset + frm + 3)%3
                for mod3_depth in range(3): # [depth%3==0, depth%3==1, depth%3==2]
                    if mod3_depth != allowed_positive and counter[key][mod3_depth]>0:
                        return False
            return True

        def dfs(v: int, depth: int, G: dict[int, set[int]], visited:list[bool], counter: dict[str, list[int]], res: list[int]) -> dict[str, list[int]]:
            if len(res)>0:
                return  defaultdict(list)
            counter_out = defaultdict(list)
            frm = colors[v]
            for neigh_v in G[v]:
                if visited[neigh_v]:
                    continue
                visited[neigh_v] = True
                to = colors[neigh_v]
                counter_to_pass =  defaultdict(list)
                if len(counter_to_pass[to+frm])==0:
                    counter_to_pass[to+frm] = [0]*3
                counter_to_pass[to+frm][0] += 1
                for key in counter:
                    for d in range(3):
                        counter_to_pass[key][(d+1)%3] = counter_to_pass[key][d%3]
                counter_res = dfs(neigh_v, depth+1, G, visited, counter_to_pass, res)
                if len(res)>0:
                    return defaultdict(list)
                if len(counter_out[frm+to])==0:
                    counter_out[frm+to] = [0]*3
                counter_out[frm+to][0] += 1
                for key in counter_res:
                    for d in range(3):
                        counter_out[key][(d+1)%3] += counter_res[key][d%3]
            if degree[v]<=2 and len(res)==0 and check(v, counter_out) and check(v, counter):
                res.append(v)
                return defaultdict(list)
            return counter_out

        res = []
        visited = [False] * n
        visited[start] = True
        counter = defaultdict(list)
        dfs(start, 0, G, visited, counter, res)

        return res[0] if len(res)>0 else -1

def test_01():
    edges = [[3,1],[3,2],[1,4],[2,0]]
    colors = "BRRWB"
    exp_output = 3
    sol = lc4085()
    output = sol.findColoredRoot(edges, colors)
    passed = "passed" if output == exp_output else "failed"
    print(f"test_01:{passed} output:{output} exp_output:{exp_output}")

def main():
    test_01()

if __name__ == "__main__":
    main()