from typing import (
    List,
)

class SolutionAbbreviations:
    """
    @param word: the given word
    @return: the generalized abbreviations of a word
             we will sort your return value in output
    """
    def generate_abbreviations(self, word: str) -> List[str]:
        # Write your code here
        def bt(start:int, w:str, prev_int: bool) -> List[str]:
            if start == len(w):
                return [""]
            if start > len(w):
                return []
            
            res = []
            res.extend([(w[start] + suff)for suff in bt(start+1, w, False)])
            if not prev_int:
                for i in range(1,10,1):
                    if start+i > len(w):
                        break
                    res.extend([(str(i) + suff) for suff in bt(start+i, w, True)])
            return res
        
        return bt(0,word, False)
    
def main():
    sol = SolutionAbbreviations()
    print(sol.generate_abbreviations("word"))

if __name__ == "__main__":
    main()