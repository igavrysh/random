class lc4085:
    def findColoredRoot(self, edges: list[list[int]], colors: str) -> int:
        n = len(colors)
        colors = list(colors)
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
        def dfs(v: int, depth: int, G: dict[int, set[int]], visited:list[bool], counter: list[dict[str, list[int]]]):
            for neigh in G[v]:
                if visited[neigh]:
                    continue
                visited[neigh] = True
                key = colors[v]+colors[neigh]
                counter[0][key][depth%3] += 1
                dfs(neigh, depth+1, G, visited, counter)
        visited[start] = True
        dfs(start, 0, G, visited, counter)
        print(f'counter: {counter}')

        def check(v: int, counter: list[dict[str, list[int]]]) -> bool:
            offset = root_to_idx[colors[v]]
            for apprdep in range(2):
                for key in counter[apprdep]:
                    frm = root_to_idx[key[0]]
                    allowed_positive = (offset + frm + 3)%3
                    for mod3_depth in range(3): # [depth%3==0, depth%3==1, depth%3==2]
                        if mod3_depth != allowed_positive and counter[apprdep][key][mod3_depth]>0:
                            return False
                        offset = root
            return True

        # def dfs2(v: int, depth: int, G: dict[int, set[int]], visited:list[bool], counter: list[dict[str, list[int]]], res: list[int]):
        #     if len(res)>0:
        #         return
            
        #     if check(v, counter):
        #         res.append(v)
        #         return

        #     for neigh in G[v]:
        #         if visited[neigh]:
        #             continue
        #         visited[neigh] = True
        #         old_key = colors[v] + colors[neigh]
        #         new_key = colors[neigh] + colors[v]
        #         new_counter = {}
        #         for apprdep in range(2):

        #         counter[0][old_key][0] -= 1
        #         counter[1][new_key][0] += 1
        return -1

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