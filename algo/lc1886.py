class lc1886:
    def findRotation(self, mat: List[List[int]], target: List[List[int]]) -> bool:
        rows = len(mat)
        cols = len(mat[0])
        def match(mat: List[List[int]], target: List[List[int]]):  
            for i in range(rows):
                for j in range(cols):
                    if mat[i][j] != target[i][j]:
                        return False
            return True
        def rotate(mat: List[List[int]]):
            for i in range(rows):
                for j in range(i,cols,1):
                    tmp = mat[i][j]
                    mat[i][j] = mat[j][i]
                    mat[j][i] = tmp
            for i in range(rows):
                for j in range(cols//2):
                    tmp = mat[i][j]
                    mat[i][j] = mat[i][cols-1-j]
                    mat[i][cols-1-j] = tmp
        if match(mat, target):
            return True
        for k in range(3):
            rotate(mat)
            for i in range(rows):
                for j in range(cols):
                    print(f"{mat[i][j]} ", end = "")
            if match(mat, target):
                return True
        return False

def main():
    sol = lc1886()
    mat = [[0,0,0],[0,1,0],[1,1,1]]
    target = [[1,1,1],[0,1,0],[0,0,0]]

    output = sol.findRotation(mat, target)
    print(f"output:{output}")

if __name__ == '__main__':
    main()

