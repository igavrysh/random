class lc1541:
    def minInsertions(self, s: str) -> int:
        n = len(s)
        open = 0
        inserted = 0
        i = 0
        while i<n:
            if s[i]=='(':
                open+=1
                i+=1
                continue
            if i+1<n and s[i+1]==')':
                if open>0:
                    open-=1
                else:
                    inserted+=1
                i+=2
                continue
            if open>0:
                open-=1
                inserted+=1
            else:
                inserted+=2
            i+=1
        inserted += open*2
        return inserted

