from typing import Optional
from typing import List

class ListNode:
     def __init__(self, val=0, next=None):
         self.val = val
         self.next = next

class Solution:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def sort_list(head: ListNode, trailing: Optional[ListNode]) -> List[ListNode]:
            if head.next == trailing:
                return [head, head]
            middle = split(head, trailing)
            [new_h1, new_t1] = sort_list(head, middle)
            [new_h2, new_t2] = sort_list(middle, trailing)
            new_t1.next = new_h2
            new_t2.next = trailing
            res = merge(new_h1, new_h2, trailing)
            return res

        def merge(h1: ListNode, h2: ListNode, trailing: ListNode) -> List[ListNode]:
            senti = ListNode(-1)
            ptr = senti
            p1 = h1
            p2 = h2
            while p2 != trailing or p1 != h2:
                if p2 == trailing or (p1 != h2 and p1.val <= p2.val):
                    ptr.next = p1
                    p1 = p1.next
                    ptr = ptr.next
                else:
                    ptr.next = p2
                    p2 = p2.next
                    ptr = ptr.next
            ptr.next = trailing
            return [senti.next, ptr]

        def split(head: ListNode, trailing: Optional[ListNode]) -> ListNode:
            fast = head
            slow = head
            while fast != trailing:
                fast = fast.next
                slow = slow.next
                if fast != trailing:
                    fast = fast.next
            return slow

        if head is None:
            return None
        [head, _] = sort_list(head, None)
        return head

def test_00():
    list1 = ListNode(4, ListNode(2, ListNode(1, ListNode(3))))
    res = Solution()
    head = res.sortList(list1)
    while head is not None:
        print(f"({head.val})->", end="")
        head = head.next

def test_02():
    # 3,4,0
    list1 = ListNode(3, ListNode(4, ListNode(0)))
    res = Solution()
    head = res.sortList(list1)
    while head is not None:
        print(f"({head.val})->", end="")
        head = head.next

def test_01():
    # -1,5,3,4,0
    list1 = ListNode(-1, ListNode(5, ListNode(3, ListNode(4, ListNode(0)))))
    res = Solution()
    head = res.sortList(list1)
    while head is not None:
        print(f"({head.val})->", end="")
        head = head.next

def test_03():
    list1 = ListNode(1, ListNode(2, ListNode(3, ListNode(4))))
    res = Solution()
    head = res.sortList(list1)
    while head is not None:
        print(f"({head.val})->", end="")
        head = head.next

def main():
    test_03()

if __name__ == "__main__":
    main()