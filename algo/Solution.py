from typing import List

from typing import (
    List,
)

class Solution:
    def isToeplitzMatrix(self, matrix: List[List[int]]) -> bool:
        rows = len(matrix)
        cols = len(matrix[0])
        for i in range(0,rows,1):
            for j in range(1,rows-i,1):
                print(f'[{i}:{j}]')
                if matrix[i][0] != matrix[i+j][j]:
                    return False
            print(f'new i:{i}')
        for j in range(1,cols,1):
            for i in range(1,rows,1):
                if matrix[0][j] != matrix[i][i+j]:
                    return False
        return True

matrix = [[1,2],[2,2]]
print(f'{Solution().isToeplitzMatrix(matrix)}')





