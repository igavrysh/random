from collections import deque
from typing import List
class TrieNode:
    def __init__(self, is_eow = False, min_len = -1, min_len_idx = -1):
        self.ch = [None] * 26
        self.is_eow = is_eow
        self.min_len = min_len
        self.min_len_idx = min_len_idx
class Solution3093:
    def stringIndices(self, wordsContainer: List[str], wordsQuery: List[str]) -> List[int]:
        def addWord(trie: TrieNode, word: str, word_idx: int):
            node = trie
            if trie.min_len == -1 or (trie.min_len > len(word) or (trie.min_len == len(word) and trie.min_len_idx == word_idx)):
                trie.min_len = len(word)
                trie.min_len_idx = word_idx 
            for (i, ch) in enumerate(word[::-1]):
                idx = ord(ch) - ord('a')
                if node.ch[idx] is None:
                    node.ch[idx] = TrieNode()
                    node.ch[idx].min_len = len(word)
                    node.ch[idx].min_len_idx = word_idx
                else:
                    if node.ch[idx].min_len > len(word) or (node.ch[idx].min_len == len(word) \
                        and node.ch[idx].min_len_idx > word_idx):
                        node.ch[idx].min_len = len(word)
                        node.ch[idx].min_len_idx = word_idx
                node = node.ch[idx]
        
        def findMaxMatchNode(trie: TrieNode, word: str):
            node = trie
            for (i, ch) in enumerate(word[::-1]):
                idx = ord(ch) - ord('a')
                if node.ch[idx] is None:
                    break
                node = node.ch[idx]
            return node
        
        trie = TrieNode()
        for (i, word) in enumerate(wordsContainer):
            addWord(trie, word, i)
        res = [-1] * len(wordsQuery)
        for (i, word) in enumerate(wordsQuery):
            node = findMaxMatchNode(trie, word)
            res[i] = node.min_len_idx
        return res

def test01():
    wordsContainer = ["abcd","bcd","xbcd"]
    wordsQuery = ["cd","bcd","xyz"]
    sol = Solution3093()
    res = sol.stringIndices(wordsContainer, wordsQuery)
    exp_res = [1,1,1]
    passed = res == exp_res
    print(f"test_01: {"passed" if passed else "failed"}")
    
def main():
    test01()

if __name__ == "__main__":
    main()