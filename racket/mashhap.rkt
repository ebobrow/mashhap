#lang racket

(provide mashhap-new
         mashhap-set
         mashhap-get
         mashhap-count
         mashhap-delete)

(struct mashhap (entries count))

; empty list = null
; '(0) = tombstone
; '(k v) otherwise

(define (mashhap-new n)
  (mashhap (make-list n '()) 0))

; TODO: resize!!!
(define (mashhap-set map k v)
  (let* ([loc (location-of map k)]
         [count-inc (if (loc-exists? map loc) 0 1)])
    (mashhap (list-set (mashhap-entries map) loc (list k v))
             (+ count-inc (mashhap-count map)))))

(define (mashhap-get map k)
  (let ([el (list-ref (mashhap-entries map) (location-of map k))])
    (if (= 2 (length el))
        (second el)
        #f)))

(define (mashhap-delete map k)
  (let ([loc (location-of map k)])
    (if (loc-exists? map loc)
        (struct-copy mashhap map [entries (list-set (mashhap-entries map) loc '(0))])
        map)))

#| (define (location-of map k) |#
#|   (define search-space |#
#|     (call-with-values |#
#|      (lambda () (split-at (mashhap-entries map) (modulo (hash k) (length (mashhap-entries map))))) |#
#|      (lambda (a b) (append b a)))) |#
#|   (for/last ([el search-space] |#
#|              #:break ((length el) . < . 2) |#
#|              #:final (= (car el) k) |#
#|              [i (in-naturals 0)]) |#
#|     (i))) |#

#| (define (location-of map k) |#
#|   (define len (length (mashhap-entries map))) |#
#|   (define start (modulo (hash k) len)) |#
#|   (for/last ([i (in-range start (+ start len))] |#
#|         #:final (or ((length el) . < . 2) (= (car el) k))) |#
#|     (i))) |#

(define (loc-exists? map i)
  (= 2 (length (list-ref (mashhap-entries map) i))))

(define (location-of map k)
  (define len (length (mashhap-entries map)))
  (define start (modulo (hash k) len))
  (do ([i start (modulo (add1 i) len)])
    ((let ([el (list-ref (mashhap-entries map) i)])
       (or (null? el) (and (= (length el) 2) (string=? (car el) k))))
     i)))

; FNV-1a
(define (hash s)
  (for/fold ([hash 2166136261])
            ([c (string->list s)])
    (modulo (* 16777619 (bitwise-xor hash (char->integer c))) (expt 2 32))))
