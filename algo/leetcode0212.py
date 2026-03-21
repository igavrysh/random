'''
[
["o","a","b","n"],
["o","t","a","e"]
["a","h","k","r"],
["a","f","l","v"]]
'''

from typing import List

class Trie:
    def __init__(self):
        self.children = {}
        self.words = 0
        self.is_eow = False

    def add(self, s:str):
        node = self
        for i,ch in enumerate(s):
            node.words += 1
            if node.children.get(ch) is None:
                node.children[ch] = Trie()
            next_node = node.children[ch]
            node = next_node
            if i==len(s)-1:
                node.is_eow = True
                node.words += 1
    
    def find(self, i:int, j:int, board: List[List[str]], acc: List[str]):
        dirs = [[-1,0],[1,0],[0,-1],[0,1]]
        rows = len(board)
        cols = len(board[0])
        def dfs(i: int, j: int, node: Trie, path: str, board: List[List[str]], acc: List[str]) -> int:
            ch = board[i][j]
            board[i][j] = "#"
            next_node = node.children.get(ch)
            found_words_counter = 0
            if next_node is not None:
                if next_node.is_eow:
                    next_node.is_eow = False
                    acc.append(path+ch)
                    found_words_counter += 1
                for d in dirs:
                    next_i = i + d[0]
                    next_j = j + d[1]
                    if next_i<0 or next_i>=rows or next_j<0 or next_j>=cols or board[next_i][next_j]=="#" or next_node.words==0:
                        continue
                    found_words_counter += dfs(next_i, next_j, next_node, path+ch, board, acc)
            board[i][j] = ch
            self.words -= found_words_counter
            return found_words_counter
        dfs(i, j, self, "", board, acc)

class Solution:                
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        trie = Trie()
        for w in words:
            trie.add(w)
        rows = len(board)
        cols = len(board[0])
        res = []
        for i in range(rows):
            for j in range(cols):
                trie.find(i, j, board, res)
        return res

def test1():
    board = [
        ["o","a","a","n"],
        ["e","t","a","e"],
        ["i","h","k","r"],
        ["i","f","l","v"]
    ]
    words = ["oath","pea","eat","rain"]
    expected_output = ["eat","oath"]
    sol = Solution()
    output = sol.findWords(board, words)
    print(f"test1: {output == expected_output} output: {output} expected_output: {expected_output}")

def test2():
    board = [
        ["o","a","b","n"],
        ["o","t","a","e"],
        ["a","h","k","r"],
        ["a","f","l","v"]
    ]
    words = ["oa","oaa"]
    expected_output = ["oa","oaa"]
    sol = Solution()
    output = sol.findWords(board, words)
    print(f"test2: {output == expected_output} output: {output} expected_output: {expected_output}")

def main():
   test2()
   test1()

if __name__ == "__main__":
    main()