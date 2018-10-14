;;;;;;;;;; Routines

(def defn
  (macro (name args &rest exps)
    (list 'def name
      (list 'fn args
        (cons 'begin exps)))))

(def defmacro
  (macro (name args body)
      (list 'def name
        (list 'macro args
            body))))

;;;;;;;;;; Math

(defn <= (a b)
  (or (< a b) (= a b)))

(defn >= (a b)
  (or (> a b) (= a b)))

;;;;;;;;;; Logic

(def else true)

(defn binary-or (a b)
  (cond
    (= a true) true
    (= b true) true
    else       false))

(defn binary-and (a b)
  (if (= a true)
    (if (= b true)
      true
      false)
    false))

(defn or (&rest xs)
  (foldl binary-or false xs))

(defn and (&rest xs)
  (foldl binary-and true xs))

(defn not (b)
  (cond
    (= b false) true
    (= b true)  false
    else        false))

;;;;;;;;;; Higher Order Procedures

(defn foldl (f init xs)
  (if (= xs '())
    init
    (foldl f
           (f init (first xs))
           (rest xs))))

(defn reverse (xs)
  (foldl (fn (acc x) (cons x acc)) '() xs))

(defn map (f l)
  (let (loop (fn (accum xs)
               (if (empty? xs)
                 accum
                 (loop (cons (f (first xs)) accum)
                       (rest xs)))))
    (loop '() (reverse l))))

;;;;;;;;;; Type Predicates

(defn list? (n)
  (= (typeof n) 'list))

(defn char? (n)
  (= (typeof n) 'char))

(defn symbol? (n)
  (= (typeof n) 'symbol))

(defn number? (n)
  (= (typeof n) 'number))

(defn procedure? (n)
  (= (typeof n) 'procedure))

(defn macro? (n)
  (= (typeof n) 'macro))

(defn environment? (n)
  (= (typeof n) 'environment))

(defn primitive? (n)
  (= (typeof n) 'primitive))

(defn string? (n)
  (= (typeof n) 'string))

(defn atom? (n)
  (not (list? n)))

(defn empty? (n)
  (cond (= n '()) true
        (= n "") true
        (= n nil) true
        else false))

(defn boolean? (n)
  (cond (= n true) true
        (= n false) true
        else false))

;; (if (= a b) (typeof a) (typeof b))
;; =>
;; (cond (= a b) (typeof a)
;;       true    (typeof b))
(defmacro if2 (condition consequent alternative)
  (list 'cond condition consequent
              true      alternative))

;;;;;;;;;;

"Prelude version 2018-10-13"