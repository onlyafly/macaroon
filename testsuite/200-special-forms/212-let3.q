(def a 1)
(def d 4)
(list
  (let (a 10
        b 20
        c 30)
    (list a b c d))
  a
  d)