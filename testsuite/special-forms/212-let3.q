; This tests nested scopes
(def a 1)
(def b 2)
(list
  (let (a 10
        c 30)
    (list a b c))
  (list a b))