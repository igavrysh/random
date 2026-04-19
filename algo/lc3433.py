from functools import cmp_to_key
from typing import List

class Solution:
    def countMentions(self, numberOfUsers: int, events: List[List[str]]) -> List[int]:
        n = numberOfUsers
        def user_idx(userid:str)->int:
            return int(userid[2:])

        priority = set(["ONLINE","OFFLINE"])
        def compare(e1, e2):
            if e1[1]==e2[1]:
                if e1[0] in priority:
                    return -1
                return 1
            return e1[1]-e2[1]
        events = [[e[0], int(e[1]), e[2]] for e in events]
        online_toadd = []
        for e in events:
            if e[0]=="OFFLINE":
                online_toadd.append(["ONLINE", e[1]+60, e[2]])
        events.extend(online_toadd)
        events = sorted(events, key=cmp_to_key(compare))
        print(f"events:{events}")
        online = [True] * n
        mentions = [0] * n
        for e in events:
            if e[0] == "MESSAGE":
                if e[2] == "ALL":
                    for i in range(n):
                        mentions[i] += 1
                elif e[2] == "HERE":
                    for i in range(n):
                        if online[i]:
                            mentions[i] += 1
                else:
                    for userid in e[2].split():
                        idx = user_idx(userid)
                        if online[idx]:
                            mentions[idx]+=1
            elif e[0] == "ONLINE":
                idx = int(e[2])
                online[idx] = True
            elif e[0] == "OFFLINE":
                idx = int(e[2])
                online[idx] = False
        return mentions

def main():
    sol = Solution()
    users = 3
    events = [["MESSAGE","1","id0 id1"],["MESSAGE","5","id2"],["MESSAGE","6","ALL"],["OFFLINE","5","2"]]

    output = sol.countMentions(users, events)

if __name__ == '__main__':
    main()