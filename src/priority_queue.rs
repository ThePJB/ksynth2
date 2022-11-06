pub struct PriorityQueue<K: Ord + Clone, V: Clone,> {
    heap: Vec<(K, V)>,
}

impl <K: Ord + Clone,V: Clone> PriorityQueue<K, V> {
    pub fn new() -> PriorityQueue<K, V> {
        PriorityQueue { heap: Vec::new() }
    }

    pub fn push(&mut self, k: K, v: V) {
        self.heap.push((k, v));
        self.upheap(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.heap.len() == 0 {
            return None;
        }
        let return_val = self.heap[0].clone();
        self.heap[0] = self.heap[self.heap.len() - 1].clone();
        self.heap.truncate(self.heap.len() - 1);
        self.downheap(0);
        Some(return_val)
    }

    fn downheap(&mut self, mut idx: usize) {
        loop {
            let l = idx * 2 + 1;
            let r = idx * 2 + 2;

            if r < self.heap.len() {
                if self.heap[l].0 < self.heap[r].0 {
                    if self.heap[l].0 < self.heap[idx].0 {
                        self.heap.swap(l, idx);
                        idx = l;
                    } else {
                        break;
                    }
                } else {
                    if self.heap[r].0 < self.heap[idx].0 {
                        self.heap.swap(r, idx);
                        idx = r;
                    } else {
                        break;
                    }
                }
            } else if l < self.heap.len() {
                if self.heap[l].0 < self.heap[idx].0 {
                    self.heap.swap(l, idx);
                    idx = l;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    
    fn upheap(&mut self, mut idx: usize) {
        let parent = idx / 2;
        while self.heap[parent].0 < self.heap[idx].0 {
            self.heap.swap(idx, parent);
            if parent == 0 {
                break;
            }
            idx = parent;
        }
    }
}