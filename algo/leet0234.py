
from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        sen = ListNode(-1)
        sen.next = head
        node = head
        n = 0
        while node is not None:
            n += 1
            node = node.next
        mid = n // 2
        counter = 0
        node = sen
        nxt = sen.next
        sen.next = None

        while counter<mid:
            print(f"counter:{counter} node:{node} nxt:{nxt}")
            nxtnxt = nxt.next
            nxt.next = node
            node = nxt
            nxt = nxtnxt
            counter+=1
        l = node
        r = nxt
        lnext = nxt

        is_pal = True
        
        if n%2==1:
            r = nxt.next
        while r is not None:
            if r.val != l.val:
                is_pal = False
            r = r.next

            tmp = l.next
            l.next = lnext
            lnext = l
            l = tmp
        l.next = lnext
        return is_pal 
    

def main():
    sol = Solution()
    head = ListNode(1, ListNode(2))
    output = sol.isPalindrome(head)
    print(f"output:{output}")

if __name__ == '__main__':
    main()





