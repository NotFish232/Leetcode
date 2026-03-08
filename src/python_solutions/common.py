import math
import random
from bisect import *
from collections import *
from heapq import *
from typing import *

from sortedcontainers import *


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
